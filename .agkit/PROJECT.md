# PROJECT.md — Bộ nhớ Bền vững của Dự án
# Antigravity đọc file này ĐẦU MỖI PHIÊN để nắm context dự án.
# Cập nhật file này sau mỗi quyết định kiến trúc hoặc thay đổi pattern lớn.

---

## Project Overview

**Tên project:** _(Điền vào khi setup)_
**Mục đích:** _(Mô tả ngắn gọn 1-2 câu: product này giải quyết vấn đề gì cho ai)_
**Giai đoạn hiện tại:** Greenfield / MVP / Production
**URL / Repo:** _(link nếu có)_

---

## Tech Stack

| Layer | Công nghệ | Lý do chọn |
|---|---|---|
| Frontend | _(Next.js 14 / React...)_ | _(Lý do ngắn)_ |
| Backend | _(Go / Python FastAPI...)_ | _(Lý do ngắn)_ |
| Database | _(PostgreSQL / MongoDB...)_ | _(Lý do ngắn)_ |
| Auth | _(Clerk / NextAuth / Supabase...)_ | _(Lý do ngắn)_ |
| Hosting | _(Vercel / Railway / VPS...)_ | _(Lý do ngắn)_ |
| State | _(Zustand / TanStack Query...)_ | _(Lý do ngắn)_ |

---

## Architecture Overview

```
_(Vẽ sơ đồ đơn giản bằng text, ví dụ:)_

Browser → Next.js App Router → Server Actions → PostgreSQL
                            ↓
                     Go Service (API gRPC/REST)
```

---

## Key Patterns (Đã chốt — bắt buộc follow)

> Đây là các pattern đã được quyết định. Antigravity KHÔNG được tự ý thay thế bằng pattern khác
> mà không có sự đồng ý của user và ghi ADR.

- **Data fetching:** _(Ví dụ: Server Components + Server Actions, không dùng useEffect để fetch)_
- **Error handling:** _(Ví dụ: Result type pattern, không throw exception ở service layer)_
- **Auth flow:** _(Ví dụ: Middleware-based route protection với Clerk)_
- **File structure:** _(Ví dụ: Feature-based — mỗi feature có folder riêng với components/, hooks/, types/)_
- **API contracts:** _(Ví dụ: Zod schemas làm single source of truth cho validation)_

---

## ADR Log (Architecture Decision Records)

> Ghi lại mỗi khi có quyết định kiến trúc quan trọng.
> Format: `[date] Title — Quyết định: X. Lý do: Y. Trade-off: Z.`

| Date | Quyết định | Lý do | Trade-off |
|---|---|---|---|
| _(YYYY-MM-DD)_ | _(Ví dụ: Dùng Zustand thay Redux)_ | _(Bundle nhỏ hơn, boilerplate ít hơn)_ | _(Không có Redux DevTools mạnh)_ |

---

## Off-limits (KHÔNG được thay đổi)

> Những phần code/config này đang ổn định và không được sửa nếu không có lý do rõ ràng.

- _(Ví dụ: Schema database đã có production data — không drop column, chỉ migrate)_
- _(Ví dụ: Auth middleware — không refactor khi chưa có E2E test coverage)_

---

## Known Issues / Technical Debt

> Ghi lại bugs đã biết hoặc shortcuts đã chấp nhận để theo dõi.

| Issue | Mức độ | Ghi chú |
|---|---|---|
| _(Mô tả issue)_ | High / Med / Low | _(Kế hoạch fix hoặc lý do defer)_ |

---

## External Dependencies Quan trọng

> Các service/API bên ngoài mà project phụ thuộc — ghi lại để không hardcode sai.

| Service | Dùng cho | Env var |
|---|---|---|
| _(Ví dụ: Stripe)_ | _(Thanh toán)_ | `STRIPE_SECRET_KEY` |
| _(Ví dụ: Resend)_ | _(Gửi email)_ | `RESEND_API_KEY` |
