# python.md — Python Rules
# Áp dụng khi project có requirements.txt hoặc pyproject.toml.

---

## 1. Type Hints (Bắt buộc — strict mypy)

```python
# ❌ Sai: không có type hints
def get_user(user_id):
    return db.query(user_id)

# ✅ Đúng: type hints đầy đủ
from typing import Optional
from uuid import UUID

async def get_user(user_id: UUID) -> Optional[UserModel]:
    return await db.query(UserModel, user_id)
```

### mypy config (pyproject.toml):
```toml
[tool.mypy]
strict = true
warn_return_any = true
warn_unused_ignores = true
disallow_untyped_defs = true
```

### Dùng khi nào:
- `Optional[T]` = `T | None` — khi giá trị có thể None
- `Union[A, B]` = `A | B` — khi có thể nhiều type
- `TypeVar` — khi viết generic functions
- `Protocol` — duck typing type-safe (thay vì ABC)

---

## 2. FastAPI Patterns

### Request / Response schemas với Pydantic v2:
```python
from pydantic import BaseModel, EmailStr, Field
from uuid import UUID
from datetime import datetime

class CreateUserRequest(BaseModel):
    name: str = Field(min_length=1, max_length=100)
    email: EmailStr
    role: Literal["admin", "user"] = "user"

class UserResponse(BaseModel):
    id: UUID
    name: str
    email: EmailStr
    created_at: datetime

    model_config = ConfigDict(from_attributes=True)  # ORM mode
```

### Router organization:
```python
# routers/users.py
from fastapi import APIRouter, Depends, HTTPException, status

router = APIRouter(prefix="/users", tags=["users"])

@router.get("/{user_id}", response_model=UserResponse)
async def get_user(
    user_id: UUID,
    service: UserService = Depends(get_user_service),  # DI
) -> UserResponse:
    user = await service.get_by_id(user_id)
    if user is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND)
    return UserResponse.model_validate(user)
```

---

## 3. Dependency Injection Pattern

```python
# dependencies.py
from functools import lru_cache
from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine

@lru_cache
def get_settings() -> Settings:
    return Settings()

async def get_db_session() -> AsyncGenerator[AsyncSession, None]:
    async with SessionLocal() as session:
        try:
            yield session
            await session.commit()
        except Exception:
            await session.rollback()
            raise

def get_user_repository(
    db: AsyncSession = Depends(get_db_session),
) -> UserRepository:
    return PostgresUserRepository(db)

def get_user_service(
    repo: UserRepository = Depends(get_user_repository),
) -> UserService:
    return UserService(repo)
```

---

## 4. Error Handling — Custom Exception Hierarchy

```python
# exceptions.py
class AppError(Exception):
    """Base exception cho toàn bộ application."""
    def __init__(self, message: str, code: str | None = None):
        super().__init__(message)
        self.code = code

class NotFoundError(AppError):
    """Resource không tồn tại."""

class ValidationError(AppError):
    """Input không hợp lệ."""

class UnauthorizedError(AppError):
    """Không có quyền truy cập."""

# Trong service layer:
async def get_user(user_id: UUID) -> UserModel:
    user = await self.repo.get_by_id(user_id)
    if user is None:
        raise NotFoundError(f"User {user_id} not found", code="USER_NOT_FOUND")
    return user

# Trong FastAPI exception handler:
@app.exception_handler(NotFoundError)
async def not_found_handler(request: Request, exc: NotFoundError):
    return JSONResponse(status_code=404, content={"error": str(exc), "code": exc.code})
```

---

## 5. Async Best Practices

```python
# ✅ Dùng async/await đúng — không blocking trong async context
async def process_users(user_ids: list[UUID]) -> list[UserModel]:
    # ✅ Concurrent, không sequential
    tasks = [get_user(uid) for uid in user_ids]
    return await asyncio.gather(*tasks)

# ❌ Sai: blocking call trong async function
async def bad_example():
    time.sleep(5)           # BLOCKS event loop
    result = requests.get() # BLOCKS — dùng httpx.AsyncClient thay thế

# ✅ Đúng: async HTTP client
async with httpx.AsyncClient() as client:
    response = await client.get(url, timeout=10.0)
```

---

## 6. File Structure

```
app/
├── main.py                  ← FastAPI app factory
├── config.py                ← Settings (pydantic-settings)
├── dependencies.py          ← DI providers
├── exceptions.py            ← Custom exceptions
│
├── routers/                 ← HTTP layer
│   ├── users.py
│   └── auth.py
│
├── services/                ← Business logic
│   ├── user_service.py
│   └── auth_service.py
│
├── repositories/            ← Data access layer
│   ├── base.py              ← Abstract repository
│   └── user_repository.py
│
├── models/                  ← SQLAlchemy ORM models
│   └── user.py
│
├── schemas/                 ← Pydantic schemas (request/response)
│   └── user.py
│
└── tests/
    ├── conftest.py          ← Fixtures
    ├── test_users.py
    └── test_auth.py
```

---

## 7. Testing với pytest

```python
# tests/conftest.py
import pytest
import pytest_asyncio
from httpx import AsyncClient, ASGITransport

@pytest_asyncio.fixture
async def client(app) -> AsyncGenerator[AsyncClient, None]:
    async with AsyncClient(
        transport=ASGITransport(app=app),
        base_url="http://test"
    ) as c:
        yield c

# tests/test_users.py
@pytest.mark.asyncio
async def test_get_user_returns_404_when_not_found(client: AsyncClient):
    response = await client.get("/users/nonexistent-id")
    assert response.status_code == 404
    assert response.json()["code"] == "USER_NOT_FOUND"
```

---

## 8. ruff Config (Linting)

```toml
# pyproject.toml
[tool.ruff]
line-length = 100
select = ["E", "F", "I", "N", "UP", "ANN", "S", "B", "A"]
ignore = ["ANN101", "ANN102"]

[tool.ruff.per-file-ignores]
"tests/*" = ["S101"]  # Allow assert in tests
```
