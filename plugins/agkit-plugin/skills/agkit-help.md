# agkit-help — Danh sách tất cả AGKit commands
# Trigger: "agkit help", "agkit", "agkit commands", "lệnh agkit là gì"

## Mô tả

Hiển thị danh sách đầy đủ tất cả AGKit skills/commands kèm mô tả ngắn và workflow.

---

## Output

Khi được gọi, hiển thị chính xác nội dung sau:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 🧰 AGKit — Antigravity Developer Kit v2.0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

 SETUP & SESSION
 ───────────────
 agkit init        Khởi tạo AGKit cho dự án mới
                   Detect stack, hỏi 3 câu, generate .agkit/

 agkit session     Nạp context đầu phiên làm việc
                   Đọc PROJECT.md + STATUS.md + load rules

 agkit status      Dashboard trạng thái project
                   Health score, blocked, next up

 agkit upgrade     Nâng cấp project từ v1.0 → v2.0
                   Copy agents/rules mới, auto-detect stack

 PLANNING & WORKFLOW
 ───────────────────
 agkit plan        Breakdown feature/task thành subtasks
 agkit plan [feature]
                   Phase plan, estimates, risk assessment

 agkit done        Đánh dấu task hoàn thành
                   Cập nhật STATUS.md, dọn Session Notes

 agkit git         Tạo commit message chuẩn Conventional Commits
 agkit git pr      Tạo PR description
 agkit git branch [name]
                   Suggest branch name

 CODE QUALITY
 ────────────
 agkit verify      Chạy test suite + verification loop
                   Tự động fix nếu fail (tối đa 3 vòng)

 agkit review      Code review chuyên sâu
 agkit review [file/folder]
                   BLOCKER / WARNING / SUGGESTION + auto-fix

 agkit debug       Debug có hệ thống: reproduce → isolate → fix
 agkit debug [error]
                   Phân tích lỗi cụ thể

 agkit refactor    Safe refactoring với test coverage protection
 agkit refactor [target]
                   Step-by-step plan, verify sau mỗi bước

 SECURITY & PERFORMANCE
 ───────────────────────
 agkit security    Scan bảo mật OWASP Top 10
 agkit security [folder]
                   CRITICAL / HIGH / MEDIUM với code fix

 agkit perf        Phân tích performance bottlenecks
 agkit perf [frontend|database|api]
                   Bundle size, Core Web Vitals, N+1, profiling

 DOCUMENTATION
 ─────────────
 agkit docs        Scan và báo cáo docs health
 agkit docs readme Cập nhật README.md
 agkit docs api    Generate API documentation
 agkit docs changelog
                   Tạo CHANGELOG entry từ git log

 DEPLOY
 ──────
 agkit deploy      Pre-deploy checklist đầy đủ
                   Build + tests + security + migrations + Docker

 ARCHITECTURE & MEMORY
 ─────────────────────
 agkit adr         Ghi Architecture Decision Record
 agkit adr [vấn đề]
                   Architect agent → 2-3 phương án → ghi PROJECT.md

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

 📁 FILES (.agkit/)
 ──────────────────
 PROJECT.md        Bộ nhớ bền vững của dự án
 STATUS.md         Trạng thái công việc hiện tại
 VERIFY.md         Verification loop protocol
 rules/            common · nextjs · golang · python
                   supabase · tailwind · docker · testing
 agents/           architect · code-reviewer · build-resolver
                   security-scanner · database-reviewer
                   frontend-reviewer · devops-checker · refactor-guide

 💡 WORKFLOW CHUẨN
 ─────────────────
 Bắt đầu project:  agkit init
 Sáng mỗi ngày:    agkit session
 Lên kế hoạch:     agkit plan
 Sau khi code:      agkit verify → agkit review
 Bug xảy ra:        agkit debug
 Trước commit:      agkit git → agkit security
 Trước deploy:      agkit deploy
 Xong task:         agkit done

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```
