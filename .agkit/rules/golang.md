# golang.md — Go Rules
# Áp dụng khi project có go.mod.

---

## 1. Error Handling (Quan trọng nhất trong Go)

### Nguyên tắc cốt lõi:
```go
// ❌ Sai: bỏ qua lỗi
result, _ := doSomething()

// ❌ Sai: lỗi không có context
return fmt.Errorf("failed")

// ✅ Đúng: wrap lỗi với context đầy đủ
result, err := doSomething()
if err != nil {
    return fmt.Errorf("userService.GetByID(%s): %w", userID, err)
}
```

### Sentinel errors và custom types:
```go
// Định nghĩa sentinel errors ở package level
var (
    ErrUserNotFound    = errors.New("user not found")
    ErrEmailDuplicate  = errors.New("email already exists")
    ErrUnauthorized    = errors.New("unauthorized")
)

// Kiểm tra lỗi với errors.Is (hỗ trợ wrapping)
if errors.Is(err, ErrUserNotFound) {
    return http.StatusNotFound
}

// Custom error type khi cần thêm data
type ValidationError struct {
    Field   string
    Message string
}
func (e *ValidationError) Error() string {
    return fmt.Sprintf("validation failed for %s: %s", e.Field, e.Message)
}
```

---

## 2. Package Structure (Domain-Driven)

```
/
├── cmd/
│   └── server/
│       └── main.go          ← Entry point, wire dependencies
├── internal/                ← Code không export ra ngoài module
│   ├── user/
│   │   ├── handler.go       ← HTTP handlers
│   │   ├── service.go       ← Business logic
│   │   ├── repository.go    ← DB interface + implementation
│   │   └── model.go         ← Domain types
│   ├── auth/
│   │   ├── middleware.go
│   │   └── service.go
│   └── platform/
│       ├── database/
│       │   └── postgres.go  ← DB connection
│       └── server/
│           └── server.go    ← HTTP server setup
├── pkg/                     ← Code có thể export (reusable)
│   ├── validator/
│   └── pagination/
└── go.mod
```

### Quy tắc package:
- ❌ Không tạo circular imports (Go sẽ compile error nhưng cần tránh thiết kế sai)
- ✅ `internal/` cho business logic — không thể import từ ngoài module
- ✅ Mỗi package có một responsibility rõ ràng
- ✅ Package name = thư mục name (short, lowercase, no underscore)

---

## 3. Interface Design

```go
// ✅ Interfaces nhỏ, định nghĩa ở phía CONSUMER (nơi dùng)
// Không phải phía implementer

// Trong user/service.go
type UserRepository interface {
    GetByID(ctx context.Context, id string) (*User, error)
    Create(ctx context.Context, user *User) error
    Update(ctx context.Context, user *User) error
}

// Trong user/repository.go — implement interface này
type postgresUserRepository struct {
    db *sql.DB
}

func (r *postgresUserRepository) GetByID(ctx context.Context, id string) (*User, error) {
    // ...
}
```

---

## 4. Context Propagation

```go
// ✅ Context luôn là tham số đầu tiên
func (s *UserService) GetByID(ctx context.Context, id string) (*User, error)

// ✅ Truyền context xuống tất cả calls
func (s *UserService) GetByID(ctx context.Context, id string) (*User, error) {
    user, err := s.repo.GetByID(ctx, id)  // truyền ctx
    if err != nil {
        return nil, fmt.Errorf("GetByID(%s): %w", id, err)
    }
    return user, nil
}

// ✅ Timeout cho external calls
ctx, cancel := context.WithTimeout(ctx, 5*time.Second)
defer cancel()
result, err := externalAPI.Call(ctx, params)
```

---

## 5. Concurrency Patterns

```go
// ✅ Dùng errgroup cho concurrent operations
import "golang.org/x/sync/errgroup"

g, ctx := errgroup.WithContext(ctx)

g.Go(func() error {
    users, err = s.userRepo.GetAll(ctx)
    return err
})
g.Go(func() error {
    stats, err = s.statsRepo.GetAll(ctx)
    return err
})

if err := g.Wait(); err != nil {
    return fmt.Errorf("concurrent fetch: %w", err)
}

// ✅ Channel với select và timeout
select {
case result := <-resultCh:
    return result, nil
case err := <-errCh:
    return nil, err
case <-ctx.Done():
    return nil, ctx.Err()
}
```

---

## 6. HTTP Handler Pattern

```go
// ✅ Handler chỉ làm: parse input → call service → format output
func (h *UserHandler) GetUser(w http.ResponseWriter, r *http.Request) {
    id := chi.URLParam(r, "id")  // hoặc mux.Vars(r)

    user, err := h.service.GetByID(r.Context(), id)
    if err != nil {
        if errors.Is(err, ErrUserNotFound) {
            h.respondError(w, http.StatusNotFound, "user not found")
            return
        }
        h.respondError(w, http.StatusInternalServerError, "internal error")
        return
    }

    h.respondJSON(w, http.StatusOK, user)
}
```

---

## 7. Testing (Table-Driven)

```go
func TestUserService_GetByID(t *testing.T) {
    tests := []struct {
        name    string
        userID  string
        mockFn  func(*MockUserRepository)
        want    *User
        wantErr error
    }{
        {
            name:   "returns user when found",
            userID: "user-123",
            mockFn: func(m *MockUserRepository) {
                m.On("GetByID", mock.Anything, "user-123").
                    Return(&User{ID: "user-123", Name: "Alice"}, nil)
            },
            want: &User{ID: "user-123", Name: "Alice"},
        },
        {
            name:   "returns ErrNotFound when user missing",
            userID: "nonexistent",
            mockFn: func(m *MockUserRepository) {
                m.On("GetByID", mock.Anything, "nonexistent").
                    Return(nil, ErrUserNotFound)
            },
            wantErr: ErrUserNotFound,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            mockRepo := new(MockUserRepository)
            tt.mockFn(mockRepo)
            svc := NewUserService(mockRepo)

            got, err := svc.GetByID(context.Background(), tt.userID)
            assert.ErrorIs(t, err, tt.wantErr)
            assert.Equal(t, tt.want, got)
        })
    }
}
```
