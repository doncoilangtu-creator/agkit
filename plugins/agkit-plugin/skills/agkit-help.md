# agkit-help — Danh sách tất cả AGKit commands (v3.0)
# Trigger: "agkit help", "agkit", "agkit commands", "lệnh agkit là gì"

## Mô tả

Hiển thị danh sách đầy đủ tất cả 23 AGKit skills/commands kèm mô tả ngắn và workflow.

---

## Output

Khi được gọi, hiển thị chính xác nội dung sau:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 🧰 AGKit — Antigravity Developer Kit v3.0
    23 Skills | Durable Layer | Risk Lanes | Test Matrix
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

 SETUP & SESSION
 ───────────────
 agkit init        Khởi tạo AGKit cho dự án mới
                   Detect stack, hỏi 3 câu, generate .agkit/

 agkit session     Nạp context đầu phiên làm việc
                   Đọc PROJECT.md + STATUS.md + load rules + ghi DB

 agkit status      Dashboard trạng thái project
                   Health score, blocked, next up

 agkit upgrade     Nâng cấp project lên phiên bản mới
                   Copy agents/rules mới, cài Durable Layer

 PLANNING & RISK 🆕
 ──────────────────
 agkit plan        Breakdown feature/task thành subtasks
                   🆕 Intake Classification + Risk Lane Assignment

 agkit intake      🆕 Phân loại rủi ro công việc
                   Auto-classify: 🟢 Tiny / 🟡 Normal / 🔴 High-risk

 agkit adr         Ghi Architecture Decision Record
                   Architect agent → 2-3 phương án → ghi PROJECT.md

 agkit done        Đánh dấu task hoàn thành
                   Cập nhật STATUS.md, dọn Session Notes

 CODE QUALITY
 ────────────
 agkit verify      Chạy test suite + verification loop
                   🆕 Test Matrix Report + trace vào DB

 agkit review      Code review chuyên sâu
                   BLOCKER / WARNING / SUGGESTION + auto-fix

 agkit matrix      🆕 Xem/quản lý Test Matrix
                   Behavior-to-Proof mapping + coverage score

 agkit debug       Debug có hệ thống: reproduce → isolate → fix
                   Phân tích lỗi cụ thể

 agkit refactor    Safe refactoring với test coverage protection
                   Step-by-step plan, verify sau mỗi bước

 SECURITY & PERFORMANCE
 ───────────────────────
 agkit security    Scan bảo mật OWASP Top 10
                   CRITICAL / HIGH / MEDIUM với code fix

 agkit perf        Phân tích performance bottlenecks
                   Bundle size, Core Web Vitals, N+1, profiling

 DOCUMENTATION & DEPLOY
 ──────────────────────
 agkit docs        Scan và báo cáo docs health
                   README, API docs, CHANGELOG

 agkit deploy      Pre-deploy checklist đầy đủ
                   Build + tests + security + migrations + Docker

 agkit git         Tạo commit/PR/branch chuẩn Conventional Commits

 DURABLE LAYER 🆕 (v3.0)
 ────────────────────────
 agkit history     🆕 Xem lịch sử hoạt động từ DB
                   Sessions, traces, intakes, ADRs timeline

 agkit trace       🆕 Ghi hành động quan trọng vào DB
                   summary + outcome → SQLite

 agkit stats       🆕 Thống kê nhanh sức khỏe dự án
                   Dashboard: sessions, ADRs, test coverage

 agkit backlog     🆕 Quản lý danh sách việc cần làm
                   Priority + category + status tracking

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

 📁 FILES (.agkit/)
 ──────────────────
 PROJECT.md        Bộ nhớ bền vững của dự án
 STATUS.md         Trạng thái công việc hiện tại
 VERIFY.md         Verification loop protocol
 agkit.db          🆕 SQLite database (Durable Layer)
 bin/agkit-cli     🆕 Rust CLI binary
 rules/            common · nextjs · golang · python
                   supabase · tailwind · docker · testing
 agents/           architect · code-reviewer · build-resolver
                   security-scanner · database-reviewer
                   frontend-reviewer · devops-checker · refactor-guide

 💡 WORKFLOW CHUẨN v3.0
 ──────────────────────
 Bắt đầu project:  agkit init
 Sáng mỗi ngày:    agkit session
 Nhận task mới:     agkit intake → agkit plan (nếu không tiny)
 Sau khi code:      agkit verify → agkit matrix → agkit review
 Bug xảy ra:        agkit debug
 Trước commit:      agkit git → agkit security
 Trước deploy:      agkit deploy
 Xong task:         agkit done → agkit trace
 Xem tổng quan:     agkit stats → agkit history

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```
