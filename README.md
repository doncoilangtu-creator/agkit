# AGKit v3.0 — Antigravity Developer Kit

**AGKit v3.0** là bộ công cụ phát triển toàn diện tối ưu cho **Antigravity AI Assistant**. Bộ Kit giúp biến mọi repository thành một workspace thông minh cho AI Coding Agent — với khả năng tự động phân loại rủi ro, theo dõi lịch sử hoạt động, và đối chiếu kiểm chứng hành vi.

> *Coding agents không chỉ cần những prompt tốt hơn. Chúng cần những repository được thiết kế tốt hơn.*

---

## ✨ Tính năng mới trong v3.0

### 🗄 Durable State Layer
- Cơ sở dữ liệu SQLite (`agkit.db`) lưu trữ có cấu trúc: sessions, traces, ADRs, stories, test matrix, backlog
- CLI binary (`agkit-cli`) bằng Rust để đọc/ghi dữ liệu
- Hai database: per-project (`.agkit/agkit.db`) + global (`~/.gemini/agkit-global.db`)

### 🚦 Risk Lanes (Phân luồng rủi ro)
- Mọi request được phân loại tự động: 🟢 Tiny / 🟡 Normal / 🔴 High-risk
- Mỗi lane có quy trình bắt buộc riêng để đảm bảo an toàn

### 🧪 Test Matrix (Ma trận kiểm chứng)
- Đối chiếu hành vi yêu cầu với bằng chứng kiểm chứng (Unit, Integration, E2E, Platform)
- Chấm điểm coverage score tự động

---

## 📂 Cấu trúc dự án

```
.agkit/
├── PROJECT.md          ← Bộ nhớ bền vững của dự án
├── STATUS.md           ← Trạng thái công việc hiện tại
├── VERIFY.md           ← Verification Loop Protocol
├── agkit.db            ← SQLite database (gitignored)
├── bin/agkit-cli.exe   ← Rust CLI binary (gitignored)
├── rules/              ← 8 bộ quy tắc (common, nextjs, golang, python, supabase, tailwind, docker, testing)
└── agents/             ← 8 agent chuyên biệt (architect, code-reviewer, security-scanner, ...)

.harness/               ← Self-Healing Test Harness
plugins/agkit-plugin/   ← 23 skills tự động hóa
guide/                  ← HTML Guide tương tác
agkit-cli/              ← Rust source code
```

---

## 🚀 23 Skills Hỗ trợ

### Core (Cốt lõi)
| Lệnh | Mô tả |
|---|---|
| `/init` | Khởi tạo AGKit cho dự án mới |
| `/session` | Bắt đầu phiên làm việc, nạp context |
| `/status` | Kiểm tra trạng thái dự án |
| `/done` | Kết thúc task và cập nhật trạng thái |
| `/help` | Xem danh sách tất cả lệnh |
| `/upgrade` | Nâng cấp AGKit lên phiên bản mới |

### Planning & Risk (Lập kế hoạch & Rủi ro)
| Lệnh | Mô tả |
|---|---|
| `/plan` | Lên kế hoạch với Intake Classification (v3.0) |
| `/intake` | 🆕 Phân loại rủi ro công việc độc lập |
| `/adr` | Ghi Architecture Decision Record |

### Quality & Security (Chất lượng & Bảo mật)
| Lệnh | Mô tả |
|---|---|
| `/verify` | Chạy test + Test Matrix Report (v3.0) |
| `/review` | Code review tự động |
| `/security` | Quét lỗi bảo mật |
| `/matrix` | 🆕 Xem/quản lý Test Matrix |

### Development (Phát triển)
| Lệnh | Mô tả |
|---|---|
| `/refactor` | Đề xuất cải tiến cấu trúc code |
| `/debug` | Phân tích và sửa lỗi |
| `/perf` | Kiểm tra hiệu suất |
| `/docs` | Tạo tài liệu tự động |
| `/deploy` | Kiểm tra các bước triển khai |
| `/git` | Quản lý Git workflow |

### Durable Layer (Lớp dữ liệu bền vững) — 🆕 v3.0
| Lệnh | Mô tả |
|---|---|
| `/history` | 🆕 Xem lịch sử hoạt động từ DB |
| `/trace` | 🆕 Ghi hành động quan trọng vào DB |
| `/stats` | 🆕 Thống kê nhanh sức khỏe dự án |
| `/backlog` | 🆕 Quản lý danh sách việc cần làm |

---

## 🛠️ Hướng dẫn cài đặt

### 1. Clone repository
```bash
git clone https://github.com/doncoilangtu-creator/agkit.git
```

### 2. Copy cấu hình vào dự án
```bash
cp -r .agkit /path/to/your/project/
cp INSTRUCTIONS.md /path/to/your/project/
```

### 3. Cài plugin cho Antigravity
```powershell
# Windows
xcopy /E /I plugins\agkit-plugin "$env:USERPROFILE\.gemini\config\plugins\agkit-plugin"
```

### 4. Build CLI (cần Rust toolchain + MSVC)
```bash
cd agkit-cli
cargo build --release
mkdir -p ../.agkit/bin
cp target/release/agkit-cli.exe ../.agkit/bin/
```

### 5. Khởi tạo Durable Layer
```bash
.agkit/bin/agkit-cli init
```

---

*Phát triển bởi Hoanghitech & Antigravity Assistant. Lấy cảm hứng từ [Harness Engineering](https://openai.com/index/harness-engineering/).*
