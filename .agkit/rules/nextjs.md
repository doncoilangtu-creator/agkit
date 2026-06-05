# nextjs.md — Next.js 14+ App Router Rules (TypeScript)
# Áp dụng khi project có package.json với next dependency.

---

## 1. App Router — Phân loại Component

### Server Components (mặc định)
Dùng khi: fetch data, access DB, render static/dynamic HTML, không cần interactivity.
```typescript
// app/users/page.tsx — Server Component
async function UsersPage() {
  const users = await getUsersFromDB(); // Fetch trực tiếp, không useEffect
  return <UserList users={users} />;
}
```

### Client Components (`'use client'`)
Dùng khi: cần useState, useEffect, event handlers, browser APIs, interactivity.
```typescript
'use client';
// Đặt 'use client' ở component nhỏ nhất có thể
// Không đặt ở layout.tsx hoặc page.tsx trừ khi cả page cần interactivity
function LikeButton({ postId }: { postId: string }) {
  const [liked, setLiked] = useState(false);
  return <button onClick={() => setLiked(!liked)}>...</button>;
}
```

### Quy tắc phân chia:
- ❌ Không dùng `useEffect` để fetch data (dùng Server Component)
- ❌ Không đặt `'use client'` ở root layout
- ✅ Pass Server Component data xuống Client Component qua props
- ✅ Giữ Client Components ở "leaf" của component tree

---

## 2. Data Fetching

### Server Actions (cho mutations)
```typescript
// app/actions/user.ts
'use server';

import { revalidatePath } from 'next/cache';
import { z } from 'zod';

const UpdateUserSchema = z.object({
  name: z.string().min(1).max(100),
  email: z.string().email(),
});

export async function updateUser(userId: string, formData: FormData) {
  const parsed = UpdateUserSchema.safeParse({
    name: formData.get('name'),
    email: formData.get('email'),
  });

  if (!parsed.success) {
    return { error: parsed.error.flatten() };
  }

  await db.user.update({ where: { id: userId }, data: parsed.data });
  revalidatePath('/users');
  return { success: true };
}
```

### Route Handlers (cho API endpoints public)
```typescript
// app/api/users/route.ts
import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams;
  const page = Number(searchParams.get('page') ?? '1');

  const users = await getUsersPaginated(page);
  return NextResponse.json(users);
}
```

### Quy tắc:
- ❌ Không dùng `getServerSideProps` (Pages Router — deprecated)
- ✅ Dùng Server Actions cho form submissions và mutations
- ✅ Dùng Route Handlers cho external API access hoặc webhooks
- ✅ Dùng TanStack Query ở client nếu cần caching + optimistic updates

---

## 3. File Structure (Feature-based)

```
app/
  (auth)/                    ← Route group: không ảnh hưởng URL
    login/
      page.tsx
      _components/           ← _ prefix: folder private, không phải route
        LoginForm.tsx
    layout.tsx
  (dashboard)/
    users/
      [id]/
        page.tsx
      page.tsx
    layout.tsx
  api/
    webhooks/
      stripe/
        route.ts
  layout.tsx
  globals.css

features/                    ← Business logic, không phụ thuộc Next.js
  users/
    user.repository.ts
    user.service.ts
    user.types.ts
    user.schema.ts           ← Zod schemas
  auth/
    auth.service.ts
    auth.config.ts

lib/                         ← Shared utilities
  db.ts                      ← Database client singleton
  validations.ts

components/                  ← Shared UI components
  ui/
    Button.tsx
    Input.tsx
  layouts/
    Navbar.tsx
```

---

## 4. TypeScript Strictness

### tsconfig.json tối thiểu:
```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "noUncheckedIndexedAccess": true
  }
}
```

### Rules bắt buộc:
```typescript
// ❌ Không bao giờ dùng any
const data: any = fetchUser(); // WRONG

// ✅ Dùng unknown nếu không biết type, sau đó narrow
const data: unknown = fetchUser();
if (isUser(data)) { /* safe */ }

// ❌ Không dùng as để ép kiểu trừ khi THỰC SỰ không có cách khác
const user = response as User; // WRONG (unsafe cast)

// ✅ Dùng Zod để parse và validate
const user = UserSchema.parse(response); // Type-safe
```

---

## 5. State Management

### Phân tầng state:
```
Server State (DB data)     → TanStack Query hoặc Server Components
URL State (filters, pages) → useSearchParams / nuqs
Global UI State            → Zustand
Local Component State      → useState
Form State                 → React Hook Form + Zod
```

### Zustand store pattern:
```typescript
// store/user.store.ts
import { create } from 'zustand';

interface UserState {
  selectedUserId: string | null;
  setSelectedUser: (id: string | null) => void;
}

export const useUserStore = create<UserState>((set) => ({
  selectedUserId: null,
  setSelectedUser: (id) => set({ selectedUserId: id }),
}));
```

---

## 6. Environment Variables

```typescript
// lib/env.ts — Validate env vars at startup với Zod
import { z } from 'zod';

const envSchema = z.object({
  DATABASE_URL: z.string().url(),
  NEXTAUTH_SECRET: z.string().min(32),
  NEXT_PUBLIC_APP_URL: z.string().url(),
});

export const env = envSchema.parse(process.env);
// Dùng `env.DATABASE_URL` thay vì `process.env.DATABASE_URL`
```

---

## 7. Performance Rules

- ✅ Dùng `next/image` cho tất cả images (auto WebP, lazy load)
- ✅ Dùng `next/font` cho Google Fonts (no FOUT, built-in optimization)
- ✅ Dynamic import cho heavy components: `dynamic(() => import('./HeavyChart'))`
- ❌ Không import thư viện lớn ở Client Component nếu chỉ dùng 1 function
- ✅ `loading.tsx` cho Suspense boundaries ở route level
