# agkit-stats — Thống kê nhanh dự án
# Skill này hiển thị dashboard thống kê từ Durable Layer.
# Trigger: "agkit stats", "thống kê", "project stats", "agkit stats json"

## Mô tả

Hiển thị dashboard thống kê tổng hợp về sức khỏe dự án:
số sessions, ADRs, traces, stories, intakes, test coverage score.

---

## Các bước thực hiện

### Bước 1 — Kiểm tra Durable Layer

Kiểm tra `.agkit/bin/agkit-cli.exe` tồn tại. Nếu không → báo cài đặt.

### Bước 2 — Chạy lệnh thống kê

```bash
agkit-cli stats
```

Hoặc nếu user yêu cầu JSON:
```bash
agkit-cli stats --json
```

### Bước 3 — Hiển thị dashboard

```
📊 AGKit Project Statistics
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📂 Sessions:     15 total (1 active)
🏗  ADRs:          5
📝 Traces:        42 total (✅35 ❌5 ⚠️2)
📖 Stories:       8/12 done
🔄 Intakes:       20 total (🟢8 🟡10 🔴2)
📋 Backlog:       3 open items
🧪 Test Matrix:   45/52 checks passed (86%)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Bước 4 — Phân tích và gợi ý

Dựa trên dữ liệu:
- Tỷ lệ trace failure cao (>30%) → "⚠️ Tỷ lệ lỗi cao. Xem xét chạy `/debug` hoặc `/refactor`."
- Test coverage thấp (<60%) → "⚠️ Test coverage thấp. Ưu tiên viết thêm tests."
- Backlog nhiều items (>10) → "📋 Backlog đang dài. Xem xét ưu tiên và dọn dẹp."
- Stories blocked → "🚧 Có stories đang bị blocked. Kiểm tra STATUS.md."

---

## Hành vi đặc biệt

### So sánh với global stats
Nếu user hỏi "so sánh với các project khác":
→ Query global DB để so sánh metrics giữa các projects.
