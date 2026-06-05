# agkit-adr — Ghi Architecture Decision Record
# Skill này được gọi khi cần record một quyết định kiến trúc.
# Trigger: "agkit adr", "ghi quyết định", "record adr", "agkit architect [vấn đề]"

## Mô tả

Invoke Architect subagent từ `.agkit/agents/architect.md` để phân tích vấn đề,
đưa ra 2-3 phương án với trade-offs, recommendation rõ ràng, và tự động
ghi ADR vào `PROJECT.md`.

---

## Các bước thực hiện

### Bước 1 — Xác định vấn đề cần quyết định

Phân tích request của user:
- "agkit adr [vấn đề]" → Vấn đề đã rõ, tiếp tục ngay
- "agkit adr" (không có gì thêm) → Hỏi: "Bạn đang cần quyết định về vấn đề gì?"

**Các loại quyết định phổ biến:**
- Lựa chọn giữa 2+ libraries/frameworks
- Cách thiết kế database schema
- Kiến trúc API (REST vs GraphQL vs tRPC)
- Cách xử lý authentication/authorization
- Cách tổ chức file structure
- Performance optimization approach

### Bước 2 — Đọc context từ PROJECT.md

Đọc `.agkit/PROJECT.md` để hiểu:
- Tech stack hiện tại (để recommendation phù hợp)
- Key patterns đã chốt (để không đề xuất gì mâu thuẫn)
- ADR log hiện có (để đánh số ADR mới đúng)

Đếm số ADR hiện có → ADR mới sẽ là ADR-[N+1]

### Bước 3 — Invoke Architect subagent

Đọc `.agkit/agents/architect.md` để lấy system prompt.

Dùng `define_subagent` với system prompt của architect.

Prompt cho subagent:
```
Context project: [Tech stack, patterns đã có từ PROJECT.md]

Vấn đề cần quyết định: [Vấn đề từ bước 1]

Yêu cầu:
1. Đề xuất 2-3 phương án với trade-offs cụ thể
2. Recommendation rõ ràng dựa trên context project này
3. Implementation sketch ngắn cho phương án được chọn
4. Format output để sẵn sàng copy vào ADR Log của PROJECT.md

Tuân theo checklist architect trong agents/architect.md.
```

### Bước 4 — Trình bày kết quả cho user

Hiển thị analysis từ architect agent theo format:

```
🏗 Architecture Decision — [Tên vấn đề]

## Các phương án

**A. [Tên phương án A]**
   ✅ Pros: ...
   ❌ Cons: ...

**B. [Tên phương án B]**
   ✅ Pros: ...
   ❌ Cons: ...

## Recommendation
→ Phương án [X] vì [lý do dựa trên context project]

## Implementation sketch
[Code/structure ngắn]
```

### Bước 5 — Xác nhận và ghi vào PROJECT.md

Hỏi user: "Bạn có muốn ghi quyết định này vào ADR Log của PROJECT.md không?"

Nếu user đồng ý (hoặc chọn một phương án):
→ Thêm ADR entry mới vào `PROJECT.md → ADR Log`:

```markdown
| YYYY-MM-DD | ADR-[N]: [Tiêu đề] | [Lý do chọn] | [Trade-off chấp nhận] |
```

→ Nếu quyết định ảnh hưởng đến Key Patterns, cập nhật section đó luôn.

→ Thông báo: "✅ ADR-[N] đã được ghi vào PROJECT.md"
