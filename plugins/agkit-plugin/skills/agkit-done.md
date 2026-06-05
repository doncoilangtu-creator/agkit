# agkit-done — Đánh dấu task hoàn thành và cập nhật memory
# Skill này được gọi khi hoàn thành một task để cập nhật STATUS.md.
# Trigger: "agkit done", "xong rồi", "task này done", "mark done [task]"

## Mô tả

Cập nhật `STATUS.md` sau khi hoàn thành task: chuyển Current Sprint sang Last Completed,
cập nhật Next Up, và dọn dẹp Session Notes. Đảm bảo bộ nhớ dự án luôn
phản ánh đúng trạng thái thực tế.

---

## Các bước thực hiện

### Bước 1 — Xác nhận task đã xong

Kiểm tra Verification Loop đã pass chưa:
- Nếu chưa chạy verify: Hỏi "Bạn đã chạy `agkit verify` chưa? Test đã pass chưa?"
  - User xác nhận đã pass → Tiếp tục
  - User chưa verify → "Recommend chạy `agkit verify` trước khi mark done"

### Bước 2 — Đọc STATUS.md hiện tại

Đọc `.agkit/STATUS.md` để lấy:
- Current Sprint (task vừa hoàn thành)
- Next Up list
- Session Notes hiện có

### Bước 3 — Tóm tắt task vừa xong

Nếu user không cung cấp tóm tắt, tự tạo dựa trên:
- Tên task từ Current Sprint
- Những gì đã thay đổi trong phiên làm việc

Format: `[Tên task] — [Kết quả ngắn: Done / Partial / Reverted]`

### Bước 4 — Cập nhật STATUS.md

Thực hiện các thay đổi sau trong `STATUS.md`:

1. **Chuyển Current Sprint → Last Completed:**
```
| [Tên task từ Current Sprint] | Done | [YYYY-MM-DD] |
```

2. **Xóa Current Sprint cũ, thêm task tiếp theo:**
```
**Đang làm:** [Next Up #1 từ list] hoặc "[Chờ user chỉ định]"
```

3. **Cập nhật Next Up:** Bỏ item #1 đã chuyển lên Current Sprint, shift các item còn lại

4. **Dọn Session Notes:** Xóa hoặc tóm tắt Session Notes vào Last Completed nếu có thông tin quan trọng

### Bước 5 — Kiểm tra có cần cập nhật PROJECT.md không

Nếu task vừa xong có:
- Thay đổi kiến trúc / pattern mới → Hỏi: "Có cần cập nhật Key Patterns trong PROJECT.md không?"
- Tạo external dependency mới → Hỏi: "Có cần thêm vào External Dependencies không?"
- Đóng một Known Issue → Hỏi: "Có Known Issue nào được fix trong task này không?"

### Bước 6 — Báo cáo

```
✅ Task marked as Done!

📋 STATUS.md đã cập nhật:
   ✓ [Tên task] → Last Completed
   ✓ Current Sprint → [Task tiếp theo]
   ✓ Session Notes → Đã dọn dẹp

💡 Tiếp theo: [Next Up #1]
   Khi sẵn sàng: "agkit session" để bắt đầu phiên mới
```
