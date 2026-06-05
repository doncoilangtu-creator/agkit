# VERIFY.md — Verification Loop Protocol
# Antigravity phải chạy protocol này SAU MỖI LẦN viết hoặc sửa code.

---

## 🔄 Vòng lặp Kiểm tra

```
[Viết / Sửa code]
        ↓
[Chạy test suite phù hợp]
        ↓
   ┌── PASS? ──────────────────────────────────────────────────────┐
   │                                                               ↓
   │                                               [Code Reviewer Check]
   │                                                               ↓
   │                                               [Cập nhật STATUS.md]
   │                                                               ↓
   │                                                [Báo "Done" cho user]
   │
   └── FAIL? ──────────────────────────────────────────────────────┐
                                                                   ↓
                                                     [Đọc error message]
                                                                   ↓
                                              [Phân tích ROOT CAUSE — không chỉ symptom]
                                                                   ↓
                                                        [Sửa theo root cause]
                                                                   ↓
                                                     [Chạy lại test suite]
                                                                   ↓
                                              [Nếu vẫn FAIL sau 3 vòng → Escalate]
```

---

## 📋 Lệnh Test Theo Stack

### Next.js / TypeScript
```bash
# Unit tests
npm run test                    # hoặc: pnpm test / yarn test / bun test

# Type checking (chạy riêng, không thay thế test)
npx tsc --noEmit

# Lint
npm run lint

# Build check (chạy khi sắp deploy)
npm run build
```

### Go
```bash
# Chạy tất cả test trong project
go test ./...

# Với coverage
go test ./... -cover

# Chạy test cụ thể
go test ./path/to/package -run TestFunctionName -v

# Vet (tìm bug tiềm ẩn)
go vet ./...
```

### Python
```bash
# Pytest
python -m pytest

# Với coverage
python -m pytest --cov=. --cov-report=term-missing

# Type check
mypy . --strict

# Lint
ruff check .
```

---

## ⚠️ Quy tắc Bắt buộc

1. **KHÔNG báo "done" khi test chưa pass** — dù lỗi có vẻ nhỏ hay không liên quan
2. **KHÔNG sửa test để test pass** — trừ trường hợp test đang sai logic nghiệp vụ
3. **KHÔNG bỏ qua TypeScript error** bằng `@ts-ignore` hoặc `as any`
4. **Không skip test** bằng `.skip()` hoặc `-k` trừ khi có comment giải thích rõ lý do
5. **Viết test trước khi sửa bug** (TDD) — xác nhận bug tồn tại bằng failing test, sau đó sửa

---

## 🆘 Escalation (Khi vẫn fail sau 3 vòng)

Khi đã sửa 3 lần mà vẫn fail:

1. **Dừng lại — không tiếp tục sửa bừa**
2. Ghi vào `STATUS.md → Blocked`: mô tả error, những gì đã thử
3. **Báo user:** "Tôi đã thử 3 lần nhưng vẫn fail. Đây là error và những gì tôi đã thử: [...]"
4. Hỏi user muốn: (a) tiếp cận khác, (b) bỏ qua tạm, hay (c) escalate lên architect agent

---

## 🔍 Code Reviewer Quick Check

Trước khi báo "done", chạy mental checklist nhanh:

- [ ] Code có follow patterns trong `PROJECT.md → Key Patterns` không?
- [ ] Có edge case nào chưa xử lý không? (null, empty array, network error...)
- [ ] Có hardcoded value nào cần chuyển thành config/env không?
- [ ] Naming có rõ ràng không? (tên function nói lên nó làm gì)
- [ ] Có side effect nào không mong muốn không?

Nếu có nghi ngờ → invoke `code-reviewer` agent.
