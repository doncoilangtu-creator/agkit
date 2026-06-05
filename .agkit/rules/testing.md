# testing.md — Testing Strategy Rules
# Áp dụng cho mọi stack: TypeScript, Go, Python.

---

## 1. Testing Pyramid — Phân bổ Đúng

```
         ╔══════╗
         ║  E2E ║  ← 10% — Playwright, ít nhưng cover critical flows
        ╔╩══════╩╗
        ║  Integ ║  ← 30% — API endpoints, DB queries, services
       ╔╩════════╩╗
       ║   Unit   ║  ← 60% — Business logic, pure functions, validators
      ╚══════════╝
```

**Nguyên tắc:** Unit test nhiều nhất, chạy nhanh nhất. E2E ít nhất, chạy chậm nhất.

---

## 2. Unit Testing

### TypeScript — Vitest (preferred) hoặc Jest

```typescript
// ✅ Test naming: "should [behavior] when [condition]"
// user.service.test.ts
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { UserService } from './user.service'
import type { UserRepository } from './user.repository'

describe('UserService', () => {
  let mockRepo: vi.Mocked<UserRepository>
  let service: UserService

  beforeEach(() => {
    mockRepo = {
      findById: vi.fn(),
      create: vi.fn(),
      update: vi.fn(),
    }
    service = new UserService(mockRepo)
  })

  describe('getById', () => {
    it('should return user when found', async () => {
      const user = { id: '1', email: 'test@example.com', name: 'Alice' }
      mockRepo.findById.mockResolvedValue(user)

      const result = await service.getById('1')

      expect(result).toEqual(user)
      expect(mockRepo.findById).toHaveBeenCalledWith('1')
    })

    it('should throw NotFoundError when user does not exist', async () => {
      mockRepo.findById.mockResolvedValue(null)

      await expect(service.getById('999')).rejects.toThrow('User 999 not found')
    })

    it('should throw when id is empty', async () => {
      await expect(service.getById('')).rejects.toThrow()
    })
  })
})
```

### Go — Table-driven Tests

```go
// user_service_test.go
func TestUserService_GetByID(t *testing.T) {
    tests := []struct {
        name      string
        userID    string
        setupMock func(*MockRepository)
        wantUser  *User
        wantErr   string
    }{
        {
            name:   "returns user when found",
            userID: "user-1",
            setupMock: func(m *MockRepository) {
                m.On("GetByID", mock.Anything, "user-1").
                    Return(&User{ID: "user-1", Email: "test@test.com"}, nil)
            },
            wantUser: &User{ID: "user-1", Email: "test@test.com"},
        },
        {
            name:   "returns error when not found",
            userID: "missing",
            setupMock: func(m *MockRepository) {
                m.On("GetByID", mock.Anything, "missing").
                    Return(nil, ErrNotFound)
            },
            wantErr: "not found",
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            repo := new(MockRepository)
            tt.setupMock(repo)
            svc := NewUserService(repo)

            got, err := svc.GetByID(context.Background(), tt.userID)

            if tt.wantErr != "" {
                assert.ErrorContains(t, err, tt.wantErr)
                return
            }
            assert.NoError(t, err)
            assert.Equal(t, tt.wantUser, got)
        })
    }
}
```

---

## 3. Integration Testing

### Next.js — API Route Testing

```typescript
// __tests__/api/users.test.ts
import { testApiHandler } from 'next-test-api-route-handler'
import * as usersRoute from '@/app/api/users/route'

describe('GET /api/users', () => {
  it('returns 401 when unauthenticated', async () => {
    await testApiHandler({
      appHandler: usersRoute,
      test: async ({ fetch }) => {
        const res = await fetch({ method: 'GET' })
        expect(res.status).toBe(401)
      },
    })
  })
})
```

### Python — pytest với TestClient

```python
# tests/test_users.py
import pytest
from httpx import AsyncClient

@pytest.mark.asyncio
async def test_get_user_returns_200(client: AsyncClient, test_user):
    response = await client.get(f"/users/{test_user.id}")
    assert response.status_code == 200
    assert response.json()["email"] == test_user.email

@pytest.mark.asyncio
async def test_get_user_returns_404_when_not_found(client: AsyncClient):
    response = await client.get("/users/nonexistent-id")
    assert response.status_code == 404
    assert response.json()["code"] == "USER_NOT_FOUND"

@pytest.mark.asyncio
async def test_create_user_validates_email(client: AsyncClient):
    response = await client.post("/users", json={"email": "not-an-email"})
    assert response.status_code == 422  # Validation error
```

---

## 4. E2E Testing — Playwright

```typescript
// e2e/auth.spec.ts
import { test, expect } from '@playwright/test'

test.describe('Authentication', () => {
  test('user can log in with valid credentials', async ({ page }) => {
    await page.goto('/login')

    await page.getByLabel('Email').fill('test@example.com')
    await page.getByLabel('Password').fill('password123')
    await page.getByRole('button', { name: 'Đăng nhập' }).click()

    await expect(page).toHaveURL('/dashboard')
    await expect(page.getByText('Xin chào')).toBeVisible()
  })

  test('shows error with invalid credentials', async ({ page }) => {
    await page.goto('/login')
    await page.getByLabel('Email').fill('wrong@example.com')
    await page.getByLabel('Password').fill('wrongpassword')
    await page.getByRole('button', { name: 'Đăng nhập' }).click()

    await expect(page.getByRole('alert')).toContainText('Email hoặc mật khẩu không đúng')
  })
})

// playwright.config.ts
import { defineConfig } from '@playwright/test'
export default defineConfig({
  testDir: './e2e',
  use: {
    baseURL: 'http://localhost:3000',
    screenshot: 'only-on-failure',
    trace: 'retain-on-failure',
  },
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
})
```

---

## 5. Test Coverage — Mục tiêu

| Layer | Minimum | Target |
|---|---|---|
| Business logic / Services | 80% | 90%+ |
| API endpoints | 70% | 85%+ |
| UI components | 50% | 70%+ |
| Critical paths (auth, payment) | 95% | 100% |
| Utilities/helpers | 90% | 100% |

---

## 6. Test Data & Fixtures

```typescript
// test/factories/user.factory.ts
import { faker } from '@faker-js/faker'

export function createUser(overrides = {}) {
  return {
    id: faker.string.uuid(),
    email: faker.internet.email(),
    name: faker.person.fullName(),
    createdAt: new Date(),
    ...overrides,
  }
}

// Dùng trong tests
const user = createUser({ email: 'fixed@test.com' })
const users = Array.from({ length: 5 }, () => createUser())
```
