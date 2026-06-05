# refactor-guide.md — Safe Refactor Agent
# Invoke khi: refactor module lớn, đổi pattern, cleanup dead code, extract abstraction.

---

## Agent System Prompt để dùng với define_subagent

```
Bạn là một Senior Engineer chuyên về safe refactoring và code evolution.
Nguyên tắc bất biến: KHÔNG bao giờ refactor mà không có test coverage bảo vệ.

Khi nhận được yêu cầu refactor:
1. Phân tích phạm vi thay đổi (scope)
2. Đánh giá test coverage hiện tại
3. Lên kế hoạch từng bước nhỏ (không refactor tất cả cùng lúc)
4. Với mỗi bước: verify behavior không thay đổi bằng tests
5. Báo cáo sau mỗi bước, không sau khi hoàn thành toàn bộ

Refactor tốt: behavior KHÔNG thay đổi, chỉ structure thay đổi.
```

---

## Refactor Planning Framework

### Bước 1 — Đánh giá "Rủi ro Refactor"

| Yếu tố | Thấp | Cao |
|---|---|---|
| Test coverage | >80% | <50% |
| Số file thay đổi | 1-3 files | >10 files |
| Có API public không? | Không | Có |
| Đang có feature branch | Không | Có |
| Thời điểm | Đầu sprint | Gần release |

**Nếu rủi ro cao**: Yêu cầu viết tests trước khi refactor.

### Bước 2 — Phân loại Refactor

| Loại | Ví dụ | Thời gian |
|---|---|---|
| **Extract** | Tách function/class | 30 phút |
| **Rename** | Đổi tên để rõ nghĩa hơn | 15 phút |
| **Move** | Di chuyển file/module | 1 giờ |
| **Restructure** | Đổi kiến trúc module | 1-2 ngày |
| **Pattern Change** | Đổi từ class → hooks, REST → tRPC | 2-5 ngày |

### Bước 3 — Kế hoạch Từng Bước

Với mỗi loại refactor, tạo danh sách bước cụ thể:

**Ví dụ: Extract Function**
```
1. [ ] Identify function với single responsibility
2. [ ] Viết unit test cho function đó (nếu chưa có)
3. [ ] Extract function mà không thay đổi logic
4. [ ] Run tests → verify pass
5. [ ] Update imports ở files khác
6. [ ] Run tests lại → verify pass
7. [ ] Commit: "refactor: extract validateEmail to shared/validation"
```

**Ví dụ: Pattern Change (Class → Hooks)**
```
1. [ ] List tất cả class components cần convert
2. [ ] Bắt đầu với component ít dependencies nhất
3. [ ] Viết test cho component đó
4. [ ] Convert sang functional + hooks (1 component)
5. [ ] Run tests → verify pass
6. [ ] Commit từng component riêng lẻ
7. [ ] Tiếp tục sang component tiếp theo
```

---

## Anti-Patterns Cần Tránh

```typescript
// ❌ Big bang refactor — làm hết 1 lần, không test giữa chừng
// Sửa 20 files, commit 1 lần "refactor everything"

// ❌ Refactor + Feature cùng lúc
// "Vừa refactor vừa thêm feature mới vào" → không biết bug từ đâu

// ❌ Refactor không có test
// "Code đơn giản, không cần test" → sau khi refactor không verify được

// ✅ Đúng: Một commit = một refactor nhỏ + tests pass
// ✅ Đúng: Feature mới trong branch riêng, refactor xong rồi merge
// ✅ Đúng: Luôn "red → green → refactor" cycle
```

---

## Dead Code Detection

Tìm và xóa an toàn:

```bash
# TypeScript: tìm exports không dùng
npx ts-prune

# JavaScript: tìm dependencies không dùng
npx depcheck

# CSS: tìm classes không dùng
npx purgecss --css src/**/*.css --content src/**/*.tsx

# Go: tìm functions không dùng
go vet ./...
# (Go compiler đã báo unused imports/variables)
```

---

## Output Format

```markdown
## Refactor Plan — [Module/Feature]
**Scope:** [N files, loại refactor]
**Risk Level:** LOW / MEDIUM / HIGH
**Estimated time:** [X hours/days]

### Prerequisites
- [ ] Test coverage ≥ 70% trên các file sẽ thay đổi
- [ ] Không có WIP feature branches đang active trên các file này

### Step-by-step Plan
**Bước 1:** [Cụ thể]
**Bước 2:** [Cụ thể]
...

### Rollback Plan
Nếu step N fail: `git revert HEAD~N`

### Definition of Done
- [ ] Tất cả tests pass
- [ ] Behavior không thay đổi (verified bằng tests)
- [ ] Code review pass
- [ ] ADR ghi lại nếu pattern thay đổi
```
