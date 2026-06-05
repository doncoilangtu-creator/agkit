# agkit-intake — Phân loại rủi ro công việc
# Skill này được gọi để phân loại mức độ rủi ro của một yêu cầu.
# Trigger: "agkit intake", "phân loại rủi ro", "classify risk", "agkit intake [mô tả]"

## Mô tả

Phân loại mức độ rủi ro của một yêu cầu/task để xác định quy trình làm việc phù hợp.
Dựa trên mô hình Feature Intake từ Harness Engineering.

---

## Các bước thực hiện

### Bước 1 — Thu thập mô tả công việc

- Nếu user gõ "agkit intake [mô tả]" → dùng mô tả đó
- Nếu chỉ gõ "agkit intake" → hỏi: "Mô tả công việc bạn cần phân loại?"

### Bước 2 — Xác định Input Type

Phân loại tự động dựa trên mô tả:

| Input Type | Keyword/Signal |
|---|---|
| `new_spec` | "tạo mới", "dự án mới", "setup", "scaffold" |
| `change_request` | "thêm", "sửa", "thay đổi", "modify" |
| `bug_fix` | "bug", "lỗi", "fix", "broken", "sai" |
| `refactor` | "refactor", "tái cấu trúc", "optimize", "clean up" |
| `maintenance` | "update", "upgrade", "dependency", "config" |
| `harness_improvement` | "agkit", "harness", "rule", "agent", "skill" |

### Bước 3 — Chạy Risk Checklist

Đánh giá điểm rủi ro:

| Tiêu chí | Điểm |
|---|---|
| DB schema thay đổi? | +2 |
| Auth/Payment liên quan? | +3 |
| Public API thay đổi? | +2 |
| Estimate > 3 ngày? | +1 |
| Ảnh hưởng > 5 files? | +1 |
| Breaking change? | +2 |
| Chạm vào Off-limits (từ PROJECT.md)? | +3 |

### Bước 4 — Xác định Lane

| Tổng điểm | Lane | Biểu tượng |
|---|---|---|
| 0-1 | `tiny` | 🟢 |
| 2-4 | `normal` | 🟡 |
| 5+ | `high_risk` | 🔴 |

### Bước 5 — Ghi vào DB và báo cáo

Chạy:
```bash
agkit-cli intake --type <type> --lane <lane> --summary "<mô tả>"
```

Hiển thị:

```
🔄 Intake Classification
━━━━━━━━━━━━━━━━━━━━━━━━━

📝 Mô tả:     Thêm tính năng đăng ký bằng Google OAuth
📋 Input Type: change_request
⚡ Risk Score: 5 (DB schema +2, Auth +3)
🔴 Lane:       HIGH-RISK

📌 Quy trình bắt buộc:
   1. /plan chi tiết + Mermaid diagram
   2. User duyệt trước khi code
   3. /security — quét bảo mật bắt buộc
   4. /verify — chạy test suite
   5. /review — code review
```

---

## Hành vi đặc biệt

### User không đồng ý lane
Nếu user nói "không, đây chỉ là tiny" → cho phép override nhưng ghi chú:
→ `agkit-cli intake --type <type> --lane tiny --summary "<mô tả> [USER OVERRIDE from high_risk]"`
