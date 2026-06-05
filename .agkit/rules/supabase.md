# supabase.md — Supabase & PostgreSQL Rules
# Áp dụng khi project dùng Supabase làm backend/database.

---

## 1. Row Level Security (RLS) — Bắt buộc 100%

```sql
-- ✅ Mọi table phải có RLS enabled
ALTER TABLE profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE posts ENABLE ROW LEVEL SECURITY;
ALTER TABLE comments ENABLE ROW LEVEL SECURITY;

-- ✅ Policy template chuẩn
-- SELECT: chỉ xem của mình
CREATE POLICY "users_select_own"
  ON profiles FOR SELECT
  USING (auth.uid() = id);

-- INSERT: chỉ insert với user_id của mình
CREATE POLICY "users_insert_own"
  ON posts FOR INSERT
  WITH CHECK (auth.uid() = user_id);

-- UPDATE: chỉ sửa của mình
CREATE POLICY "users_update_own"
  ON posts FOR UPDATE
  USING (auth.uid() = user_id)
  WITH CHECK (auth.uid() = user_id);

-- DELETE: chỉ xóa của mình
CREATE POLICY "users_delete_own"
  ON posts FOR DELETE
  USING (auth.uid() = user_id);
```

**Lưu ý quan trọng:**
- `USING` → filter rows khi SELECT/UPDATE/DELETE
- `WITH CHECK` → validate rows khi INSERT/UPDATE
- Service role key bypass RLS — KHÔNG expose ở client

---

## 2. Supabase Client Pattern

```typescript
// lib/supabase/server.ts — Server-side (Next.js Server Components)
import { createServerClient } from '@supabase/ssr'
import { cookies } from 'next/headers'

export function createClient() {
  const cookieStore = cookies()
  return createServerClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    {
      cookies: {
        get(name) { return cookieStore.get(name)?.value },
        set(name, value, options) { cookieStore.set({ name, value, ...options }) },
        remove(name, options) { cookieStore.set({ name, value: '', ...options }) },
      },
    }
  )
}

// lib/supabase/client.ts — Client-side
import { createBrowserClient } from '@supabase/ssr'
export const supabase = createBrowserClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL!,
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
)
```

---

## 3. Query Patterns

```typescript
// ✅ Select cụ thể columns, không SELECT *
const { data, error } = await supabase
  .from('posts')
  .select('id, title, created_at, user:profiles(name, avatar_url)')
  .eq('status', 'published')
  .order('created_at', { ascending: false })
  .range(0, 9) // Pagination

// ✅ Luôn handle error
if (error) throw new Error(`Failed to fetch posts: ${error.message}`)

// ✅ Type-safe với generated types
import type { Database } from '@/types/supabase'
type Post = Database['public']['Tables']['posts']['Row']

// ❌ Không dùng any
const { data } = await supabase.from('posts').select() // data: any — WRONG
```

---

## 4. Migrations Best Practices

```sql
-- ✅ Migration file naming: timestamp_description.sql
-- 20240115_add_tags_to_posts.sql

-- ✅ Luôn có rollback comment
-- UP
ALTER TABLE posts ADD COLUMN tags text[] DEFAULT '{}';
CREATE INDEX CONCURRENTLY idx_posts_tags ON posts USING GIN(tags);

-- DOWN (rollback)
-- DROP INDEX IF EXISTS idx_posts_tags;
-- ALTER TABLE posts DROP COLUMN IF EXISTS tags;

-- ✅ ADD COLUMN không nullable phải có DEFAULT
-- ❌ ALTER TABLE posts ADD COLUMN status text NOT NULL; -- Lỗi nếu có rows
-- ✅ ALTER TABLE posts ADD COLUMN status text NOT NULL DEFAULT 'draft';

-- ✅ CREATE INDEX với CONCURRENTLY để không lock table
CREATE INDEX CONCURRENTLY idx_posts_user_id ON posts(user_id);
```

---

## 5. Realtime Subscriptions

```typescript
// ✅ Cleanup subscription khi component unmount
useEffect(() => {
  const channel = supabase
    .channel('posts-changes')
    .on('postgres_changes',
      { event: 'INSERT', schema: 'public', table: 'posts' },
      (payload) => setNewPost(payload.new as Post)
    )
    .subscribe()

  return () => { supabase.removeChannel(channel) } // Cleanup
}, [])
```

---

## 6. Auth Patterns

```typescript
// ✅ Protected route với middleware
// middleware.ts
import { createMiddlewareClient } from '@supabase/auth-helpers-nextjs'
import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export async function middleware(req: NextRequest) {
  const res = NextResponse.next()
  const supabase = createMiddlewareClient({ req, res })
  const { data: { session } } = await supabase.auth.getSession()

  if (!session && req.nextUrl.pathname.startsWith('/dashboard')) {
    return NextResponse.redirect(new URL('/login', req.url))
  }
  return res
}

export const config = {
  matcher: ['/dashboard/:path*', '/api/protected/:path*']
}
```

---

## 7. Storage Rules

```typescript
// ✅ Bucket policy: users chỉ upload vào folder của mình
// Storage policy (trong Supabase dashboard):
// USING: (auth.uid()::text = (storage.foldername(name))[1])

// ✅ Upload pattern
const { data, error } = await supabase.storage
  .from('avatars')
  .upload(`${userId}/avatar.jpg`, file, {
    upsert: true,  // overwrite nếu đã tồn tại
    contentType: 'image/jpeg'
  })

// ✅ Get public URL
const { data: { publicUrl } } = supabase.storage
  .from('avatars')
  .getPublicUrl(`${userId}/avatar.jpg`)
```
