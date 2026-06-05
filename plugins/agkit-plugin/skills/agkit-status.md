# agkit-status — Hiển thị trạng thái và sức khỏe dự án
# Skill này được gọi để xem nhanh tình hình project.
# Trigger: "agkit status", "tình hình project", "đang ở đâu rồi", "project health"

## Mô tả

Đọc `PROJECT.md` và `STATUS.md`, tổng hợp thành một dashboard ngắn gọn cho thấy
project đang ở đâu, có gì blocked, và những gì cần làm tiếp theo.
Khác với `agkit-session` (chỉ load context), skill này còn tính "sức khỏe" dự án.

---

## Các bước thực hiện

### Bước 1 — Đọc dữ liệu

Đọc tuần tự:
1. `.agkit/PROJECT.md` — overview, known issues, off-limits
2. `.agkit/STATUS.md` — current sprint, blocked, next up

### Bước 2 — Tính Project Health Score

Đánh giá sức khỏe dự án dựa trên:

| Chỉ số | Tốt | Cần chú ý | Xấu |
|---|---|---|---|
| Known Issues | 0 High | 1-2 | 3+ |
| Blocked items | 0 | 1 | 2+ |
| ADR Log | Có entries | Ít entries | Trống |
| Off-limits | Được định nghĩa | Mơ hồ | Trống |
| Key Patterns | Đầy đủ | Một phần | Chưa điền |

**Health Score:**
- 🟢 **Healthy** — Ít blocked, ít known issues, docs đầy đủ
- 🟡 **Needs Attention** — Có blocked hoặc nhiều known issues
- 🔴 **At Risk** — Nhiều blocker hoặc project docs thiếu

### Bước 3 — Hiển thị Dashboard

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 AGKit Status — [Tên Project]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

 Health: 🟢 Healthy  (hoặc 🟡/🔴)

 📌 Đang làm:
    [Current Sprint]

 ⛔ Blocked: [N items]
    [Liệt kê nếu có, "Không có" nếu trống]

 ⏭ Tiếp theo:
    1. [Next Up 1]
    2. [Next Up 2]

 🐛 Known Issues: [N items]
    [High priority issues nếu có]

 🏗 Tech Stack: [Stack]
 📅 Cập nhật lần cuối: [Date từ STATUS.md]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Lệnh gợi ý:
  agkit verify    — Chạy tests
  agkit review    — Review code mới nhất
  agkit security  — Scan trước commit
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Bước 4 — Cảnh báo thông minh

Tự động thêm cảnh báo nếu phát hiện:

**PROJECT.md chưa được điền đầy đủ:**
```
⚠️  PROJECT.md có [N] section chưa điền. Gọi `agkit init` để hoàn thiện.
```

**Quá nhiều Session Notes tích lũy:**
```
💡 Session Notes khá dài. Hãy dọn dẹp vào Last Completed sau khi xong task.
```

**Off-limits trống:**
```
💡 Chưa định nghĩa Off-limits. Nên ghi những phần code stable để bảo vệ.
```
