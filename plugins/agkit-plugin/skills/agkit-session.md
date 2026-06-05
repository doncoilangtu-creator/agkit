# agkit-session — Nạp context đầu phiên làm việc (v3.0)
# Skill này được gọi khi bắt đầu một phiên làm việc mới.
# Trigger: "agkit session", "bắt đầu phiên", "load context", "nạp context dự án"

## Mô tả

Thực hiện toàn bộ Session Start Checklist từ `INSTRUCTIONS.md` một cách tự động:
đọc bộ nhớ dự án, trạng thái hiện tại, load đúng rules theo stack, và báo cáo
tóm tắt sẵn sàng làm việc.
**v3.0: Tự động ghi nhận session vào Durable Layer (SQLite).**

---

## Các bước thực hiện

### Bước 1 — Kiểm tra AGKit tồn tại

Tìm `INSTRUCTIONS.md` hoặc thư mục `.agkit/` trong project root:
- **Không tìm thấy**: Báo "Chưa có AGKit. Gọi `agkit init` để setup." → Dừng
- **Tìm thấy**: Tiếp tục

### Bước 1.5 — Ghi session vào Durable Layer (v3.0 MỚI)

Kiểm tra `.agkit/bin/agkit-cli.exe` (hoặc `agkit-cli` trên Linux/Mac):
- **Có**: Chạy `agkit-cli session start --summary "Session started"`
- **Không có**: Cảnh báo nhẹ: "💡 Durable Layer chưa cài. Chạy `agkit upgrade` để bổ sung."
  → Tiếp tục bình thường (không block)

### Bước 2 — Đọc PROJECT.md

Đọc `.agkit/PROJECT.md` và extract:
- Tên project + mục đích
- Tech stack đang dùng
- Key patterns đã chốt
- ADR log (các quyết định đã làm)
- Off-limits (những gì không được sửa)
- Known issues

### Bước 3 — Đọc STATUS.md

Đọc `.agkit/STATUS.md` và extract:
- Current Sprint: đang làm gì
- Blocked: có gì bị chặn không
- Next Up: task tiếp theo

### Bước 4 — Detect và load Rules

Scan project root:
- Tìm `package.json` → đọc `.agkit/rules/nextjs.md` (nếu có next dependency)
- Tìm `go.mod` → đọc `.agkit/rules/golang.md`
- Tìm `requirements.txt` / `pyproject.toml` → đọc `.agkit/rules/python.md`
- Luôn đọc `.agkit/rules/common.md`

### Bước 5 — Kiểm tra Context Window

Ước tính độ dài conversation hiện tại:
- **Phiên mới** (ít history): Bình thường
- **Phiên cũ, dài**: Thêm cảnh báo vào báo cáo

### Bước 6 — Báo cáo tóm tắt (ngắn gọn, không verbose)

In ra report theo format sau — **tối đa 15 dòng**:

```
🔄 AGKit Session — [Tên project]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📌 Đang làm: [Current Sprint từ STATUS.md]
⛔ Blocked:  [Nếu có, nếu không: "Không có"]
⏭ Tiếp theo: [Next Up từ STATUS.md]

🔧 Stack: [Danh sách stack đã detect]
📋 Rules: [Danh sách rules đã load]

⚠️ Lưu ý: [Known issues quan trọng nếu có, nếu không thì bỏ qua]

✅ Sẵn sàng. Bắt đầu nhé!
```

---

## Hành vi đặc biệt

### Khi PROJECT.md còn là template (chưa điền)
Phát hiện bằng cách tìm `_(Điền vào khi setup)_` trong file.
→ Cảnh báo: "PROJECT.md chưa được điền thông tin. Gọi `agkit init` để hoàn thiện."

### Khi STATUS.md trống
→ Báo: "Chưa có task nào được ghi. Hãy cho biết bạn muốn làm gì hôm nay?"
→ Sau khi user trả lời, tự động cập nhật `STATUS.md → Current Sprint`
