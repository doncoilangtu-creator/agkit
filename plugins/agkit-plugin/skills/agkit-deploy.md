# agkit-deploy — Pre-Deploy Checklist
# Trigger: "agkit deploy", "chuẩn bị deploy", "deploy check", "sắp lên production"

## Mô tả

Chạy toàn bộ pre-deploy checklist: build verification, security scan, performance check,
database migration check, và environment validation. Invoke DevOps Checker subagent.
Chỉ cho phép deploy khi không có BLOCKER nào.

---

## Các bước thực hiện

### Bước 1 — Detect môi trường deploy

Hỏi nếu không rõ:
- "Bạn đang deploy lên môi trường nào?" → staging / production
- Production mode: Strict hơn, không cho phép WARNING chưa giải quyết
- Staging mode: Cho phép WARNING với acknowledgment

### Bước 2 — Automated Checks (chạy ngay, không cần approve)

```bash
# 1. Build check
npm run build 2>&1 | tail -20

# 2. Type check
npx tsc --noEmit 2>&1 | head -20

# 3. Tests
npm test -- --passWithNoTests 2>&1 | tail -10

# 4. Tìm console.log debug còn sót
grep -rn "console\.log\|console\.debug\|console\.error" src/ \
  --include="*.ts" --include="*.tsx" \
  --exclude="*.test.*" | head -10

# 5. Tìm TODO/FIXME còn sót
grep -rn "TODO\|FIXME\|HACK\|XXX" src/ --include="*.ts" | head -10

# 6. Check env vars có đủ không
node -e "
const required = ['DATABASE_URL', 'NEXTAUTH_SECRET'];
const missing = required.filter(k => !process.env[k]);
if (missing.length) { console.error('Missing:', missing.join(', ')); process.exit(1); }
console.log('Env vars OK');
"
```

### Bước 3 — Security Scan

Chạy `agkit-security` tự động:
- Nếu FAIL (có CRITICAL/HIGH): **BLOCK deploy**
  > "🚨 DEPLOY BLOCKED: Security scan FAIL. Fix [N] critical issues trước."
- Nếu PASS: Tiếp tục

### Bước 4 — Database Migration Check

```bash
# Kiểm tra có pending migrations không
# Next.js + Prisma:
npx prisma migrate status

# Supabase:
supabase db diff  # Xem có thay đổi chưa apply

# Go + goose:
goose status
```

Nếu có pending migrations:
- Hỏi: "Có [N] migrations chưa chạy. Bạn đã có plan chạy migrations trước/sau deploy chưa?"

### Bước 5 — DevOps Checker subagent

Đọc `.agkit/agents/devops-checker.md`, invoke subagent để check:
- Dockerfile best practices
- docker-compose.yml production settings
- Environment variables completeness
- Health check endpoint có không

### Bước 6 — Deploy Report

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 🚀 Pre-Deploy Report — [env]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

 Build:        ✅ PASS
 TypeScript:   ✅ PASS
 Tests:        ✅ 47/47 passed
 Security:     ✅ PASS (0 Critical, 0 High)
 Migrations:   ⚠️  2 pending — cần chạy trước deploy
 Docker:       ✅ PASS
 Env Vars:     ✅ All required vars present

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Verdict: READY WITH WARNINGS

 Action required:
 □ Chạy pending migrations: npx prisma migrate deploy
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Nếu tất cả PASS:**
```
━━━━━━━━━━━━━━━━━━━━━
 ✅ READY TO DEPLOY
━━━━━━━━━━━━━━━━━━━━━
 Tất cả checks passed.
 Chúc deploy thành công! 🎉
━━━━━━━━━━━━━━━━━━━━━
```
