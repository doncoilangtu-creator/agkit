# common.md — Universal Rules (Áp dụng cho mọi tech stack)
# Antigravity phải follow TẤT CẢ các rule này, không có ngoại lệ.

---

## 1. SOLID Principles

### Single Responsibility
- Mỗi file/module chỉ có **một lý do để thay đổi**
- ❌ Sai: `UserService` vừa query DB, vừa gửi email, vừa format response
- ✅ Đúng: Tách thành `UserRepository`, `EmailService`, `UserSerializer`

### Open/Closed
- Mở rộng bằng **thêm code mới**, không sửa code cũ
- ❌ Sai: Thêm `if (type === 'new_type')` vào function cũ
- ✅ Đúng: Dùng strategy pattern / polymorphism / plugin system

### Liskov Substitution
- Subtype phải thay thế được supertype mà không phá vỡ behavior
- Áp dụng khi dùng interface/abstract class

### Interface Segregation
- Interface nhỏ, chuyên biệt — không tạo "god interface"
- ❌ Sai: Interface có 15 method, implementer chỉ cần 3
- ✅ Đúng: Tách thành nhiều interface nhỏ, compose khi cần

### Dependency Inversion
- Depend vào abstraction, không depend vào concrete implementation
- ❌ Sai: `new PostgresDatabase()` hardcoded trong service
- ✅ Đúng: Inject `Database` interface, truyền PostgresDatabase từ ngoài

---

## 2. Naming Conventions

### Files
- **TypeScript/JS:** `kebab-case.ts` (ví dụ: `user-repository.ts`, `auth-middleware.ts`)
- **Go:** `snake_case.go` (ví dụ: `user_repository.go`, `auth_middleware.go`)
- **Python:** `snake_case.py` (ví dụ: `user_repository.py`, `auth_middleware.py`)
- **React Components:** `PascalCase.tsx` (ví dụ: `UserCard.tsx`, `AuthProvider.tsx`)

### Functions / Methods
- **Động từ + Danh từ**: `getUserById()`, `validateEmail()`, `parseConfig()`
- Boolean functions: `isAuthenticated()`, `hasPermission()`, `canEdit()`
- Event handlers: `handleSubmit()`, `onUserClick()`
- **Không dùng**: `doStuff()`, `processData()`, `handle()` (quá mơ hồ)

### Variables
- Mô tả rõ nội dung: `userList`, `errorMessage`, `isLoading`
- Không viết tắt khó hiểu: `usr`, `msg`, `e` (trừ `i` trong loop ngắn)
- Constants: `SCREAMING_SNAKE_CASE` — `MAX_RETRY_COUNT`, `API_BASE_URL`

### Types / Interfaces
- **TypeScript:** PascalCase — `UserProfile`, `ApiResponse<T>`, `AuthState`
- **Go:** PascalCase (exported) — `UserRepository`, `AuthConfig`

---

## 3. Commit Message Format (Conventional Commits)

```
type(scope): subject

[optional body]

[optional footer]
```

**Types:**
- `feat`: Tính năng mới
- `fix`: Sửa bug
- `refactor`: Tái cấu trúc không thêm tính năng, không sửa bug
- `test`: Thêm/sửa test
- `docs`: Chỉ thay đổi tài liệu
- `chore`: Build, config, tooling
- `perf`: Cải thiện performance
- `ci`: CI/CD changes

**Ví dụ tốt:**
```
feat(auth): add JWT refresh token rotation
fix(api): handle null user in profile endpoint
refactor(user): extract email validation to shared utility
```

**Ví dụ sai:**
```
fix bug
update code
WIP
```

---

## 4. Error Handling Philosophy

### Nguyên tắc
- **Explicit errors** — không để lỗi im lặng (silent failure)
- **Fail fast** — phát hiện lỗi càng sớm càng tốt
- **Context đầy đủ** — error message phải nói rõ CÁI GÌ sai và Ở ĐÂU

### Anti-patterns cần tránh
```typescript
// ❌ Nuốt lỗi
try {
  doSomething();
} catch (e) {
  // nothing
}

// ❌ Generic error không có context
throw new Error('Something went wrong');

// ❌ Log rồi throw lại — double logging
catch (e) {
  console.error(e);
  throw e;
}
```

```typescript
// ✅ Đúng: wrap với context
catch (e) {
  throw new Error(`Failed to fetch user ${userId}: ${e.message}`);
}

// ✅ Đúng: Log ở boundary (top level), không ở deep trong stack
```

---

## 5. Testing Requirements

### Viết test cho:
- Business logic (pure functions, domain rules)
- API endpoints (integration test với test database)
- Edge cases: null, empty, max values, concurrent requests

### Không cần test:
- Framework boilerplate (Next.js routing, ORM config)
- Trivial getters/setters không có logic

### Coverage minimum:
- Business logic: 80%+
- Critical paths (auth, payment): 95%+

### Test naming:
```
describe('UserService', () => {
  it('should return null when user not found', ...)
  it('should throw when email already exists', ...)
  it('should hash password before saving', ...)
})
```

---

## 6. Code Review Checklist (Tự check trước khi báo done)

- [ ] Có test cho logic mới không?
- [ ] Naming có rõ ràng không?
- [ ] Có hardcoded value không? (strings, numbers magic)
- [ ] Có xử lý error đúng không?
- [ ] Có SQL injection / XSS / auth bypass potential không?
- [ ] Có thể đọc hiểu code sau 1 tháng không cần giải thích không?
