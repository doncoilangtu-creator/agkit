# agkit-history — Xem lịch sử hoạt động từ Durable Layer
# Skill này được gọi để xem lịch sử sessions, traces, và các hoạt động đã ghi nhận.
# Trigger: "agkit history", "xem lịch sử", "history", "agkit history [filter]"

## Mô tả

Truy vấn cơ sở dữ liệu SQLite (Durable Layer) để hiển thị lịch sử các hoạt động
trong dự án: sessions, traces, intakes, ADRs theo dòng thời gian.

---

## Các bước thực hiện

### Bước 1 — Kiểm tra Durable Layer

Kiểm tra file `.agkit/bin/agkit-cli.exe` tồn tại:
- **Không có**: Báo "Durable Layer chưa cài đặt. Chạy `agkit upgrade` để bổ sung." → Dừng
- **Có**: Tiếp tục

### Bước 2 — Xác định loại lịch sử cần xem

Phân tích request của user:
- "agkit history" → Xem tất cả (mặc định 10 records gần nhất)
- "agkit history sessions" → Chỉ xem sessions
- "agkit history traces" → Chỉ xem traces
- "agkit history traces failure" → Chỉ xem traces thất bại
- "agkit history 20" → Xem 20 records gần nhất

### Bước 3 — Chạy lệnh CLI tương ứng

```bash
# Xem tất cả
agkit-cli history --last 10

# Chỉ sessions
agkit-cli history --last 10 --filter sessions

# Traces thất bại
agkit-cli history --last 10 --filter traces --outcome failure
```

### Bước 4 — Format và hiển thị kết quả

Hiển thị output từ CLI theo format đẹp mắt:

```
📜 Lịch sử hoạt động (10 gần nhất)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🕐 2024-01-15 09:30  📂 SESSION  Bắt đầu phiên: Implement auth flow
🕐 2024-01-15 09:35  🔄 INTAKE   [🟡normal] Add login page
🕐 2024-01-15 10:00  📝 TRACE    Fixed CORS issue [✅success]
🕐 2024-01-15 10:30  🏗  ADR      ADR-003: Dùng JWT thay session cookies
🕐 2024-01-15 11:00  📂 SESSION  Kết thúc phiên
```

---

## Hành vi đặc biệt

### Khi database trống
→ Báo: "Chưa có hoạt động nào được ghi nhận. Bắt đầu bằng `agkit session` để tạo phiên đầu tiên."

### Khi user hỏi về project khác
→ Gợi ý: "Để xem lịch sử project khác, mở thư mục project đó trước."
