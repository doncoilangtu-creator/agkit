# agkit-git — Git Workflow Helper
# Trigger: "agkit git", "commit này", "tạo commit message", "agkit git pr"

## Mô tả

Hỗ trợ git workflow: tạo commit messages chuẩn Conventional Commits,
tóm tắt changes cho PR description, kiểm tra branch strategy,
và gợi ý changelog entry.

---

## Các bước thực hiện

### Bước 1 — Xác định action

| User nói gì | Action |
|---|---|
| "agkit git" hoặc "agkit git commit" | Tạo commit message |
| "agkit git pr" | Tạo PR description |
| "agkit git changelog" | Gọi `agkit docs changelog` |
| "agkit git branch [feature]" | Suggest branch name |
| "agkit git cleanup" | List merged branches có thể xóa |

### Bước 2 — Commit Message (Conventional Commits)

**Đọc staged changes:**
```bash
git diff --cached --stat
git diff --cached
```

**Phân tích và generate message theo format:**
```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

**Types:**
| Type | Dùng khi |
|---|---|
| `feat` | Thêm feature mới |
| `fix` | Sửa bug |
| `refactor` | Restructure code, không thêm feature/fix bug |
| `perf` | Cải thiện performance |
| `test` | Thêm/sửa tests |
| `docs` | Cập nhật docs |
| `style` | Format, whitespace (không ảnh hưởng logic) |
| `chore` | Build, deps, CI |
| `security` | Fix security issue |
| `revert` | Revert commit trước |

**Examples:**
```bash
# Feature mới
feat(auth): add Google OAuth login

# Bug fix với context
fix(posts): prevent duplicate tag creation on concurrent requests

Concurrent requests to POST /posts could create duplicate tags
when the same tag name was submitted simultaneously.
Fixed by adding unique constraint check before insert.

# Breaking change
feat(api)!: rename /users to /accounts

BREAKING CHANGE: The /users endpoint has been renamed to /accounts.
Update all client API calls accordingly.

# Scope từ folder structure
fix(ui/button): correct disabled state opacity
refactor(services/user): extract email validation to shared util
```

**Sau khi generate, hỏi user:**
> "Commit message: `[message]`
> Bạn muốn dùng message này không? (yes / chỉnh sửa)"

### Bước 3 — PR Description Template

```markdown
## Summary
[1-2 câu mô tả PR làm gì]

## Changes
- [Thay đổi chính 1]
- [Thay đổi chính 2]

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing: [mô tả test thủ công]

## Screenshots (nếu có UI changes)
[Paste screenshots]

## Checklist
- [ ] Code follows project conventions (.agkit/rules/)
- [ ] Tests pass (agkit verify)
- [ ] Security checked (agkit security)
- [ ] No debug code / console.log
- [ ] Docs updated if needed
```

### Bước 4 — Branch Naming Convention

```bash
# Format: <type>/<ticket-or-description>
feat/user-authentication
fix/post-duplicate-tags
refactor/extract-validation-utils
perf/optimize-dashboard-queries
hotfix/payment-timeout

# Không dùng:
my-branch
test123
fix          # Quá chung chung
```

### Bước 5 — Git Hygiene

```bash
# List branches đã merge (safe to delete)
git branch --merged main | grep -v "main\|master\|develop"

# Xóa local branches đã merge
git branch --merged main | grep -v "main\|master" | xargs git branch -d

# Kiểm tra có unstaged changes không
git status --porcelain

# Stash changes tạm thời
git stash push -m "WIP: [mô tả]"
git stash pop
```
