# agkit-perf — Performance Analysis & Optimization
# Trigger: "agkit perf", "tối ưu performance", "app chậm", "agkit perf [url/file]"

## Mô tả

Phân tích performance bottlenecks theo stack, đưa ra danh sách tối ưu có impact cao nhất
trước. Bao gồm: bundle size, database queries, Core Web Vitals, memory usage.

---

## Các bước thực hiện

### Bước 1 — Xác định scope

| User nói gì | Phân tích |
|---|---|
| "agkit perf" | Full audit — tất cả layers |
| "agkit perf frontend" | Bundle size, LCP, CLS, INP |
| "agkit perf database" | Slow queries, N+1, missing indexes |
| "agkit perf api" | Response time, memory, CPU |
| "agkit perf [file]" | File cụ thể |

### Bước 2 — Frontend Performance (Next.js)

**Bundle Analysis:**
```bash
# Phân tích bundle size
npm run build
# Xem .next/analyze/ nếu có @next/bundle-analyzer

# Tìm dependencies lớn
npx bundlephobia-cli [package-name]

# Tìm duplicate packages
npx duplicate-package-checker-webpack-plugin
```

**Kiểm tra tự động:**
- [ ] Images có dùng `next/image` không? (auto WebP, lazy load)
- [ ] Fonts có dùng `next/font` không? (no FOUT)
- [ ] Third-party scripts có `next/script` với strategy không?
- [ ] Dynamic imports cho heavy components?
- [ ] Unused imports làm tăng bundle?

**Core Web Vitals Targets:**
| Metric | Good | Needs Improvement | Poor |
|---|---|---|---|
| LCP | < 2.5s | 2.5–4s | > 4s |
| CLS | < 0.1 | 0.1–0.25 | > 0.25 |
| INP | < 200ms | 200–500ms | > 500ms |

**Quick Wins (Impact cao nhất):**
```typescript
// 1. Lazy load heavy components
const HeavyChart = dynamic(() => import('./HeavyChart'), {
  loading: () => <ChartSkeleton />,
  ssr: false
})

// 2. Optimize images
<Image
  src="/hero.jpg"
  width={1200} height={630}
  priority          // LCP image
  placeholder="blur" // Prevent layout shift
  blurDataURL="..."
/>

// 3. Preload critical data
// Trong layout.tsx — không fetch trong page
export async function generateMetadata() {
  const data = await getCriticalData() // Parallel với page render
}
```

### Bước 3 — Database Performance

**Scan Patterns:**
```typescript
// Tự detect N+1 trong code
// Tìm: query trong loop, await trong forEach, Promise.all thiếu
grep -rn "await.*\.forEach\|for.*await" src/

// Tìm SELECT * patterns
grep -rn "select(\*\|'*'\|\"*\")" src/
```

**Index Recommendations:**
```sql
-- Analyze query performance
EXPLAIN ANALYZE SELECT * FROM posts WHERE user_id = $1 ORDER BY created_at DESC;

-- Nếu thấy "Seq Scan" → cần index
-- Nếu thấy "Index Scan" → đã tốt

-- Tìm missing indexes (chạy trong Supabase SQL Editor)
SELECT schemaname, tablename, attname, n_distinct, correlation
FROM pg_stats
WHERE schemaname = 'public'
ORDER BY n_distinct DESC;
```

### Bước 4 — API Performance (Go/Python)

**Go:**
```bash
# CPU profiling
go test -cpuprofile=cpu.prof -bench=. ./...
go tool pprof cpu.prof

# Memory profiling
go test -memprofile=mem.prof -bench=. ./...
go tool pprof mem.prof

# Benchmark specific function
go test -bench=BenchmarkGetUser -benchmem -count=5 ./internal/user/
```

**Python FastAPI:**
```python
# Thêm timing middleware để tìm slow endpoints
import time
from fastapi import Request

@app.middleware("http")
async def add_process_time_header(request: Request, call_next):
    start_time = time.time()
    response = await call_next(request)
    process_time = time.time() - start_time
    response.headers["X-Process-Time"] = str(process_time)
    # Log slow requests
    if process_time > 0.5:
        logger.warning(f"SLOW: {request.url} took {process_time:.3f}s")
    return response
```

### Bước 5 — Report và Action Plan

```
⚡ Performance Report — [Date]

### 🔴 High Impact (Fix ngay)
1. [Issue] — Expected improvement: [X]ms / [X]%
   Fix: [Cụ thể]

### 🟡 Medium Impact
1. [Issue] — Expected improvement: [X]%

### 🔵 Low Impact / Nice to have
1. [Issue]

### Quick Wins (< 1 giờ để implement)
- [ ] [Fix 1]
- [ ] [Fix 2]

Estimated total improvement: [X]% faster / [X]MB smaller
```
