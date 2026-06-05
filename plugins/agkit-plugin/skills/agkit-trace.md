# agkit-trace — Ghi nhận hành động quan trọng
# Skill này được gọi để ghi lại một hành động/sự kiện quan trọng vào Durable Layer.
# Trigger: "agkit trace", "ghi trace", "log action", "agkit trace [mô tả] [outcome]"

## Mô tả

Ghi nhận một hành động quan trọng vào cơ sở dữ liệu SQLite.
Mỗi trace bao gồm: mô tả hành động, kết quả (success/failure/partial/escalated),
và tự động liên kết với session hiện tại.

---

## Các bước thực hiện

### Bước 1 — Thu thập thông tin

Phân tích request:
- "agkit trace 'Fixed N+1 query' success" → có đủ thông tin
- "agkit trace" → hỏi: "Mô tả hành động cần ghi nhận? Và kết quả (success/failure/partial)?"

### Bước 2 — Xác định Outcome

Nếu user không chỉ định outcome, tự động phân loại:
- Có keyword "fix", "done", "complete", "pass" → `success`
- Có keyword "fail", "error", "bug", "broken" → `failure`
- Có keyword "partial", "wip", "incomplete" → `partial`
- Có keyword "escalate", "stuck", "blocked" → `escalated`
- Không rõ → hỏi user

### Bước 3 — Ghi vào DB

Chạy:
```bash
agkit-cli trace --summary "<mô tả>" --outcome <outcome>
```

### Bước 4 — Xác nhận

Hiển thị:
```
✅ Trace recorded: Fixed N+1 query in user list [success]
   Session: #12 | Time: 2024-01-15 10:30
```

---

## Hành vi đặc biệt

### Tự động trace
Các skill khác có thể tự động gọi trace khi hoàn thành:
- `/verify` PASS → trace "Verify: all tests passed" success
- `/verify` FAIL → trace "Verify: tests failed" failure
- `/security` → trace "Security scan: N issues found" success/failure
- `/deploy` → trace "Deploy checklist completed" success
