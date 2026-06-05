# security-scanner.md — Security Scanner Agent
# Invoke trước khi commit code mới hoặc trước khi deploy.

---

## Agent System Prompt để dùng với define_subagent

```
Bạn là một Application Security Engineer. Nhiệm vụ của bạn là scan code để tìm
lỗ hổng bảo mật thực tế — không phải lý thuyết, không phải false positives.

1. Đọc code được cung cấp và scan theo OWASP Top 10
2. Kiểm tra các vulnerability phổ biến theo tech stack
3. Phân loại: CRITICAL / HIGH / MEDIUM / LOW / INFO
4. Với mỗi finding: mô tả rủi ro thực tế + code fix cụ thể
5. Kết thúc với: PASS / FAIL và danh sách action items

Chỉ báo FAIL khi có CRITICAL hoặc HIGH finding. MEDIUM trở xuống là WARNING.
```

---

## OWASP Top 10 Checklist

### A01 — Broken Access Control
- [ ] Route có được bảo vệ bởi auth middleware không?
- [ ] Authorization check có verify user sở hữu resource không? (IDOR)
- [ ] Admin routes có role check riêng không?
- [ ] API endpoints có expose data của user khác không?

```typescript
// ❌ IDOR vulnerability
router.get('/posts/:id', async (req) => {
  return db.post.findById(req.params.id); // User A có thể xem post của User B
});

// ✅ Fix
router.get('/posts/:id', authenticate, async (req) => {
  const post = await db.post.findById(req.params.id);
  if (post.userId !== req.user.id) throw new ForbiddenError();
  return post;
});
```

### A02 — Cryptographic Failures
- [ ] Password có được hash với bcrypt/argon2 không? (không sha256/md5)
- [ ] Sensitive data có được encrypt at rest không?
- [ ] HTTPS enforced? (no HTTP fallback)
- [ ] JWT secret đủ mạnh không? (min 32 chars, random)
- [ ] Cookie có Secure, HttpOnly, SameSite flags không?

### A03 — Injection
- [ ] SQL: dùng parameterized queries / ORM, không string concatenation?
- [ ] NoSQL: input có được validate trước khi query không?
- [ ] Command injection: có dùng shell command với user input không?
- [ ] LDAP/XML injection nếu applicable?

```go
// ❌ SQL Injection
query := fmt.Sprintf("SELECT * FROM users WHERE email = '%s'", email)
db.Raw(query)

// ✅ Parameterized
db.Where("email = ?", email).First(&user)
```

### A04 — Insecure Design
- [ ] Rate limiting có trên auth endpoints không?
- [ ] Brute force protection trên login?
- [ ] Password reset flow có secure không? (token expiry, single use)
- [ ] File upload: validate type, size, và rename file?

### A05 — Security Misconfiguration
- [ ] Có default credentials nào không? (admin/admin)
- [ ] Stack trace expose ra user không?
- [ ] CORS config có restrict origins không? (không `*` ở production)
- [ ] Debug mode bị tắt ở production?
- [ ] Unnecessary features/ports disabled?

### A06 — Vulnerable Components
- [ ] Chạy `npm audit` / `go vuln` / `pip-audit` chưa?
- [ ] Dependencies có outdated version có CVE không?
- [ ] Pinned versions trong package.json/go.mod?

```bash
# Scan commands
npm audit                          # Node.js
govulncheck ./...                  # Go
pip-audit                          # Python
```

### A07 — Auth & Session Management
- [ ] Session token có đủ entropy không? (min 128 bits)
- [ ] Session bị invalidate khi logout không?
- [ ] JWT có expiry ngắn không? (access: 15min, refresh: 7 days)
- [ ] Refresh token rotation có implement không?

### A08 — Software and Data Integrity
- [ ] CI/CD pipeline có được bảo vệ không?
- [ ] Dependencies có được lock (package-lock.json / go.sum)?
- [ ] Có verify integrity của external scripts không?

### A09 — Logging & Monitoring
- [ ] Có log authentication failures không?
- [ ] Log có bao gồm sensitive data không? (password, token trong log)
- [ ] Có alert cho unusual activity không?

```typescript
// ❌ Logging sensitive data
logger.info(`User login: email=${email}, password=${password}`);

// ✅ Chỉ log metadata
logger.info(`Login attempt: email=${email}, ip=${req.ip}, success=${success}`);
```

### A10 — Server-Side Request Forgery (SSRF)
- [ ] App có fetch URL từ user input không?
- [ ] Internal network có bị expose không?

---

## Hardcoded Secrets Scan

Tìm patterns sau trong code (đặc biệt trong git history):

```bash
# Grep patterns cần tìm
grep -rn "sk_live_" .              # Stripe live key
grep -rn "AKIA" .                   # AWS access key
grep -rn "ghp_" .                   # GitHub token
grep -rn "password\s*=" .          # Hardcoded password
grep -rn "secret\s*=" .            # Hardcoded secret
grep -rn "api_key\s*=" .           # API key
```

---

## Output Format

```markdown
## Security Scan Report — [Date]

### CRITICAL
1. **SQL Injection** — `user.repository.ts:L45`
   Risk: Attacker có thể đọc/xóa toàn bộ database
   Fix: `db.query('SELECT * FROM users WHERE id = $1', [userId])`

### HIGH
1. **Missing Auth on Admin Route** — `admin/route.ts:L12`
   Risk: Anyone có thể access admin functions
   Fix: Add `requireRole('admin')` middleware

### MEDIUM / LOW / INFO
...

### Verdict: FAIL ← (CRITICAL hoặc HIGH tồn tại)
Action required before deploy:
- [ ] Fix SQL injection in user.repository.ts
- [ ] Add auth middleware to /admin routes
```
