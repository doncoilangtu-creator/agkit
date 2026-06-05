# agkit-refactor — Safe Refactoring Workflow
# Trigger: "agkit refactor", "refactor code này", "cleanup", "agkit refactor [target]"

## Mô tả

Invoke Refactor Guide subagent để lập kế hoạch và thực hiện refactoring an toàn,
từng bước nhỏ có test coverage bảo vệ. Không bao giờ refactor mà không có tests.

---

## Các bước thực hiện

### Bước 1 — Xác định target refactor

| User nói gì | Target |
|---|---|
| "agkit refactor" | Hỏi: "Bạn muốn refactor gì?" |
| "agkit refactor [file]" | File đó |
| "agkit refactor feature/auth" | Folder đó |
| "cleanup dead code" | Scan toàn project tìm dead code |
| "extract [function/module]" | Specific extraction |

### Bước 2 — Kiểm tra test coverage hiện tại

Scan tests cho target:
- Nếu coverage < 60%: **Dừng và cảnh báo**
  > "⚠️ Coverage quá thấp để refactor an toàn.
  > Gọi `agkit plan` để lên kế hoạch viết tests trước."
- Nếu coverage ≥ 60%: Tiếp tục với cảnh báo về phần chưa cover
- Nếu không có tests: **Từ chối refactor**, yêu cầu viết tests trước

### Bước 3 — Đọc Refactor Guide

Đọc `.agkit/agents/refactor-guide.md` để lấy:
- System prompt của refactor agent
- Risk assessment framework
- Step-by-step planning templates
- Anti-patterns cần tránh

### Bước 4 — Invoke Refactor subagent

Invoke subagent với prompt:
```
Target: [target]
Test coverage: [X%]

1. Đánh giá risk level cho refactor này
2. Tạo kế hoạch từng bước nhỏ (mỗi bước không quá 30 phút)
3. Với mỗi bước: chỉ rõ files thay đổi và cách verify
4. Commit message cho mỗi bước
5. Rollback plan nếu có bước nào fail
```

### Bước 5 — Thực hiện từng bước

Sau khi có plan, thực hiện **từng bước một**:
1. Thực hiện bước N
2. Chạy `agkit verify` sau mỗi bước
3. Nếu tests pass → commit, tiếp tục bước N+1
4. Nếu tests fail → revert bước đó, phân tích lại

**Không bao giờ skip bước verify giữa chừng.**

### Bước 6 — Báo cáo và cập nhật

Sau khi hoàn thành:
```
✅ Refactor hoàn thành!

📊 Kết quả:
   Files thay đổi: [N]
   Tests: [N] passed
   Code reduction: [N] lines removed (nếu có)

📝 Commits đã tạo:
   - refactor: [bước 1]
   - refactor: [bước 2]
   ...
```

Cập nhật STATUS.md với kết quả refactor.
