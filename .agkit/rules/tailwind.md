# tailwind.md — Tailwind CSS Rules
# Áp dụng khi project dùng Tailwind CSS.

---

## 1. Utility Function `cn()` — Bắt buộc

```typescript
// lib/utils.ts — Cài đặt một lần, dùng mọi nơi
import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// ✅ Dùng cn() cho conditional classes
<div className={cn(
  'flex items-center rounded-lg px-4 py-2',
  isActive && 'bg-blue-600 text-white',
  isDisabled && 'opacity-50 cursor-not-allowed',
  size === 'lg' ? 'text-lg h-12' : 'text-sm h-9'
)} />

// ❌ Không dùng string concatenation
<div className={`flex ${isActive ? 'bg-blue-600' : 'bg-gray-100'}`} />
```

---

## 2. Component Variants với `cva`

```typescript
// Dùng class-variance-authority cho components có nhiều variants
import { cva, type VariantProps } from 'class-variance-authority'

const buttonVariants = cva(
  // Base classes (luôn apply)
  'inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50',
  {
    variants: {
      variant: {
        default:  'bg-blue-600 text-white hover:bg-blue-700',
        outline:  'border border-gray-300 bg-transparent hover:bg-gray-50',
        ghost:    'hover:bg-gray-100',
        danger:   'bg-red-600 text-white hover:bg-red-700',
      },
      size: {
        sm:  'h-8 px-3 text-xs',
        md:  'h-9 px-4 text-sm',
        lg:  'h-11 px-6 text-base',
        icon: 'h-9 w-9',
      },
    },
    defaultVariants: { variant: 'default', size: 'md' },
  }
)

interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {}

export function Button({ variant, size, className, ...props }: ButtonProps) {
  return (
    <button className={cn(buttonVariants({ variant, size }), className)} {...props} />
  )
}
```

---

## 3. Design Tokens trong `tailwind.config.ts`

```typescript
// tailwind.config.ts
import type { Config } from 'tailwindcss'

const config: Config = {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        // Brand colors — dùng CSS variables để support dark mode
        brand: {
          50:  'hsl(var(--brand-50) / <alpha-value>)',
          500: 'hsl(var(--brand-500) / <alpha-value>)',
          900: 'hsl(var(--brand-900) / <alpha-value>)',
        },
      },
      fontFamily: {
        sans: ['var(--font-inter)', 'system-ui', 'sans-serif'],
        mono: ['var(--font-jetbrains)', 'monospace'],
      },
      borderRadius: {
        DEFAULT: '0.5rem',
      },
      keyframes: {
        'fade-in': {
          from: { opacity: '0', transform: 'translateY(4px)' },
          to:   { opacity: '1', transform: 'translateY(0)' },
        },
      },
      animation: {
        'fade-in': 'fade-in 0.2s ease-out',
      },
    },
  },
  plugins: [require('@tailwindcss/typography')],
}

export default config
```

```css
/* globals.css — CSS variables cho design tokens */
:root {
  --brand-50:  210 100% 97%;
  --brand-500: 210 100% 56%;
  --brand-900: 210 100% 15%;
  --radius: 0.5rem;
}

.dark {
  --brand-50:  210 30% 10%;
  --brand-500: 210 100% 60%;
  --brand-900: 210 100% 90%;
}
```

---

## 4. Dark Mode Pattern

```typescript
// ✅ Dùng class strategy (không media)
// tailwind.config.ts: darkMode: 'class'

// ✅ Dark mode classes ngay trong component
<div className="bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100">

// ✅ Toggle dark mode
'use client'
import { useTheme } from 'next-themes'

function ThemeToggle() {
  const { theme, setTheme } = useTheme()
  return (
    <button onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}>
      Toggle
    </button>
  )
}
```

---

## 5. Responsive — Mobile First

```typescript
// ✅ Luôn viết mobile styles trước, sau đó breakpoint lớn hơn
<div className="
  flex flex-col gap-4          /* mobile */
  sm:flex-row sm:gap-6         /* ≥640px */
  lg:gap-8                     /* ≥1024px */
">

// ❌ Sai: Desktop first
<div className="flex-row lg:flex-col">  {/* logic ngược */}

// Breakpoints chuẩn
// sm: 640px  — tablet nhỏ
// md: 768px  — tablet
// lg: 1024px — desktop
// xl: 1280px — desktop lớn
// 2xl: 1536px — wide screen
```

---

## 6. Animation & Transitions

```typescript
// ✅ Dùng Tailwind built-in transitions
<button className="transition-all duration-200 ease-in-out hover:scale-105 active:scale-95">

// ✅ Custom animation với keyframes trong config
<div className="animate-fade-in">

// ✅ Respect prefers-reduced-motion
// Tailwind tự handle với motion-safe: và motion-reduce:
<div className="transition-transform motion-reduce:transition-none">
```

---

## 7. Anti-patterns Cần Tránh

```typescript
// ❌ Inline styles khi Tailwind có thể handle
style={{ marginTop: '16px' }}  // Dùng mt-4 thay

// ❌ Arbitrary values quá nhiều
className="w-[347px] mt-[23px]"  // Magic numbers → dùng design tokens

// ❌ Quá nhiều classes trên 1 element (> 15 classes)
// → Extract thành component hoặc dùng cva()

// ❌ Không có focus styles
<button className="bg-blue-600">  // Missing focus-visible:ring-2...

// ✅ Focus styles bắt buộc cho interactive elements
<button className="bg-blue-600 focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 focus-visible:outline-none">
```
