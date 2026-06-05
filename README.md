# AGKit v2.0 — Antigravity Developer Kit

**AGKit v2.0** là một bộ công cụ phát triển và hỗ trợ lập trình cặp (pair programming) tùy chỉnh tối ưu cho **Antigravity AI Assistant**. Bộ công cụ này giúp tự động hóa quy trình làm việc, thiết lập các tiêu chuẩn code, tự sửa lỗi kiểm thử (self-healing tests), và hỗ trợ đánh giá bảo mật cũng như kiến trúc hệ thống.

---

## 📂 Cấu trúc dự án

Dự án được tổ chức như sau:

*   **`.agkit/`**: Chứa các cấu hình lõi của AGKit:
    *   `PROJECT.md`, `STATUS.md`, `VERIFY.md`: Theo dõi trạng thái và quy trình xác minh dự án.
    *   `rules/`: Các tiêu chuẩn lập trình cho các công nghệ phổ biến (Next.js, Golang, Python, Supabase, Tailwind, Docker, Testing).
    *   `agents/`: Các vai trò Agent chuyên biệt (Architect, Code Reviewer, Security Scanner, DevOps, v.v.).
*   **`.harness/`**: Bộ khung kiểm thử tự sửa lỗi (Self-Healing Test Harness).
*   **`plugins/agkit-plugin/`**: Plugin cài đặt cho Antigravity (chứa 17 skills hỗ trợ tự động hóa như khởi tạo dự án, lập kế hoạch, refactor, kiểm tra bảo mật, deploy...).
*   **`guide/`**: Giao diện hướng dẫn HTML tương tác trực quan đẹp mắt để tra cứu nhanh các lệnh và copy-paste.
*   **`INSTRUCTIONS.md`**: Tài liệu hướng dẫn tích hợp và quy định chung của dự án.

---

## 🛠️ Hướng dẫn cài đặt & sử dụng

### 1. Cài đặt Cấu hình Dự án (.agkit)
Để áp dụng bộ kit này cho một dự án mới, hãy sao chép thư mục `.agkit/` và file `INSTRUCTIONS.md` vào thư mục gốc của dự án đó:
```bash
cp -r .agkit /path/to/your/new-project/
cp INSTRUCTIONS.md /path/to/your/new-project/
```

### 2. Cài đặt Plugin cho Antigravity
Để tích hợp 17 phím tắt/skills mới vào Antigravity, hãy sao chép thư mục plugin vào thư mục cấu hình của hệ thống:
```powershell
# Trên Windows
xcopy /E /I plugins\agkit-plugin "$env:USERPROFILE\.gemini\config\plugins\agkit-plugin"
```

### 3. Tra cứu nhanh bằng HTML Guide
Mở file `guide/index.html` trong trình duyệt của bạn hoặc click vào shortcut ngoài Desktop để xem danh sách phím tắt và nhấp để copy nhanh lệnh cần dùng.

---

## 🚀 Các Skills Hỗ trợ trong Plugin

Dưới đây là một số phím tắt/skills tiêu biểu trong tổng số 17 skills:
1.  **`/init`**: Khởi tạo cấu hình AGKit cho dự án mới.
2.  **`/session`**: Bắt đầu phiên làm việc mới, ghi nhận ngữ cảnh.
3.  **`/status`**: Kiểm tra trạng thái hiện tại của dự án.
4.  **`/verify`**: Chạy bộ tự sửa lỗi (self-healing verification).
5.  **`/review`**: Đánh giá chất lượng mã nguồn.
6.  **`/security`**: Quét lỗi bảo mật.
7.  **`/plan`**: Lập kế hoạch triển khai kiến trúc.
8.  **`/refactor`**: Đề xuất cải tiến và tối ưu cấu trúc code.
9.  **`/deploy`**: Kiểm tra các bước triển khai DevOps.
... và nhiều lệnh hữu ích khác chi tiết trong hướng dẫn HTML.

---
*Phát triển bởi đội ngũ Hoanghitech & Antigravity Assistant.*
