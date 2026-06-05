# INSTRUCTIONS — AGKit Session Protocol
# Antigravity phải đọc file này trước BẤT KỲ thao tác nào trong project.

---

## ⚡ Session Start Checklist (Thực hiện ngay khi bắt đầu phiên)

**Bước 1 — Nạp bộ nhớ dự án:**
Đọc `.agkit/PROJECT.md` để nắm kiến trúc, tech decisions, và các pattern đã chốt.

**Bước 2 — Nắm trạng thái hiện tại:**
Đọc `.agkit/STATUS.md` để biết đang ở đâu, task nào đang dở, và có gì bị block không.

**Bước 3 — Detect tech stack và load rules:**
- Nếu tìm thấy `package.json` → đọc `.agkit/rules/nextjs.md`
- Nếu tìm thấy `go.mod` → đọc `.agkit/rules/golang.md`
- Nếu tìm thấy `requirements.txt` hoặc `pyproject.toml` → đọc `.agkit/rules/python.md`
- Luôn đọc `.agkit/rules/common.md` bất kể stack nào

**Bước 4 — Xác nhận với user:**
Báo ngắn gọn: "Đã nạp context. Đang ở: [task hiện tại từ STATUS.md]. Sẵn sàng."

---

## 🧠 Quy tắc Quản lý Context Window

### Khi nào cập nhật STATUS.md:
- Hoàn thành một task (dù nhỏ)
- Chuyển sang task khác
- Phát hiện blocker mới
- Context window ước tính > 60% (conversation quá dài)

### Khi context window sắp đầy:
1. Tóm tắt những gì đã làm vào `STATUS.md → Last Completed`
2. Ghi task tiếp theo vào `STATUS.md → Next Up`
3. **Thông báo cho user:** "Context window đang dài, nên bắt đầu phiên mới. Tôi đã lưu trạng thái vào STATUS.md."
4. Không tự tiếp tục làm trong phiên cũ sau khi cảnh báo

---

## 🔄 Verification Loop (Bắt buộc sau mỗi thay đổi code)

Xem chi tiết tại `.agkit/VERIFY.md`.

**Tóm tắt:** Sau khi viết/sửa code → chạy test → nếu fail thì tự sửa → chạy lại.
Không được báo "done" khi test chưa pass.

---

## 🤖 Khi nào dùng Subagent

| Tình huống | Subagent cần dùng |
|---|---|
| Thiết kế feature mới / refactor lớn | `.agkit/agents/architect.md` |
| Vừa viết xong một module | `.agkit/agents/code-reviewer.md` |
| Build / compile / import fail | `.agkit/agents/build-resolver.md` |
| Trước khi commit hoặc deploy | `.agkit/agents/security-scanner.md` |

Để invoke: đọc file agent tương ứng và dùng `define_subagent` + `invoke_subagent`.

---

## 📋 Quy tắc Bất biến (Không được vi phạm)

1. **Không xóa hoặc sửa code đang chạy** mà không có test coverage bảo vệ
2. **Không báo "hoàn thành"** khi test chưa pass
3. **Không tự ý thay đổi architecture** mà không ghi ADR vào `PROJECT.md`
4. **Không dùng `any` trong TypeScript**, không tắt linter rule
5. **Không hardcode secret, API key, hoặc credential** bất kỳ đâu trong code
6. **Luôn cập nhật PROJECT.md và STATUS.md** khi có thay đổi đáng kể

---

## 🚀 Smart Setup (Chạy khi project chưa có .agkit/)

Nếu không tìm thấy thư mục `.agkit/` trong project:

```
1. Scan root: package.json, go.mod, requirements.txt, pyproject.toml
2. Hỏi user 3 câu:
   a) "Tên project và mục đích chính là gì?" (1-2 câu)
   b) "Database/infra chính bạn đang dùng?" (PostgreSQL, MongoDB, None...)
   c) "Project đang ở giai đoạn nào?" (Greenfield mới / Đang có code rồi)
3. Generate toàn bộ .agkit/ với context đã có
4. Confirm: "Kit đã sẵn sàng. Đây là những gì tôi sẽ follow..."
```
