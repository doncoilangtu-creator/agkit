# agkit-matrix — Xem và quản lý Test Matrix
# Skill này được gọi để xem ma trận kiểm chứng hành vi (Behavior-to-Proof).
# Trigger: "agkit matrix", "test matrix", "xem matrix", "agkit matrix [story_id]"

## Mô tả

Hiển thị và quản lý Test Matrix — ma trận đối chiếu giữa các hành vi yêu cầu trong Spec
với các bằng chứng kiểm chứng (Unit, Integration, E2E, Platform).

---

## Các bước thực hiện

### Bước 1 — Kiểm tra Durable Layer

Kiểm tra `.agkit/bin/agkit-cli.exe` tồn tại. Nếu không → báo cài đặt.

### Bước 2 — Xác định story ID

- Nếu user gõ "agkit matrix US-001" → dùng story ID đó
- Nếu user chỉ gõ "agkit matrix" → đọc `STATUS.md` để tìm story đang active
- Nếu không tìm được → hỏi: "Bạn muốn xem matrix cho story nào?"

### Bước 3 — Truy vấn và hiển thị

Chạy: `agkit-cli matrix query --story <story_id>`

Hiển thị kết quả:

```
📋 Test Matrix — US-001 "User Registration"
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

| Hành vi                  | Unit | Integration | E2E  | Platform |
|--------------------------|------|-------------|------|----------|
| User đăng ký OK          |  ✅  |     ✅      |  ❌  |    —    |
| Email trùng → reject     |  ✅  |     ❌      |  ❌  |    —    |
| Password weak → reject   |  ✅  |     —      |  —  |    —    |

Score: 4/7 (57%) — Cần thêm Integration + E2E
```

### Bước 4 — Cập nhật kết quả (nếu user yêu cầu)

Nếu user nói "cập nhật matrix" hoặc "set unit pass cho behavior X":

Chạy:
```bash
agkit-cli matrix set --story US-001 --behavior "User đăng ký OK" --unit 1 --integration 1 --e2e 0
```

Giá trị: 1 = ✅ pass, -1 = ❌ fail, 0 = — chưa chạy

### Bước 5 — Gợi ý hành động

Dựa trên score:
- Score >= 80% → "✅ Coverage tốt. Sẵn sàng cho /review"
- Score 50-79% → "⚠️ Cần bổ sung thêm tests. Xem các ❌ phía trên."
- Score < 50% → "❌ Coverage thấp. Ưu tiên viết tests trước khi tiếp tục."

---

## Hành vi đặc biệt

### Xem tổng hợp tất cả stories
Nếu user gõ "agkit matrix all" → chạy `agkit-cli matrix query --numeric`
