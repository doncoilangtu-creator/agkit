# agkit-review — Code Review tự động
# Skill này được gọi để review code vừa viết hoặc một file/module cụ thể.
# Trigger: "agkit review", "review code này", "kiểm tra code", "agkit review [file/folder]"

## Mô tả

Tạo một Code Reviewer subagent với system prompt chuyên biệt từ
`.agkit/agents/code-reviewer.md`, giao cho nó review target được chỉ định,
và trả về report phân loại BLOCKER / WARNING / SUGGESTION với fix examples cụ thể.

---

## Các bước thực hiện

### Bước 1 — Xác định target review

Phân tích request của user:

| User nói gì | Target review |
|---|---|
| "agkit review" (không có gì thêm) | Files được thay đổi gần nhất (git diff HEAD) |
| "agkit review [tên file]" | File/folder cụ thể đó |
| "agkit review feature/auth" | Tất cả files trong folder đó |
| Có code được paste trong chat | Code đó |

Nếu không xác định được target → Hỏi: "Bạn muốn review file nào hoặc code nào?"

### Bước 2 — Đọc rules phù hợp

Detect stack từ file target:
- `.ts`, `.tsx` → đọc `.agkit/rules/nextjs.md` + `common.md`
- `.go` → đọc `.agkit/rules/golang.md` + `common.md`
- `.py` → đọc `.agkit/rules/python.md` + `common.md`
- Mixed → đọc tất cả relevant rules

### Bước 3 — Đọc system prompt từ agents/code-reviewer.md

Đọc `.agkit/agents/code-reviewer.md` để lấy:
- System prompt của reviewer agent
- Full review checklist (BLOCKER / WARNING / SUGGESTION)
- Output format chuẩn

### Bước 4 — Tạo và invoke Code Reviewer subagent

Dùng `define_subagent` với system prompt từ bước 3.

Prompt cho subagent:
```
Review [target] theo checklist đầy đủ trong .agkit/agents/code-reviewer.md.
Rules áp dụng: [danh sách rules từ bước 2].
Trả về report đúng format: BLOCKER / WARNING / SUGGESTION với code fix cụ thể.
Kết thúc bằng verdict: APPROVED / APPROVED_WITH_CHANGES / NEEDS_REVISION.
```

### Bước 5 — Xử lý kết quả

Sau khi subagent trả về report:

**Nếu có BLOCKER:**
- Hiển thị report đầy đủ
- Hỏi: "Bạn muốn tôi fix ngay các BLOCKER này không?"
- Nếu user đồng ý → Fix từng BLOCKER theo thứ tự, chạy lại verify sau mỗi fix

**Nếu chỉ có WARNING / SUGGESTION:**
- Hiển thị report
- Hỏi: "Bạn muốn tôi fix các WARNING này không, hay để sau?"

**Nếu APPROVED:**
- Chúc mừng ngắn gọn
- Hỏi: "Bạn muốn chạy security scan trước khi commit không? (`agkit security`)"

### Bước 6 — Cập nhật STATUS.md

Thêm vào Session Notes:
```
[HH:MM] Code review [target]: [verdict] — [số issues nếu có]
```
