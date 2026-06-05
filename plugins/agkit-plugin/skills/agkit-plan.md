# agkit-plan — Lên kế hoạch và breakdown task lớn
# Trigger: "agkit plan", "lên kế hoạch", "breakdown task", "agkit plan [mô tả feature]"

## Mô tả

Nhận mô tả feature/task lớn, phân tích và tạo kế hoạch chi tiết với subtasks có priority,
ước tính thời gian, xác định dependencies, và cập nhật STATUS.md ngay.

---

## Các bước thực hiện

### Bước 1 — Thu thập thông tin

Nếu user chỉ gõ "agkit plan" (không có mô tả), hỏi:
> "Mô tả feature/task bạn cần lên kế hoạch? (ví dụ: 'Làm tính năng authentication với email + Google OAuth')"

Nếu đã có mô tả, tiếp tục ngay.

### Bước 2 — Phân tích và Phân loại

Tự động nhận diện loại task:
- **Feature mới**: Cần thiết kế trước, estimate dài hơn
- **Bug fix**: Ưu tiên cao, cần reproduce trước
- **Refactor**: Cần test coverage trước, xem `agkit-refactor.md`
- **Performance**: Cần benchmark trước và sau

### Bước 3 — Breakdown thành subtasks

Tạo danh sách subtasks theo thứ tự logic:

**Format chuẩn:**
```
## Plan: [Tên Feature]
**Estimate tổng:** [X giờ / X ngày]
**Priority:** HIGH / MEDIUM / LOW

### Phase 1 — Foundation [X giờ]
- [ ] [Subtask cụ thể] — [estimate]
- [ ] [Subtask cụ thể] — [estimate]

### Phase 2 — Core Logic [X giờ]
- [ ] [Subtask cụ thể] — [estimate]

### Phase 3 — Testing & Polish [X giờ]
- [ ] Viết unit tests cho [X]
- [ ] Viết integration tests cho [X]
- [ ] E2E test critical path
- [ ] Code review (agkit review)
- [ ] Security scan (agkit security)

### Dependencies
- Cần xong trước: [ghi nếu có]
- Blocked bởi: [ghi nếu có]

### Definition of Done
- [ ] Tất cả tests pass (agkit verify)
- [ ] Code review pass (agkit review)
- [ ] Security scan PASS (agkit security)
- [ ] [Business criteria cụ thể]
```

### Bước 4 — Risk Assessment

Tự động đánh giá rủi ro:
- **Database schema thay đổi?** → Cảnh báo: cần migration plan
- **Authentication/Payment?** → Cảnh báo: cần security review bắt buộc
- **Public API thay đổi?** → Cảnh báo: cần versioning strategy
- **Estimate > 3 ngày?** → Gợi ý tách nhỏ hơn

### Bước 5 — Cập nhật STATUS.md

Sau khi user confirm plan, tự động cập nhật:
```
**Đang làm:** [Feature name] — Phase 1
**Next Up:**
1. [Phase 2 tasks]
2. [Phase 3 tasks]
```

Hỏi: "Bạn muốn bắt đầu Phase 1 ngay không?"
