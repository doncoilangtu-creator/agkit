# agkit-upgrade — Nâng cấp AGKit lên phiên bản mới nhất (v3.0)
# Trigger: "agkit upgrade", "nâng cấp agkit", "lên v3", "update kit"

## Mô tả

Kiểm tra phiên bản AGKit hiện tại của project và tự động nâng cấp lên v3.0.
Bao gồm: copy agents/rules mới, cài đặt Durable Layer (Rust CLI + SQLite),
và cập nhật PROJECT.md.

---

## Các bước thực hiện

### Bước 1 — Detect phiên bản hiện tại

Đọc `.agkit/PROJECT.md` và tìm dòng `AGKit Version`:
- Tìm thấy `v3.0` → Báo: "✅ Project này đã là AGKit v3.0. Không cần upgrade."
- Tìm thấy `v2.0` → Tiếp tục upgrade v2→v3
- Tìm thấy `v1.0` hoặc không có dòng version → Cần upgrade v1→v3 (bao gồm tất cả)
- Không tìm thấy `.agkit/` → Báo: "Chưa có AGKit. Gọi `agkit init` để setup từ đầu."

### Bước 2 — Kiểm tra những gì cần bổ sung

**Nếu từ v2.0 → v3.0:**

Kiểm tra các thành phần v3.0:

| Thành phần | Kiểm tra |
|---|---|
| Durable Layer CLI | `.agkit/bin/agkit-cli.exe` tồn tại? |
| Local Database | `.agkit/agkit.db` tồn tại? |
| Global Database | `~/.gemini/agkit-global.db` tồn tại? |
| 6 Skills mới | Kiểm tra trong plugins/agkit-plugin/skills/ |
| 3 Skills nâng cấp | Kiểm tra version header (v3.0) |

**Nếu từ v1.0 → v3.0:**

Bao gồm tất cả upgrade v2.0 (agents, rules) + v3.0 (Durable Layer).

### Bước 3 — Xác nhận với user

```
📋 AGKit Upgrade: v2.0 → v3.0

Project: [tên project]
Hiện có: [N agents, N rules, N skills]

Sẽ thêm/cập nhật:
  🆕 Durable Layer:
     • agkit-cli.exe (Rust CLI binary)
     • .agkit/agkit.db (SQLite database)
     • ~/.gemini/agkit-global.db
  
  🆕 6 Skills mới:
     • /history, /matrix, /intake, /trace, /stats, /backlog
  
  ⬆️ 3 Skills nâng cấp:
     • /plan (+ Intake Classification)
     • /verify (+ Test Matrix Report)
     • /session (+ DB recording)

Tiếp tục? (yes/no)
```

### Bước 4 — Cài đặt Durable Layer

**4.1 — Kiểm tra Rust toolchain:**
```bash
rustc --version
```
- Nếu chưa có → Hướng dẫn cài: `winget install Rustlang.Rustup`
- Nếu đã có → Tiếp tục

**4.2 — Build CLI binary:**
```bash
cd agkit-cli
cargo build --release
mkdir -p ../.agkit/bin
cp target/release/agkit-cli.exe ../.agkit/bin/
```

**4.3 — Khởi tạo databases:**
```bash
.agkit/bin/agkit-cli init
```

### Bước 5 — Copy Skills mới

Copy 6 skill files mới vào `plugins/agkit-plugin/skills/`:
- agkit-history.md
- agkit-matrix.md
- agkit-intake.md
- agkit-trace.md
- agkit-stats.md
- agkit-backlog.md

Cập nhật 3 skill files:
- agkit-plan.md (thêm Bước 0 — Intake Classification)
- agkit-verify.md (thêm Bước 6 — Test Matrix + Bước 7 — Trace)
- agkit-session.md (thêm Bước 1.5 — DB recording)

### Bước 6 — Cập nhật cấu hình

1. Cập nhật `plugin.json` → version 3.0.0
2. Cập nhật `.gitignore` → thêm `agkit.db`, `.agkit/bin/`, `agkit-cli/target/`
3. Cập nhật `agkit-help.md` → 23 skills
4. Copy plugin vào `~/.gemini/config/plugins/agkit-plugin/`

### Bước 7 — Cập nhật PROJECT.md

Thêm/cập nhật:
```markdown
**AGKit Version:** 3.0
**Upgraded:** [YYYY-MM-DD]
**Durable Layer:** ✅ Installed (agkit-cli v3.0.0)
```

### Bước 8 — Báo cáo

```
✅ AGKit Upgrade v3.0 hoàn thành!

📁 Durable Layer:
   .agkit/bin/agkit-cli.exe  ← CLI binary (1.7MB)
   .agkit/agkit.db           ← Local SQLite database
   ~/.gemini/agkit-global.db ← Global database

🆕 6 Skills mới:
   /history   — Xem lịch sử hoạt động
   /matrix    — Test Matrix (Behavior-to-Proof)
   /intake    — Phân loại rủi ro công việc
   /trace     — Ghi hành động vào DB
   /stats     — Thống kê sức khỏe dự án
   /backlog   — Quản lý danh sách việc

⬆️ 3 Skills nâng cấp:
   /plan      — + Intake Classification (Risk Lanes)
   /verify    — + Test Matrix Report + Trace
   /session   — + Durable Layer recording

🚦 Risk Lanes sẵn sàng:
   🟢 Tiny    → Sửa trực tiếp → /verify
   🟡 Normal  → /plan → Code → /verify → /review
   🔴 High-risk → /plan + Mermaid → User duyệt → /security → /verify → /review

Project đã ở AGKit v3.0 🎉
Gõ "agkit stats" để xem dashboard sức khỏe dự án.
```

---

## Hành vi đặc biệt

### Khi không có Rust toolchain
Nếu máy chưa có Rust, hướng dẫn cài:
1. Windows: `winget install Rustlang.Rustup`
2. Cài MSVC Build Tools: `winget install Microsoft.VisualStudio.2022.BuildTools`
3. Cài Windows SDK: `winget install Microsoft.WindowsSDK.10.0.22621`
4. Sau đó thử lại `agkit upgrade`

### Khi agkit-cli đã tồn tại
Kiểm tra version: `.agkit/bin/agkit-cli --version`
Nếu đã là 3.0.0 → bỏ qua bước build
