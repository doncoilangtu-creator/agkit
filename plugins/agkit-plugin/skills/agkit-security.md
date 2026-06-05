# agkit-security — Security Scan trước commit/deploy
# Skill này được gọi để scan lỗ hổng bảo mật trong code.
# Trigger: "agkit security", "scan bảo mật", "kiểm tra security", "agkit security [folder]"

## Mô tả

Tạo Security Scanner subagent từ `.agkit/agents/security-scanner.md`, scan code theo
OWASP Top 10 và tìm hardcoded secrets. Trả về report phân loại CRITICAL/HIGH/MEDIUM/LOW
với verdict PASS/FAIL rõ ràng.

---

## Các bước thực hiện

### Bước 1 — Xác định scope scan

| User nói gì | Scope |
|---|---|
| "agkit security" | Toàn bộ source code (bỏ qua node_modules, .git, dist, build) |
| "agkit security [folder]" | Chỉ folder đó |
| "agkit security trước commit" | Git staged files (`git diff --cached --name-only`) |

### Bước 2 — Chạy automated scans (nếu tools có sẵn)

Trước khi invoke subagent, chạy các lệnh scan tự động nếu tools được cài:

**Node.js:**
```bash
npm audit --audit-level=moderate 2>&1 | head -30
```

**Go:**
```bash
go list -m all 2>/dev/null | head -5  # Kiểm tra go vuln có không
```

**Python:**
```bash
pip-audit 2>&1 | head -30  # Nếu pip-audit được cài
```

**Grep hardcoded secrets (mọi stack):**
```bash
# Chạy grep patterns tìm secrets phổ biến
grep -rn --include="*.ts" --include="*.tsx" --include="*.js" \
     --include="*.go" --include="*.py" \
     -E "(sk_live_|AKIA|ghp_|password\s*=\s*[\"'][^\"']+[\"']|api_key\s*=)" \
     . --exclude-dir={node_modules,.git,dist,build,.next} 2>/dev/null | head -20
```

Thu thập output để cung cấp cho subagent làm input bổ sung.

### Bước 3 — Đọc system prompt từ agents/security-scanner.md

Đọc `.agkit/agents/security-scanner.md` để lấy:
- System prompt của scanner
- OWASP Top 10 checklist đầy đủ
- Output format chuẩn

### Bước 4 — Invoke Security Scanner subagent

Dùng `define_subagent` với system prompt từ security-scanner.md.

Prompt cho subagent:
```
Scan [scope] theo OWASP Top 10 checklist trong .agkit/agents/security-scanner.md.

Kết quả automated scan sẵn có:
[output từ bước 2]

Phân tích thêm code thủ công với checklist đầy đủ.
Trả về report với CRITICAL/HIGH/MEDIUM/LOW/INFO và verdict PASS/FAIL.
Với mỗi finding: mô tả rủi ro thực tế + code fix cụ thể.
```

### Bước 5 — Xử lý kết quả

**Nếu FAIL (có CRITICAL hoặc HIGH):**
```
🚨 SECURITY SCAN: FAIL

[Hiển thị đầy đủ report]

❗ KHÔNG nên commit/deploy cho đến khi fix xong các CRITICAL và HIGH findings.
Bạn muốn tôi fix ngay không?
```

**Nếu PASS (chỉ có MEDIUM/LOW/INFO):**
```
✅ SECURITY SCAN: PASS

[Hiển thị summary ngắn]

⚠️ Có [N] WARNING cần lưu ý (không block commit):
[Danh sách ngắn]
```

### Bước 6 — Auto-fix offer

Nếu có CRITICAL/HIGH và user muốn fix:
- Fix từng issue theo thứ tự CRITICAL → HIGH
- Sau mỗi fix, chạy lại grep để verify secret đã được remove
- Cập nhật STATUS.md với kết quả scan

### Bước 7 — Cập nhật STATUS.md

```
[HH:MM] Security scan [scope]: [PASS/FAIL] — [N CRITICAL, N HIGH, N MEDIUM]
```
