# docker.md — Docker & Container Rules
# Áp dụng khi project có Dockerfile hoặc docker-compose.yml.

---

## 1. Dockerfile — Multi-stage Build (Bắt buộc)

### Next.js Production
```dockerfile
# ✅ Multi-stage: builder → runner (image nhỏ ~200MB thay vì ~1GB)
FROM node:20.11-alpine AS base
WORKDIR /app

# Dependencies stage
FROM base AS deps
COPY package.json package-lock.json* ./
RUN npm ci

# Builder stage
FROM base AS builder
COPY --from=deps /app/node_modules ./node_modules
COPY . .
ENV NEXT_TELEMETRY_DISABLED 1
RUN npm run build

# Runner stage — chỉ copy những gì cần
FROM node:20.11-alpine AS runner
WORKDIR /app
ENV NODE_ENV production
ENV NEXT_TELEMETRY_DISABLED 1

# Non-root user
RUN addgroup --system --gid 1001 nodejs
RUN adduser  --system --uid 1001 nextjs

COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs
EXPOSE 3000
ENV PORT 3000
ENV HOSTNAME "0.0.0.0"

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget -qO- http://localhost:3000/api/health || exit 1

CMD ["node", "server.js"]
```

### Go Production
```dockerfile
FROM golang:1.22-alpine AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o server ./cmd/server

# Distroless/scratch — image siêu nhỏ
FROM gcr.io/distroless/static-debian12 AS runner
COPY --from=builder /app/server /server
EXPOSE 8080
USER nonroot:nonroot
CMD ["/server"]
```

---

## 2. `.dockerignore` — Bắt buộc

```
# .dockerignore
node_modules
.next
.git
*.log
.env
.env.local
.env.*.local
Dockerfile*
docker-compose*
README.md
.gitignore
coverage/
__tests__/
*.test.ts
*.spec.ts
```

---

## 3. `docker-compose.yml` — Development

```yaml
# docker-compose.yml (development)
version: '3.8'

services:
  app:
    build:
      context: .
      target: deps          # Chỉ build đến deps stage để dev nhanh
    ports:
      - "3000:3000"
    volumes:
      - .:/app              # Hot reload
      - /app/node_modules   # Exclude node_modules từ mount
    environment:
      - NODE_ENV=development
    env_file:
      - .env.local          # Load từ file, không hardcode
    depends_on:
      db:
        condition: service_healthy  # Chờ DB ready

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: ${POSTGRES_USER:-dev}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD:-devpassword}
      POSTGRES_DB: ${POSTGRES_DB:-myapp_dev}
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER:-dev}"]
      interval: 5s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:
```

---

## 4. Environment Variables Pattern

```bash
# .env.example — Commit file này (không có values thật)
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
NEXTAUTH_SECRET=your-secret-here-min-32-chars
NEXT_PUBLIC_APP_URL=http://localhost:3000

# .env.local — KHÔNG commit (có trong .gitignore)
DATABASE_URL=postgresql://dev:devpassword@localhost:5432/myapp_dev
NEXTAUTH_SECRET=abc123...actual-secret
```

---

## 5. Lệnh Docker Cần Nhớ

```bash
# Build image
docker build -t myapp:latest .

# Build với specific stage
docker build --target builder -t myapp:builder .

# Run container
docker run -p 3000:3000 --env-file .env.local myapp:latest

# Docker Compose
docker compose up -d          # Start (detached)
docker compose down           # Stop + remove containers
docker compose down -v        # Stop + remove containers + volumes
docker compose logs -f app    # Follow logs
docker compose exec app sh    # Shell vào container

# Xem image size
docker images myapp

# Dọn dẹp
docker system prune -af       # Xóa tất cả unused images/containers
```

---

## 6. Health Check Endpoint

```typescript
// app/api/health/route.ts — Required cho Docker healthcheck
import { NextResponse } from 'next/server'

export async function GET() {
  return NextResponse.json({
    status: 'ok',
    timestamp: new Date().toISOString(),
    version: process.env.npm_package_version,
  })
}
```
