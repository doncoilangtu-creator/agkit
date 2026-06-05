# agkit-docs — Auto-generate & Sync Documentation
# Trigger: "agkit docs", "tạo docs", "cập nhật README", "document code này"

## Mô tả

Tự động tạo hoặc cập nhật documentation: README.md, JSDoc/GoDoc comments,
API docs từ code, và CHANGELOG entries. Đảm bảo docs luôn sync với code thực tế.

---

## Các bước thực hiện

### Bước 1 — Xác định loại docs cần tạo/cập nhật

| User nói gì | Action |
|---|---|
| "agkit docs" | Scan và báo cáo docs nào đang thiếu/outdated |
| "agkit docs readme" | Cập nhật README.md cho project |
| "agkit docs api" | Generate API documentation |
| "agkit docs [file]" | Generate JSDoc/GoDoc cho file đó |
| "agkit docs changelog" | Tạo CHANGELOG entry cho changes gần nhất |

### Bước 2 — README.md Template

Nếu chưa có hoặc cần update README, generate với structure:

```markdown
# [Project Name]

> [Mô tả 1 câu — từ PROJECT.md]

## Tech Stack
[Từ PROJECT.md — tech stack]

## Yêu cầu
- Node.js >= 20 / Go >= 1.22 / Python >= 3.11
- [Các yêu cầu khác]

## Quick Start
```bash
# Clone
git clone [url]

# Install
npm install  # hoặc go mod download / pip install -r requirements.txt

# Setup env
cp .env.example .env.local
# Điền các biến trong .env.local

# Dev server
npm run dev
```

## Project Structure
```
[Tự detect từ cấu trúc thực tế]
```

## API Endpoints
[Tự scan từ route files]

## Environment Variables
| Variable | Required | Description |
|---|---|---|
[Tự scan từ .env.example hoặc env validation file]

## Scripts
| Command | Description |
|---|---|
[Từ package.json scripts]

## Contributing
[Standard contribution guide]
```

### Bước 3 — JSDoc cho TypeScript

Scan tất cả functions/classes trong file target chưa có JSDoc:

```typescript
// Tự generate JSDoc từ code:

// ❌ Trước
async function getUserById(id: string): Promise<User | null> {
  return db.user.findUnique({ where: { id } })
}

// ✅ Sau — Generated JSDoc
/**
 * Tìm user theo ID.
 *
 * @param id - UUID của user cần tìm
 * @returns User nếu tìm thấy, null nếu không tồn tại
 * @throws {DatabaseError} Khi có lỗi kết nối database
 *
 * @example
 * const user = await getUserById('123e4567-e89b-12d3-a456-426614174000')
 * if (!user) throw new NotFoundError('User not found')
 */
async function getUserById(id: string): Promise<User | null> {
  return db.user.findUnique({ where: { id } })
}
```

### Bước 4 — Go Comments

```go
// ❌ Trước
func (s *UserService) GetByID(ctx context.Context, id string) (*User, error) {
    return s.repo.GetByID(ctx, id)
}

// ✅ Sau — Go doc format
// GetByID retrieves a user by their unique identifier.
// Returns ErrUserNotFound if the user does not exist.
// Returns a wrapped error with context if the database operation fails.
func (s *UserService) GetByID(ctx context.Context, id string) (*User, error) {
    return s.repo.GetByID(ctx, id)
}
```

### Bước 5 — CHANGELOG Entry

Khi user gọi "agkit docs changelog", đọc git log từ tag cuối:

```bash
git log [last-tag]..HEAD --pretty=format:"%s" --no-merges
```

Phân loại commits thành:
- **Added**: feat commits
- **Changed**: refactor, update commits
- **Fixed**: fix, hotfix commits
- **Security**: security commits

Generate theo Keep a Changelog format:
```markdown
## [Unreleased] — 2024-01-15

### Added
- [Feature mới từ feat: commits]

### Fixed
- [Bug fixes từ fix: commits]

### Changed
- [Thay đổi từ refactor: commits]
```

### Bước 6 — Kiểm tra docs outdated

Scan và báo cáo:
- README có mention features không còn tồn tại không?
- API endpoints trong docs có match với routes thực tế không?
- Environment variables trong docs có đầy đủ không?
- Có hàm public nào chưa có docstring không?

Báo cáo:
```
📚 Docs Health Check

✅ README.md — Up to date
⚠️  3 functions thiếu JSDoc (user.service.ts, auth.service.ts)
❌ API docs mention /api/v1/users — route này không còn tồn tại
✅ CHANGELOG — Updated

Gọi "agkit docs [file]" để fix từng issue.
```
