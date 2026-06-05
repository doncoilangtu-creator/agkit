# agkit-init — Khởi tạo AGKit cho dự án mới
# Skill này được gọi khi user muốn setup AGKit cho một project.
# Trigger: "agkit init", "khởi tạo agkit", "setup kit cho project này"

## Mô tả

Tự động detect tech stack, hỏi 3 câu về project, và generate toàn bộ cấu trúc `.agkit/`
cùng với `INSTRUCTIONS.md` được điền sẵn context thực tế.

---

## Các bước thực hiện

### Bước 1 — Kiểm tra xem AGKit đã tồn tại chưa

Kiểm tra thư mục `.agkit/` trong project root hiện tại:
- Nếu **đã tồn tại**: Hỏi user "AGKit đã được setup. Bạn muốn (a) reset toàn bộ, hay (b) chỉ cập nhật PROJECT.md?"
- Nếu **chưa tồn tại**: Tiếp tục bước 2

### Bước 2 — Detect Tech Stack

Scan các file sau trong project root và subdirectories:

| File tìm thấy | Stack detected |
|---|---|
| `package.json` (có `"next"` trong dependencies) | Next.js + TypeScript |
| `package.json` (không có next) | Node.js / React |
| `go.mod` | Go |
| `requirements.txt` hoặc `pyproject.toml` | Python |
| Nhiều file cùng lúc | Multi-stack (liệt kê tất cả) |
| Không tìm thấy gì | Báo "Không detect được stack, sẽ dùng template generic" |

### Bước 3 — Hỏi 3 câu nhanh (dùng ask_question tool)

Hỏi tuần tự, không hỏi cùng lúc:

**Câu 1:** "Tên project và mục đích chính là gì? (1-2 câu ngắn)"

**Câu 2:** "Database/infrastructure chính bạn đang dùng hoặc dự định dùng?"
- Options: PostgreSQL / MySQL / MongoDB / Supabase / PlanetScale / SQLite / Không có / Khác

**Câu 3:** "Project đang ở giai đoạn nào?"
- Options: Greenfield (mới hoàn toàn) / Đã có code rồi (thêm kit vào) / MVP sắp launch

### Bước 4 — Generate file cấu trúc

Dựa vào thông tin thu thập được, tạo hoặc copy từ `.agkit/` template:

**Nếu `.agkit/` chưa tồn tại trong project:**
Tạo toàn bộ cấu trúc mới:
```
.agkit/
├── PROJECT.md    ← Điền thông tin từ bước 3
├── STATUS.md     ← Template sẵn, thêm "Current Sprint: Setup mới"
├── VERIFY.md     ← Copy từ agkit template
├── rules/
│   ├── common.md
│   ├── [stack].md   ← Chỉ copy rules của stack detect được
└── agents/
    ├── architect.md
    ├── code-reviewer.md
    ├── build-resolver.md
    └── security-scanner.md
```

**Tạo `INSTRUCTIONS.md` ở project root** với nội dung đầy đủ theo đúng spec.

### Bước 5 — Điền PROJECT.md với thông tin thực tế

Thay thế các placeholder trong `PROJECT.md` bằng thông tin đã thu thập:
- `Project Overview`: Tên + mục đích từ câu 1
- `Tech Stack`: Stack detect được + database từ câu 2
- `Architecture Overview`: Sketch đơn giản phù hợp với stack

### Bước 6 — Báo cáo kết quả

In ra danh sách files đã tạo và hướng dẫn tiếp theo:

```
✅ AGKit đã được khởi tạo cho [Tên project]!

📁 Đã tạo:
  INSTRUCTIONS.md
  .agkit/PROJECT.md     ← Đã điền thông tin dự án
  .agkit/STATUS.md
  .agkit/VERIFY.md
  .agkit/rules/common.md
  .agkit/rules/[stack].md
  .agkit/agents/ (4 files)

📋 Bước tiếp theo:
  1. Review .agkit/PROJECT.md và bổ sung Key Patterns nếu cần
  2. Mỗi phiên làm việc: nói "agkit session" để nạp context
  3. Trước commit: nói "agkit security" để scan bảo mật
```
