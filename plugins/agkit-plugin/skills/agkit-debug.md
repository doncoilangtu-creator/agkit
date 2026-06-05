# agkit-debug — Systematic Debugging Workflow
# Trigger: "agkit debug", "bug này", "không hiểu lỗi gì", "agkit debug [error message]"

## Mô tả

Hướng dẫn debug có hệ thống: reproduce → isolate → hypothesize → test → fix.
Không đoán mò, không patch blindly. Mỗi bước phải có evidence.

---

## Các bước thực hiện

### Bước 1 — Thu thập thông tin đủ trước khi diagnose

Checklist thông tin cần có:
- [ ] Error message đầy đủ (không chỉ dòng cuối)
- [ ] Stack trace (nếu có)
- [ ] Bước reproduce (bao nhiêu bước để trigger bug?)
- [ ] Bug xảy ra mọi lúc hay intermittent?
- [ ] Lần cuối code hoạt động đúng là khi nào?
- [ ] Có thay đổi gì gần đây không? (new deploy, dependency update?)

Nếu thiếu thông tin → Hỏi user trước khi debug.

### Bước 2 — Classify Bug

| Loại bug | Dấu hiệu | Hướng debug |
|---|---|---|
| **Type Error** | `Cannot read property of undefined` | Check data shape, null guard |
| **Logic Error** | Sai kết quả, không crash | Add logging, trace data flow |
| **Race Condition** | Intermittent, hard to reproduce | Check async order, concurrent state |
| **State Bug** | Works on refresh, breaks after navigation | Check state initialization, cleanup |
| **Network Bug** | Timeout, CORS, 4xx/5xx | Check request/response, headers |
| **Build Bug** | Works in dev, breaks in production | Check env vars, bundle differences |
| **Memory Leak** | Slows down over time | Check cleanup, event listeners |

### Bước 3 — Reproduce Trước (Critical)

```
Nguyên tắc: Nếu chưa reproduce được bug → chưa debug.

Tạo minimal reproduction:
1. Bắt đầu từ case đơn giản nhất có thể trigger bug
2. Loại bỏ từng phần cho đến khi không reproduce được
3. Phần vừa loại bỏ đó chứa bug

Ví dụ: Bug xảy ra khi "user upload avatar sau khi đổi email"
→ Thử: Chỉ upload avatar (không đổi email) → OK?
→ Thử: Chỉ đổi email (không upload) → OK?
→ Thử: Đổi email TRƯỚC rồi upload → Bug? → Isolated!
```

### Bước 4 — Add Strategic Logging

```typescript
// ✅ Log với context, không phải chỉ value
console.log('[UserService.getById] Input:', { userId, caller: 'ProfilePage' })
console.log('[UserService.getById] DB result:', { found: !!user, userId })

// ✅ Log async flow order
console.log('[Auth] 1. Session check start')
const session = await getSession()
console.log('[Auth] 2. Session result:', { hasSession: !!session })

// ✅ Typescript: Kiểm tra type tại runtime
function assertType<T>(val: unknown, check: (v: unknown) => v is T): T {
  if (!check(val)) {
    console.error('Type assertion failed:', val)
    throw new Error(`Expected type but got: ${typeof val}`)
  }
  return val
}

// Go
log.Printf("[DEBUG] getUserByID: id=%s, found=%v, err=%v", id, user != nil, err)

// Python
logger.debug("get_user: id=%s, result=%s", user_id, "found" if user else "not_found")
```

### Bước 5 — Hypothesis → Test → Confirm

```
Quy trình:
1. Đặt hypothesis: "Tôi nghĩ bug xảy ra vì X"
2. Predict: "Nếu X đúng, thì khi tôi làm Y, tôi sẽ thấy Z"
3. Test prediction
4. Kết quả có khớp Z không?
   - Có → X có thể là nguyên nhân → fix X
   - Không → X không phải nguyên nhân → đặt hypothesis mới

Không fix cho đến khi có hypothesis được confirm.
```

### Bước 6 — Fix và Verify

```
Sau khi tìm ra root cause:
1. Fix root cause (không chỉ symptom)
2. Viết test reproduce bug này
3. Confirm test fail trước khi fix
4. Apply fix
5. Confirm test pass
6. Run full test suite (agkit verify)
7. Commit: "fix(scope): [mô tả bug và fix]"
```

### Bước 7 — Post-mortem ngắn (nếu bug nghiêm trọng)

Ghi vào STATUS.md → Session Notes:
```
[BUG] [Tên bug] — Root cause: [X]. Fix: [Y]. Prevent: [Z].
```

---

## Common Bug Patterns và Quick Fixes

```typescript
// ❌ Optional chaining bị quên
user.profile.avatar.url  // Crash nếu profile null
// ✅ Fix
user?.profile?.avatar?.url ?? '/default-avatar.png'

// ❌ Async/await quên await
const user = getUserById(id)  // user là Promise, không phải User
// ✅ Fix
const user = await getUserById(id)

// ❌ Mutation của state trực tiếp (React)
state.items.push(newItem)  // React không detect thay đổi
// ✅ Fix
setState(prev => ({ ...prev, items: [...prev.items, newItem] }))

// ❌ useEffect cleanup thiếu
useEffect(() => {
  const sub = subscribe(handler)
  // Missing cleanup!
}, [])
// ✅ Fix
useEffect(() => {
  const sub = subscribe(handler)
  return () => sub.unsubscribe()  // Cleanup
}, [])
```
