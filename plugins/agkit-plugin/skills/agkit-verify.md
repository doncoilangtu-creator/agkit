# agkit-verify — Verification Loop thủ công (v3.0)
# Skill này được gọi để chạy test suite và thực hiện verification loop.
# Trigger: "agkit verify", "chạy test", "kiểm tra code có pass không", "agkit verify [stack]"

## Mô tả

Chạy test suite phù hợp với stack đang dùng, theo đúng protocol trong `VERIFY.md`.
Nếu test fail, tự đọc lỗi, tìm root cause và sửa. Lặp tối đa 3 vòng trước khi escalate.
**v3.0: Tự động xuất Test Matrix Report và ghi trace vào Durable Layer sau mỗi lần verify.**

---

## Các bước thực hiện

### Bước 1 — Detect stack và lệnh test

Scan project root để chọn lệnh test phù hợp:

**Next.js / TypeScript:**
```bash
# Chạy theo thứ tự: type check → lint → unit test
npx tsc --noEmit 2>&1
npm run lint 2>&1
npm run test 2>&1
```

**Go:**
```bash
go vet ./... 2>&1
go test ./... 2>&1
```

**Python:**
```bash
mypy . --strict 2>&1 | head -30
ruff check . 2>&1 | head -20
python -m pytest -x --tb=short 2>&1
```

**Multi-stack:** Chạy tất cả theo thứ tự.

### Bước 2 — Chạy test lần 1

Thực thi lệnh test, capture toàn bộ output.

**Nếu PASS tất cả:**
→ Báo: "✅ Tất cả tests pass. Code sẵn sàng." → Kết thúc

**Nếu FAIL:**
→ Tiếp tục bước 3

### Bước 3 — Phân tích lỗi (Vòng lặp, tối đa 3 lần)

```
[Vòng lặp #N — tối đa 3]

1. Đọc toàn bộ error output — không chỉ dòng đầu
2. Phân loại lỗi:
   - Type error → tìm interface thay đổi, missing property
   - Test assertion fail → logic bug, check expected vs actual
   - Build error → import/compile issue
   - Lint error → style/rule violation (thường fix tự động)

3. Xác định ROOT CAUSE (không chỉ symptom)
4. Sửa code
5. Chạy lại test
6. Nếu PASS → Dừng và báo cáo
   Nếu FAIL → Vòng lặp tiếp theo (nếu còn)
```

### Bước 4 — Escalation (sau 3 vòng vẫn fail)

```
⚠️ Verification Loop — Cần hỗ trợ

Đã thử 3 lần nhưng vẫn fail:

❌ Error:
[Error message]

🔍 Đã thử:
- Lần 1: [Mô tả fix attempt 1]
- Lần 2: [Mô tả fix attempt 2]
- Lần 3: [Mô tả fix attempt 3]

📋 Bạn muốn:
  (a) Thử cách tiếp cận khác (mô tả hướng bạn muốn thử)
  (b) Skip test này tạm thời và ghi vào Known Issues
  (c) Invoke Build Resolver agent để phân tích sâu hơn
```

Ghi vào `STATUS.md → Blocked`:
```
Test fail sau 3 vòng: [mô tả lỗi] — [ngày giờ]
```

### Bước 5 — Báo cáo kết quả cuối

**PASS:**
```
✅ Verification Loop: PASS
   TypeScript: ✓  |  Lint: ✓  |  Tests: ✓ (N passed)
   Thời gian: [X]s
```

**FAIL → Escalated:**
```
❌ Verification Loop: ESCALATED
   Đã ghi vào STATUS.md → Blocked
```

### Bước 6 — Test Matrix Report (v3.0 MỚI)

Sau khi chạy test suite, tự động:

1. Đọc story hiện tại từ `STATUS.md` (nếu có story ID dạng US-XXX)
2. Nếu có story ID, query matrix:
   ```bash
   agkit-cli matrix query --story <story_id>
   ```
3. Xuất báo cáo:
   ```
   📋 Test Matrix — US-001 "User Registration"
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

   | Hành vi                  | Unit | Integration | E2E  | Platform |
   |--------------------------|------|-------------|------|----------|
   | User đăng ký OK          |  ✅  |     ✅      |  ❌  |    —    |
   | Email trùng → reject     |  ✅  |     ❌      |  ❌  |    —    |

   Score: 3/6 (50%) — Cần thêm Integration + E2E
   ```
4. Nếu test vừa chạy pass, tự động cập nhật matrix:
   ```bash
   agkit-cli matrix set --story <story_id> --behavior "<behavior>" --unit 1
   ```

### Bước 7 — Ghi Trace vào Durable Layer (v3.0 MỚI)

Ghi kết quả verify vào Durable Layer:

**Nếu PASS:**
```bash
agkit-cli trace --summary "Verify: all tests passed (N tests)" --outcome success
```

**Nếu FAIL → Escalated:**
```bash
agkit-cli trace --summary "Verify: tests failed after 3 attempts" --outcome failure
```

