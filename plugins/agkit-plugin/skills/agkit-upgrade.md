# agkit-upgrade — Nâng cấp AGKit lên phiên bản mới nhất
# Trigger: "agkit upgrade", "nâng cấp agkit", "lên v2", "update kit"

## Mô tả

Kiểm tra phiên bản AGKit hiện tại của project và tự động copy các files mới
từ một project đã có v2.0 (hoặc từ template), cập nhật PROJECT.md với version mới.

---

## Các bước thực hiện

### Bước 1 — Detect phiên bản hiện tại

Đọc `.agkit/PROJECT.md` và tìm dòng `AGKit Version`:
- Tìm thấy `v2.0` → Báo: "✅ Project này đã là AGKit v2.0. Không cần upgrade."
- Tìm thấy `v1.0` hoặc không có dòng version → Tiếp tục upgrade
- Không tìm thấy `.agkit/` → Báo: "Chưa có AGKit. Gọi `agkit init` để setup từ đầu."

### Bước 2 — Kiểm tra những gì còn thiếu

So sánh files trong `.agkit/` của project với v2.0 spec:

**Agents cần có (v2.0):**
- architect.md ✓ (v1)
- code-reviewer.md ✓ (v1)
- build-resolver.md ✓ (v1)
- security-scanner.md ✓ (v1)
- database-reviewer.md ← MỚI v2.0
- frontend-reviewer.md ← MỚI v2.0
- devops-checker.md ← MỚI v2.0
- refactor-guide.md ← MỚI v2.0

**Rules cần có (v2.0):**
- common.md ✓ (v1)
- nextjs.md ✓ (v1) — nếu dùng Next.js
- golang.md ✓ (v1) — nếu dùng Go
- python.md ✓ (v1) — nếu dùng Python
- testing.md ← MỚI v2.0 (mọi project)
- supabase.md ← MỚI v2.0 (nếu dùng Supabase)
- tailwind.md ← MỚI v2.0 (nếu dùng Tailwind)
- docker.md ← MỚI v2.0 (nếu có Dockerfile)

Liệt kê những gì đang thiếu.

### Bước 3 — Xác nhận với user

```
📋 AGKit Upgrade: v1.0 → v2.0

Project: [tên project]
Hiện có: [N agents, N rules]

Sẽ thêm vào:
  Agents: database-reviewer, frontend-reviewer, devops-checker, refactor-guide
  Rules:  testing [+ supabase, tailwind, docker nếu phát hiện stack]

Skills (commands) đã tự động update — không cần làm gì.

Tiếp tục? (yes/no)
```

### Bước 4 — Copy files còn thiếu

Với mỗi file còn thiếu, tạo file đó trong `.agkit/` của project hiện tại
bằng cách copy nội dung từ project nguồn (`D:\anti\.agkit\`).

Nếu không tìm thấy project nguồn, tạo files từ template built-in.

**Thứ tự copy:**
1. Agents (4 files mới)
2. Rules theo stack:
   - `testing.md` → luôn copy
   - `supabase.md` → nếu tìm thấy `@supabase/supabase-js` trong package.json
   - `tailwind.md` → nếu tìm thấy `tailwind.config`
   - `docker.md` → nếu tìm thấy `Dockerfile`

### Bước 5 — Cập nhật PROJECT.md

Thêm hoặc cập nhật dòng version trong PROJECT.md:
```markdown
**AGKit Version:** 2.0
**Upgraded:** [YYYY-MM-DD]
```

### Bước 6 — Báo cáo

```
✅ AGKit Upgrade hoàn thành!

📁 Đã thêm vào .agkit/:
   agents/database-reviewer.md
   agents/frontend-reviewer.md
   agents/devops-checker.md
   agents/refactor-guide.md
   rules/testing.md
   rules/tailwind.md    ← Detect Tailwind trong project

🆕 Skills mới có thể dùng ngay (đã global):
   agkit plan    — Breakdown feature/task
   agkit git     — Conventional commits
   agkit debug   — Systematic debugging
   agkit refactor— Safe refactoring
   agkit perf    — Performance analysis
   agkit docs    — Generate documentation
   agkit deploy  — Pre-deploy checklist

Project đã ở AGKit v2.0 🎉
```
