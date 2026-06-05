# agkit-backlog — Quản lý danh sách việc cần làm
# Skill này quản lý backlog items trong Durable Layer.
# Trigger: "agkit backlog", "backlog", "danh sách việc", "agkit backlog add [title]"

## Mô tả

Quản lý danh sách việc cần làm (backlog) bao gồm: product features, tech debt,
và harness improvements. Mỗi item có priority và category.

---

## Các bước thực hiện

### Bước 1 — Kiểm tra Durable Layer

Kiểm tra `.agkit/bin/agkit-cli.exe` tồn tại. Nếu không → báo cài đặt.

### Bước 2 — Xác định hành động

Phân tích request:
- "agkit backlog" → liệt kê tất cả items đang open
- "agkit backlog add 'Thêm E2E test cho payment' high" → thêm mới
- "agkit backlog done 3" → đánh dấu item #3 là done
- "agkit backlog all" → liệt kê tất cả (kể cả done)

### Bước 3 — Thực thi

**Liệt kê:**
```bash
agkit-cli backlog list --status open
```

**Thêm mới:**
```bash
agkit-cli backlog add --title "<title>" --priority <high|medium|low|critical> --category <product|tech_debt|harness>
```

**Đánh dấu done:**
```bash
agkit-cli backlog done --id <id>
```

### Bước 4 — Hiển thị kết quả

```
📋 Backlog (open items)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#  Title                              Priority  Category
1  Thêm E2E test cho payment flow     🟠 high    product
2  Refactor database connection pool  🟡 medium  tech_debt
3  Cập nhật VERIFY.md cho Go stack    🟢 low     harness
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Bước 5 — Gợi ý

- Nếu có items priority critical/high → "🔴 Có items ưu tiên cao. Xem xét xử lý trước."
- Nếu category harness nhiều → "💡 Nhiều harness improvements pending. Chạy `/upgrade`?"

---

## Hành vi đặc biệt

### Tự động thêm backlog
Các skill khác có thể tự động thêm backlog:
- `/verify` fail nhiều lần → backlog "Fix flaky tests" high
- `/security` tìm thấy issues → backlog "Fix security issues" critical
- `/review` gợi ý refactor → backlog "Refactor [component]" medium
