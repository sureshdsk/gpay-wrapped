# finn-lens v2: Frontend Architecture

> **Purpose**: Complete frontend architecture specification using modern React ecosystem with TanStack, shadcn/ui, Zustand, and other best-in-class libraries.

---

## Technology Stack

### Core Framework
| Library | Version | Purpose |
|---------|---------|---------|
| React | 18.3+ | UI framework |
| TypeScript | 5.4+ | Type safety |
| Vite | 5.4+ | Build tool, dev server |

### Routing & Data Fetching
| Library | Version | Purpose |
|---------|---------|---------|
| TanStack Router | 1.x | Type-safe file-based routing |
| TanStack Query | 5.x | Server state management, caching |
| TanStack Table | 8.x | Headless table with sorting, filtering, pagination |
| TanStack Virtual | 3.x | Virtualization for long lists |

### State Management
| Library | Version | Purpose |
|---------|---------|---------|
| Zustand | 4.x | Client state management |
| Immer | 10.x | Immutable state updates (with Zustand) |

### UI Components
| Library | Version | Purpose |
|---------|---------|---------|
| shadcn/ui | latest | Component library (copy-paste, customizable) |
| Radix UI | latest | Accessible primitives (used by shadcn) |
| Tailwind CSS | 3.4+ | Utility-first CSS |
| tailwind-merge | latest | Merge Tailwind classes |
| clsx | latest | Conditional class names |
| class-variance-authority | latest | Component variants |

### Forms & Validation
| Library | Version | Purpose |
|---------|---------|---------|
| React Hook Form | 7.x | Form state management |
| Zod | 3.x | Schema validation |
| @hookform/resolvers | latest | Zod resolver for RHF |

### Charts & Visualization
| Library | Version | Purpose |
|---------|---------|---------|
| Recharts | 2.x | Chart library (built on D3) |

### Utilities
| Library | Version | Purpose |
|---------|---------|---------|
| date-fns | 3.x | Date manipulation |
| numeral | 2.x | Number formatting |
| lucide-react | latest | Icons |
| sonner | latest | Toast notifications |

### Development
| Library | Version | Purpose |
|---------|---------|---------|
| Vitest | 1.x | Unit testing |
| @testing-library/react | latest | Component testing |
| Playwright | latest | E2E testing |
| ESLint | 8.x | Linting |
| Prettier | 3.x | Code formatting |

---

## Project Structure

```
frontend/
├── index.html
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
├── components.json              # shadcn/ui config
│
├── public/
│   └── favicon.ico
│
├── src/
│   ├── main.tsx                 # App entry point
│   ├── App.tsx                  # Root component
│   ├── index.css                # Global styles + Tailwind
│   │
│   ├── components/
│   │   ├── ui/                  # shadcn/ui components
│   │   │   ├── button.tsx
│   │   │   ├── input.tsx
│   │   │   ├── card.tsx
│   │   │   ├── dialog.tsx
│   │   │   ├── dropdown-menu.tsx
│   │   │   ├── select.tsx
│   │   │   ├── table.tsx
│   │   │   ├── tabs.tsx
│   │   │   ├── toast.tsx
│   │   │   ├── form.tsx
│   │   │   └── ...
│   │   │
│   │   ├── layout/              # Layout components
│   │   │   ├── root-layout.tsx
│   │   │   ├── header.tsx
│   │   │   ├── sidebar.tsx
│   │   │   ├── mobile-nav.tsx
│   │   │   └── page-header.tsx
│   │   │
│   │   ├── features/            # Feature flag components
│   │   │   ├── feature-gate.tsx
│   │   │   ├── feature-card.tsx
│   │   │   └── feature-settings.tsx
│   │   │
│   │   ├── accounts/            # Account components
│   │   │   ├── account-card.tsx
│   │   │   ├── account-form.tsx
│   │   │   └── account-list.tsx
│   │   │
│   │   ├── transactions/        # Transaction components
│   │   │   ├── transaction-table.tsx
│   │   │   ├── transaction-filters.tsx
│   │   │   ├── transaction-detail.tsx
│   │   │   └── category-picker.tsx
│   │   │
│   │   ├── statements/          # Statement components
│   │   │   ├── statement-card.tsx
│   │   │   ├── statement-uploader.tsx
│   │   │   └── upload-wizard.tsx
│   │   │
│   │   ├── insights/            # Insights components (Phase 2)
│   │   │   ├── spending-chart.tsx
│   │   │   ├── trend-chart.tsx
│   │   │   └── summary-cards.tsx
│   │   │
│   │   ├── budgets/             # Budget components (Phase 2)
│   │   │   ├── budget-card.tsx
│   │   │   ├── budget-progress.tsx
│   │   │   └── budget-form.tsx
│   │   │
│   │   ├── family/              # Family components (Phase 2)
│   │   │   ├── member-card.tsx
│   │   │   ├── invite-form.tsx
│   │   │   ├── role-manager.tsx
│   │   │   └── permission-gate.tsx
│   │   │
│   │   ├── portfolio/           # Investment components (Phase 3)
│   │   │   ├── portfolio-card.tsx
│   │   │   ├── holding-row.tsx
│   │   │   ├── allocation-chart.tsx
│   │   │   └── performance-chart.tsx
│   │   │
│   │   ├── trading/             # Trading components (Phase 3)
│   │   │   ├── strategy-card.tsx
│   │   │   ├── strategy-builder.tsx
│   │   │   ├── backtest-results.tsx
│   │   │   └── signal-card.tsx
│   │   │
│   │   └── shared/              # Shared components
│   │       ├── amount-display.tsx
│   │       ├── date-range-picker.tsx
│   │       ├── loading-skeleton.tsx
│   │       ├── empty-state.tsx
│   │       ├── error-boundary.tsx
│   │       └── confirm-dialog.tsx
│   │
│   ├── routes/                  # TanStack Router routes
│   │   ├── __root.tsx           # Root layout
│   │   ├── index.tsx            # Dashboard (/)
│   │   ├── login.tsx
│   │   ├── register.tsx
│   │   ├── accounts/
│   │   │   ├── index.tsx        # /accounts
│   │   │   └── $accountId.tsx   # /accounts/:accountId
│   │   ├── transactions/
│   │   │   └── index.tsx        # /transactions
│   │   ├── statements/
│   │   │   └── index.tsx        # /statements
│   │   ├── insights/
│   │   │   └── index.tsx        # /insights (Phase 2)
│   │   ├── budgets/
│   │   │   └── index.tsx        # /budgets (Phase 2)
│   │   ├── family/
│   │   │   ├── index.tsx        # /family (Phase 2)
│   │   │   └── roles.tsx        # /family/roles
│   │   ├── portfolio/
│   │   │   ├── index.tsx        # /portfolio (Phase 3)
│   │   │   └── accounts.tsx     # /portfolio/accounts
│   │   ├── trading/
│   │   │   ├── index.tsx        # /trading (Phase 3)
│   │   │   ├── $strategyId.tsx  # /trading/:strategyId
│   │   │   └── signals.tsx      # /trading/signals
│   │   └── settings/
│   │       ├── index.tsx        # /settings
│   │       ├── profile.tsx      # /settings/profile
│   │       ├── features.tsx     # /settings/features
│   │       └── security.tsx     # /settings/security
│   │
│   ├── stores/                  # Zustand stores
│   │   ├── auth-store.ts
│   │   ├── feature-store.ts
│   │   ├── ui-store.ts
│   │   └── index.ts
│   │
│   ├── hooks/                   # Custom hooks
│   │   ├── use-auth.ts
│   │   ├── use-features.ts
│   │   ├── use-permissions.ts
│   │   ├── use-debounce.ts
│   │   └── use-media-query.ts
│   │
│   ├── lib/                     # Utilities
│   │   ├── api.ts               # API client (fetch wrapper)
│   │   ├── utils.ts             # General utilities
│   │   ├── format.ts            # Formatting helpers
│   │   ├── validators.ts        # Zod schemas
│   │   └── constants.ts
│   │
│   ├── types/                   # TypeScript types
│   │   ├── index.ts
│   │   ├── api.ts               # API response types
│   │   ├── user.ts
│   │   ├── account.ts
│   │   ├── transaction.ts
│   │   ├── feature.ts
│   │   ├── family.ts
│   │   ├── portfolio.ts
│   │   └── trading.ts
│   │
│   └── test/                    # Test utilities
│       ├── setup.ts
│       ├── mocks/
│       └── utils.tsx
│
└── e2e/                         # Playwright E2E tests
    ├── auth.spec.ts
    ├── accounts.spec.ts
    └── transactions.spec.ts
```

---

## Configuration Files

### vite.config.ts

```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { TanStackRouterVite } from '@tanstack/router-vite-plugin';
import path from 'path';

export default defineConfig({
  plugins: [
    react(),
    TanStackRouterVite(),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          'react-vendor': ['react', 'react-dom'],
          'tanstack': ['@tanstack/react-query', '@tanstack/react-router', '@tanstack/react-table'],
          'ui': ['@radix-ui/react-dialog', '@radix-ui/react-dropdown-menu', '@radix-ui/react-select'],
          'charts': ['recharts'],
        },
      },
    },
  },
});
```

### tailwind.config.js

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: ['class'],
  content: [
    './index.html',
    './src/**/*.{ts,tsx}',
  ],
  theme: {
    container: {
      center: true,
      padding: '2rem',
      screens: {
        '2xl': '1400px',
      },
    },
    extend: {
      colors: {
        border: 'hsl(var(--border))',
        input: 'hsl(var(--input))',
        ring: 'hsl(var(--ring))',
        background: 'hsl(var(--background))',
        foreground: 'hsl(var(--foreground))',
        primary: {
          DEFAULT: 'hsl(var(--primary))',
          foreground: 'hsl(var(--primary-foreground))',
        },
        secondary: {
          DEFAULT: 'hsl(var(--secondary))',
          foreground: 'hsl(var(--secondary-foreground))',
        },
        destructive: {
          DEFAULT: 'hsl(var(--destructive))',
          foreground: 'hsl(var(--destructive-foreground))',
        },
        muted: {
          DEFAULT: 'hsl(var(--muted))',
          foreground: 'hsl(var(--muted-foreground))',
        },
        accent: {
          DEFAULT: 'hsl(var(--accent))',
          foreground: 'hsl(var(--accent-foreground))',
        },
        popover: {
          DEFAULT: 'hsl(var(--popover))',
          foreground: 'hsl(var(--popover-foreground))',
        },
        card: {
          DEFAULT: 'hsl(var(--card))',
          foreground: 'hsl(var(--card-foreground))',
        },
        // Custom colors for finn-lens
        success: {
          DEFAULT: '#10B981',
          foreground: '#FFFFFF',
        },
        warning: {
          DEFAULT: '#F59E0B',
          foreground: '#FFFFFF',
        },
        income: '#10B981',
        expense: '#EF4444',
      },
      borderRadius: {
        lg: 'var(--radius)',
        md: 'calc(var(--radius) - 2px)',
        sm: 'calc(var(--radius) - 4px)',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      keyframes: {
        'accordion-down': {
          from: { height: 0 },
          to: { height: 'var(--radix-accordion-content-height)' },
        },
        'accordion-up': {
          from: { height: 'var(--radix-accordion-content-height)' },
          to: { height: 0 },
        },
      },
      animation: {
        'accordion-down': 'accordion-down 0.2s ease-out',
        'accordion-up': 'accordion-up 0.2s ease-out',
      },
    },
  },
  plugins: [require('tailwindcss-animate')],
};
```

### tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

### components.json (shadcn/ui)

```json
{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "default",
  "rsc": false,
  "tsx": true,
  "tailwind": {
    "config": "tailwind.config.js",
    "css": "src/index.css",
    "baseColor": "slate",
    "cssVariables": true,
    "prefix": ""
  },
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  }
}
```

---

## TanStack Router Setup

### Route Tree

```typescript
// src/routeTree.gen.ts (auto-generated)
// Run: npx @tanstack/router-cli generate

import { Route as rootRoute } from './routes/__root'
import { Route as IndexRoute } from './routes/index'
import { Route as LoginRoute } from './routes/login'
import { Route as RegisterRoute } from './routes/register'
import { Route as AccountsIndexRoute } from './routes/accounts/index'
import { Route as AccountsAccountIdRoute } from './routes/accounts/$accountId'
// ... more routes
```

### Root Layout

```typescript
// src/routes/__root.tsx
import { createRootRoute, Outlet, useRouter } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/router-devtools';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { Toaster } from '@/components/ui/sonner';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes
      gcTime: 30 * 60 * 1000, // 30 minutes (formerly cacheTime)
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  return (
    <QueryClientProvider client={queryClient}>
      <Outlet />
      <Toaster position="top-right" />
      {import.meta.env.DEV && (
        <>
          <TanStackRouterDevtools position="bottom-right" />
          <ReactQueryDevtools initialIsOpen={false} />
        </>
      )}
    </QueryClientProvider>
  );
}
```

### Protected Route Layout

```typescript
// src/routes/_authenticated.tsx
import { createFileRoute, redirect, Outlet } from '@tanstack/react-router';
import { RootLayout } from '@/components/layout/root-layout';
import { useAuthStore } from '@/stores/auth-store';

export const Route = createFileRoute('/_authenticated')({
  beforeLoad: async ({ location }) => {
    const { isAuthenticated, checkAuth } = useAuthStore.getState();

    // Check if user is authenticated
    if (!isAuthenticated) {
      const isValid = await checkAuth();
      if (!isValid) {
        throw redirect({
          to: '/login',
          search: { redirect: location.href },
        });
      }
    }
  },
  component: AuthenticatedLayout,
});

function AuthenticatedLayout() {
  return (
    <RootLayout>
      <Outlet />
    </RootLayout>
  );
}
```

### Feature-Gated Route

```typescript
// src/routes/_authenticated/insights/index.tsx
import { createFileRoute } from '@tanstack/react-router';
import { useFeatureStore } from '@/stores/feature-store';
import { InsightsPage } from '@/components/insights/insights-page';
import { FeatureDisabled } from '@/components/features/feature-disabled';

export const Route = createFileRoute('/_authenticated/insights/')({
  component: InsightsRoute,
});

function InsightsRoute() {
  const { isEnabled } = useFeatureStore();

  if (!isEnabled('insights')) {
    return <FeatureDisabled feature="insights" />;
  }

  return <InsightsPage />;
}
```

### Route with Data Loading

```typescript
// src/routes/_authenticated/accounts/index.tsx
import { createFileRoute } from '@tanstack/react-router';
import { accountsQueryOptions } from '@/lib/api/accounts';
import { AccountsPage } from '@/components/accounts/accounts-page';

export const Route = createFileRoute('/_authenticated/accounts/')({
  loader: ({ context: { queryClient } }) =>
    queryClient.ensureQueryData(accountsQueryOptions()),
  component: AccountsPage,
  pendingComponent: () => <AccountsPageSkeleton />,
  errorComponent: ({ error }) => <ErrorPage error={error} />,
});
```

---

## TanStack Query Setup

### Query Client Configuration

```typescript
// src/lib/query-client.ts
import { QueryClient } from '@tanstack/react-query';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes
      gcTime: 30 * 60 * 1000, // 30 minutes
      retry: (failureCount, error) => {
        // Don't retry on 4xx errors
        if (error instanceof ApiError && error.status >= 400 && error.status < 500) {
          return false;
        }
        return failureCount < 2;
      },
      refetchOnWindowFocus: false,
    },
    mutations: {
      retry: false,
    },
  },
});
```

### Query Options Pattern

```typescript
// src/lib/api/accounts.ts
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '@/lib/api';
import type { Account, CreateAccountInput } from '@/types/account';

// Query options (can be used with useQuery or loader)
export const accountsQueryOptions = () =>
  queryOptions({
    queryKey: ['accounts'],
    queryFn: () => api.get<Account[]>('/accounts'),
  });

export const accountQueryOptions = (id: string) =>
  queryOptions({
    queryKey: ['accounts', id],
    queryFn: () => api.get<Account>(`/accounts/${id}`),
  });

// Hooks
export function useAccounts() {
  return useQuery(accountsQueryOptions());
}

export function useAccount(id: string) {
  return useQuery(accountQueryOptions(id));
}

export function useCreateAccount() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: CreateAccountInput) =>
      api.post<Account>('/accounts', data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['accounts'] });
    },
  });
}

export function useUpdateAccount() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<Account> }) =>
      api.put<Account>(`/accounts/${id}`, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['accounts'] });
      queryClient.invalidateQueries({ queryKey: ['accounts', id] });
    },
  });
}

export function useDeleteAccount() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => api.delete(`/accounts/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['accounts'] });
    },
  });
}
```

### Transactions with Filters

```typescript
// src/lib/api/transactions.ts
import { queryOptions, useInfiniteQuery } from '@tanstack/react-query';
import { api } from '@/lib/api';
import type { Transaction, TransactionFilters } from '@/types/transaction';

interface TransactionsResponse {
  data: Transaction[];
  pagination: {
    page: number;
    pageSize: number;
    total: number;
    totalPages: number;
  };
}

export const transactionsQueryOptions = (filters: TransactionFilters) =>
  queryOptions({
    queryKey: ['transactions', filters],
    queryFn: () => api.get<TransactionsResponse>('/transactions', { params: filters }),
  });

// Infinite query for virtualized list
export function useInfiniteTransactions(filters: Omit<TransactionFilters, 'page'>) {
  return useInfiniteQuery({
    queryKey: ['transactions', 'infinite', filters],
    queryFn: ({ pageParam = 1 }) =>
      api.get<TransactionsResponse>('/transactions', {
        params: { ...filters, page: pageParam },
      }),
    getNextPageParam: (lastPage) =>
      lastPage.pagination.page < lastPage.pagination.totalPages
        ? lastPage.pagination.page + 1
        : undefined,
    initialPageParam: 1,
  });
}
```

---

## TanStack Table Setup

### Transaction Table

```typescript
// src/components/transactions/transaction-table.tsx
import {
  useReactTable,
  getCoreRowModel,
  getSortedRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  flexRender,
  type ColumnDef,
  type SortingState,
  type ColumnFiltersState,
} from '@tanstack/react-table';
import { useState } from 'react';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { AmountDisplay } from '@/components/shared/amount-display';
import { format } from 'date-fns';
import type { Transaction } from '@/types/transaction';

const columns: ColumnDef<Transaction>[] = [
  {
    id: 'select',
    header: ({ table }) => (
      <Checkbox
        checked={table.getIsAllPageRowsSelected()}
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
        aria-label="Select all"
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(value) => row.toggleSelected(!!value)}
        aria-label="Select row"
      />
    ),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: 'transactionDate',
    header: 'Date',
    cell: ({ row }) => format(new Date(row.getValue('transactionDate')), 'MMM d, yyyy'),
  },
  {
    accessorKey: 'description',
    header: 'Description',
    cell: ({ row }) => (
      <span className="font-medium">{row.getValue('description')}</span>
    ),
  },
  {
    accessorKey: 'category',
    header: 'Category',
    cell: ({ row }) => (
      <span className="text-muted-foreground">
        {row.original.category?.name || 'Uncategorized'}
      </span>
    ),
  },
  {
    accessorKey: 'amount',
    header: () => <div className="text-right">Amount</div>,
    cell: ({ row }) => (
      <div className="text-right">
        <AmountDisplay
          amount={row.getValue('amount')}
          currency={row.original.currency}
          colorize
        />
      </div>
    ),
  },
];

interface TransactionTableProps {
  data: Transaction[];
  isLoading?: boolean;
}

export function TransactionTable({ data, isLoading }: TransactionTableProps) {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [rowSelection, setRowSelection] = useState({});

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    onRowSelectionChange: setRowSelection,
    state: {
      sorting,
      columnFilters,
      rowSelection,
    },
  });

  const selectedRows = table.getFilteredSelectedRowModel().rows;

  return (
    <div className="space-y-4">
      {/* Bulk Actions */}
      {selectedRows.length > 0 && (
        <div className="flex items-center gap-2 p-2 bg-muted rounded-lg">
          <span className="text-sm">{selectedRows.length} selected</span>
          <Button variant="outline" size="sm">
            Categorize
          </Button>
          <Button variant="outline" size="sm" className="text-destructive">
            Delete
          </Button>
        </div>
      )}

      {/* Table */}
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && 'selected'}
                  className="cursor-pointer hover:bg-muted/50"
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell colSpan={columns.length} className="h-24 text-center">
                  No transactions found.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>

      {/* Pagination */}
      <div className="flex items-center justify-between px-2">
        <div className="text-sm text-muted-foreground">
          Showing {table.getRowModel().rows.length} of {data.length} transactions
        </div>
        <div className="flex items-center space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.previousPage()}
            disabled={!table.getCanPreviousPage()}
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.nextPage()}
            disabled={!table.getCanNextPage()}
          >
            Next
          </Button>
        </div>
      </div>
    </div>
  );
}
```

---

## Zustand Stores

### Auth Store

```typescript
// src/stores/auth-store.ts
import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import { api } from '@/lib/api';
import type { User } from '@/types/user';

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;

  // Actions
  login: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  register: (data: RegisterInput) => Promise<void>;
  checkAuth: () => Promise<boolean>;
  updateUser: (data: Partial<User>) => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    immer((set, get) => ({
      user: null,
      isAuthenticated: false,
      isLoading: false,

      login: async (email, password) => {
        set({ isLoading: true });
        try {
          const response = await api.post<{ user: User }>('/auth/login', {
            email,
            password,
          });
          set({
            user: response.user,
            isAuthenticated: true,
            isLoading: false,
          });
        } catch (error) {
          set({ isLoading: false });
          throw error;
        }
      },

      logout: async () => {
        try {
          await api.post('/auth/logout');
        } finally {
          set({
            user: null,
            isAuthenticated: false,
          });
        }
      },

      register: async (data) => {
        set({ isLoading: true });
        try {
          const response = await api.post<{ user: User }>('/auth/register', data);
          set({
            user: response.user,
            isAuthenticated: true,
            isLoading: false,
          });
        } catch (error) {
          set({ isLoading: false });
          throw error;
        }
      },

      checkAuth: async () => {
        try {
          const response = await api.get<{ user: User }>('/auth/me');
          set({
            user: response.user,
            isAuthenticated: true,
          });
          return true;
        } catch {
          set({
            user: null,
            isAuthenticated: false,
          });
          return false;
        }
      },

      updateUser: (data) => {
        set((state) => {
          if (state.user) {
            state.user = { ...state.user, ...data };
          }
        });
      },
    })),
    {
      name: 'auth-storage',
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        isAuthenticated: state.isAuthenticated,
      }),
    }
  )
);
```

### Feature Store

```typescript
// src/stores/feature-store.ts
import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { api } from '@/lib/api';
import type { Feature } from '@/types/feature';

interface FeatureState {
  features: Feature[];
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchFeatures: () => Promise<void>;
  enableFeature: (featureId: string) => Promise<void>;
  disableFeature: (featureId: string) => Promise<void>;

  // Selectors
  isEnabled: (featureId: string) => boolean;
  getFeature: (featureId: string) => Feature | undefined;
  getEnabledFeatures: () => Feature[];
}

export const useFeatureStore = create<FeatureState>()(
  immer((set, get) => ({
    features: [],
    isLoading: false,
    error: null,

    fetchFeatures: async () => {
      set({ isLoading: true, error: null });
      try {
        const features = await api.get<Feature[]>('/user/features');
        set({ features, isLoading: false });
      } catch (error) {
        set({
          isLoading: false,
          error: error instanceof Error ? error.message : 'Failed to load features',
        });
      }
    },

    enableFeature: async (featureId) => {
      try {
        await api.post(`/user/features/${featureId}/enable`);
        set((state) => {
          const feature = state.features.find((f) => f.id === featureId);
          if (feature) {
            feature.enabled = true;
          }
        });
      } catch (error) {
        throw error;
      }
    },

    disableFeature: async (featureId) => {
      try {
        await api.post(`/user/features/${featureId}/disable`);
        set((state) => {
          const feature = state.features.find((f) => f.id === featureId);
          if (feature) {
            feature.enabled = false;
          }
        });
      } catch (error) {
        throw error;
      }
    },

    isEnabled: (featureId) => {
      const feature = get().features.find((f) => f.id === featureId);
      return feature?.enabled ?? false;
    },

    getFeature: (featureId) => {
      return get().features.find((f) => f.id === featureId);
    },

    getEnabledFeatures: () => {
      return get().features.filter((f) => f.enabled);
    },
  }))
);
```

### UI Store

```typescript
// src/stores/ui-store.ts
import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

interface UIState {
  // Sidebar
  sidebarCollapsed: boolean;
  sidebarOpen: boolean; // Mobile drawer

  // Theme
  theme: 'light' | 'dark' | 'system';

  // Preferences
  dateFormat: string;
  numberFormat: string;
  transactionsPerPage: number;

  // Actions
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  setTheme: (theme: 'light' | 'dark' | 'system') => void;
  setPreference: <K extends keyof UIState>(key: K, value: UIState[K]) => void;
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      sidebarCollapsed: false,
      sidebarOpen: false,
      theme: 'system',
      dateFormat: 'MMM d, yyyy',
      numberFormat: 'en-US',
      transactionsPerPage: 25,

      toggleSidebar: () =>
        set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),

      setSidebarOpen: (open) => set({ sidebarOpen: open }),

      setTheme: (theme) => set({ theme }),

      setPreference: (key, value) => set({ [key]: value }),
    }),
    {
      name: 'ui-storage',
      storage: createJSONStorage(() => localStorage),
    }
  )
);
```

---

## API Client

```typescript
// src/lib/api.ts
import { useAuthStore } from '@/stores/auth-store';

const API_BASE = '/api/v1';

interface RequestConfig extends RequestInit {
  params?: Record<string, string | number | boolean | undefined>;
}

class ApiError extends Error {
  constructor(
    public status: number,
    public code: string,
    message: string,
    public details?: Record<string, string[]>
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

async function request<T>(
  endpoint: string,
  config: RequestConfig = {}
): Promise<T> {
  const { params, ...init } = config;

  // Build URL with query params
  const url = new URL(`${API_BASE}${endpoint}`, window.location.origin);
  if (params) {
    Object.entries(params).forEach(([key, value]) => {
      if (value !== undefined) {
        url.searchParams.append(key, String(value));
      }
    });
  }

  const response = await fetch(url.toString(), {
    ...init,
    headers: {
      'Content-Type': 'application/json',
      ...init.headers,
    },
    credentials: 'include', // Include cookies
  });

  if (!response.ok) {
    // Handle 401 - Unauthorized
    if (response.status === 401) {
      useAuthStore.getState().logout();
      window.location.href = '/login';
    }

    const error = await response.json().catch(() => ({
      code: 'unknown',
      message: 'An error occurred',
    }));

    throw new ApiError(
      response.status,
      error.code || 'unknown',
      error.message || 'An error occurred',
      error.details
    );
  }

  // Handle 204 No Content
  if (response.status === 204) {
    return undefined as T;
  }

  return response.json();
}

export const api = {
  get: <T>(endpoint: string, config?: RequestConfig) =>
    request<T>(endpoint, { ...config, method: 'GET' }),

  post: <T>(endpoint: string, data?: unknown, config?: RequestConfig) =>
    request<T>(endpoint, {
      ...config,
      method: 'POST',
      body: data ? JSON.stringify(data) : undefined,
    }),

  put: <T>(endpoint: string, data?: unknown, config?: RequestConfig) =>
    request<T>(endpoint, {
      ...config,
      method: 'PUT',
      body: data ? JSON.stringify(data) : undefined,
    }),

  patch: <T>(endpoint: string, data?: unknown, config?: RequestConfig) =>
    request<T>(endpoint, {
      ...config,
      method: 'PATCH',
      body: data ? JSON.stringify(data) : undefined,
    }),

  delete: <T>(endpoint: string, config?: RequestConfig) =>
    request<T>(endpoint, { ...config, method: 'DELETE' }),
};

export { ApiError };
```

---

## Forms with React Hook Form + Zod

### Account Form

```typescript
// src/components/accounts/account-form.tsx
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { useCreateAccount, useUpdateAccount } from '@/lib/api/accounts';
import { toast } from 'sonner';

const accountSchema = z.object({
  bankName: z.string().min(1, 'Bank name is required'),
  accountName: z.string().optional(),
  accountType: z.enum(['checking', 'savings', 'credit', 'investment']),
  lastFour: z
    .string()
    .length(4, 'Must be exactly 4 digits')
    .regex(/^\d+$/, 'Must be digits only')
    .optional(),
  currency: z.string().min(1, 'Currency is required'),
  initialBalance: z.number().optional(),
});

type AccountFormValues = z.infer<typeof accountSchema>;

interface AccountFormProps {
  account?: AccountFormValues & { id: string };
  onSuccess?: () => void;
}

export function AccountForm({ account, onSuccess }: AccountFormProps) {
  const createAccount = useCreateAccount();
  const updateAccount = useUpdateAccount();
  const isEditing = !!account;

  const form = useForm<AccountFormValues>({
    resolver: zodResolver(accountSchema),
    defaultValues: account || {
      bankName: '',
      accountName: '',
      accountType: 'checking',
      lastFour: '',
      currency: 'USD',
      initialBalance: 0,
    },
  });

  async function onSubmit(data: AccountFormValues) {
    try {
      if (isEditing) {
        await updateAccount.mutateAsync({ id: account.id, data });
        toast.success('Account updated successfully');
      } else {
        await createAccount.mutateAsync(data);
        toast.success('Account created successfully');
      }
      onSuccess?.();
    } catch (error) {
      toast.error(isEditing ? 'Failed to update account' : 'Failed to create account');
    }
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
        <FormField
          control={form.control}
          name="bankName"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Bank Name</FormLabel>
              <FormControl>
                <Input placeholder="Chase, Bank of America, etc." {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="accountName"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Account Name (Optional)</FormLabel>
              <FormControl>
                <Input placeholder="My Checking Account" {...field} />
              </FormControl>
              <FormDescription>
                A nickname to help identify this account
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="accountType"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Account Type</FormLabel>
              <Select onValueChange={field.onChange} defaultValue={field.value}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Select account type" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectItem value="checking">Checking</SelectItem>
                  <SelectItem value="savings">Savings</SelectItem>
                  <SelectItem value="credit">Credit Card</SelectItem>
                  <SelectItem value="investment">Investment</SelectItem>
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="lastFour"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Last 4 Digits (Optional)</FormLabel>
              <FormControl>
                <Input placeholder="1234" maxLength={4} {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="currency"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Currency</FormLabel>
              <Select onValueChange={field.onChange} defaultValue={field.value}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Select currency" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectItem value="USD">USD - US Dollar</SelectItem>
                  <SelectItem value="EUR">EUR - Euro</SelectItem>
                  <SelectItem value="GBP">GBP - British Pound</SelectItem>
                  <SelectItem value="INR">INR - Indian Rupee</SelectItem>
                  <SelectItem value="CAD">CAD - Canadian Dollar</SelectItem>
                  <SelectItem value="AUD">AUD - Australian Dollar</SelectItem>
                  <SelectItem value="JPY">JPY - Japanese Yen</SelectItem>
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />

        <div className="flex justify-end gap-2">
          <Button type="button" variant="outline" onClick={() => form.reset()}>
            Cancel
          </Button>
          <Button
            type="submit"
            disabled={createAccount.isPending || updateAccount.isPending}
          >
            {createAccount.isPending || updateAccount.isPending
              ? 'Saving...'
              : isEditing
              ? 'Update Account'
              : 'Create Account'}
          </Button>
        </div>
      </form>
    </Form>
  );
}
```

---

## Custom Hooks

### useFeatures Hook

```typescript
// src/hooks/use-features.ts
import { useEffect } from 'react';
import { useFeatureStore } from '@/stores/feature-store';

export function useFeatures() {
  const {
    features,
    isLoading,
    error,
    fetchFeatures,
    enableFeature,
    disableFeature,
    isEnabled,
    getFeature,
    getEnabledFeatures,
  } = useFeatureStore();

  useEffect(() => {
    if (features.length === 0 && !isLoading) {
      fetchFeatures();
    }
  }, [features.length, isLoading, fetchFeatures]);

  return {
    features,
    isLoading,
    error,
    enableFeature,
    disableFeature,
    isEnabled,
    getFeature,
    getEnabledFeatures,
    refetch: fetchFeatures,
  };
}

// Shorthand hook for checking single feature
export function useFeature(featureId: string) {
  const { isEnabled, getFeature, isLoading } = useFeatureStore();

  return {
    enabled: isEnabled(featureId),
    feature: getFeature(featureId),
    isLoading,
  };
}
```

### usePermissions Hook (Phase 2)

```typescript
// src/hooks/use-permissions.ts
import { useAuthStore } from '@/stores/auth-store';

export function usePermissions() {
  const user = useAuthStore((state) => state.user);
  const permissions = user?.permissions || [];

  const hasPermission = (permission: string): boolean => {
    return permissions.includes(permission) || permissions.includes('*');
  };

  const hasAnyPermission = (perms: string[]): boolean => {
    return perms.some(hasPermission);
  };

  const hasAllPermissions = (perms: string[]): boolean => {
    return perms.every(hasPermission);
  };

  return {
    permissions,
    hasPermission,
    hasAnyPermission,
    hasAllPermissions,
    isOwner: user?.isFamilyOwner ?? false,
    isAdmin: hasPermission('family:*') || user?.isFamilyOwner,
  };
}
```

### useDebounce Hook

```typescript
// src/hooks/use-debounce.ts
import { useState, useEffect } from 'react';

export function useDebounce<T>(value: T, delay: number = 500): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(timer);
    };
  }, [value, delay]);

  return debouncedValue;
}
```

---

## Feature Gate Component

```typescript
// src/components/features/feature-gate.tsx
import { type ReactNode } from 'react';
import { useFeature } from '@/hooks/use-features';

interface FeatureGateProps {
  feature: string;
  children: ReactNode;
  fallback?: ReactNode;
  showLoading?: boolean;
}

export function FeatureGate({
  feature,
  children,
  fallback = null,
  showLoading = false,
}: FeatureGateProps) {
  const { enabled, isLoading } = useFeature(feature);

  if (isLoading && showLoading) {
    return <FeatureGateSkeleton />;
  }

  if (!enabled) {
    return <>{fallback}</>;
  }

  return <>{children}</>;
}

function FeatureGateSkeleton() {
  return (
    <div className="animate-pulse">
      <div className="h-4 bg-muted rounded w-1/4 mb-2" />
      <div className="h-4 bg-muted rounded w-1/2" />
    </div>
  );
}

// HOC version
export function withFeature<P extends object>(
  Component: React.ComponentType<P>,
  featureId: string,
  Fallback?: React.ComponentType
) {
  return function FeatureWrappedComponent(props: P) {
    const { enabled } = useFeature(featureId);

    if (!enabled) {
      return Fallback ? <Fallback /> : null;
    }

    return <Component {...props} />;
  };
}
```

---

## Shared Components

### Amount Display

```typescript
// src/components/shared/amount-display.tsx
import { cn } from '@/lib/utils';

interface AmountDisplayProps {
  amount: number;
  currency: string;
  size?: 'sm' | 'md' | 'lg' | 'xl';
  showSign?: boolean;
  colorize?: boolean;
  className?: string;
}

const sizeClasses = {
  sm: 'text-sm',
  md: 'text-base',
  lg: 'text-lg font-semibold',
  xl: 'text-2xl font-bold',
};

export function AmountDisplay({
  amount,
  currency,
  size = 'md',
  showSign = false,
  colorize = false,
  className,
}: AmountDisplayProps) {
  const isPositive = amount >= 0;
  const absAmount = Math.abs(amount);

  const formatted = new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency,
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(absAmount);

  const sign = showSign ? (isPositive ? '+' : '-') : amount < 0 ? '-' : '';
  const displayValue = `${sign}${formatted.replace('-', '')}`;

  return (
    <span
      className={cn(
        'font-mono',
        sizeClasses[size],
        colorize && (isPositive ? 'text-income' : 'text-expense'),
        className
      )}
    >
      {displayValue}
    </span>
  );
}
```

### Empty State

```typescript
// src/components/shared/empty-state.tsx
import { type LucideIcon } from 'lucide-react';
import { Button } from '@/components/ui/button';

interface EmptyStateProps {
  icon: LucideIcon;
  title: string;
  description: string;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export function EmptyState({
  icon: Icon,
  title,
  description,
  action,
}: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center py-12 px-4 text-center">
      <div className="rounded-full bg-muted p-4 mb-4">
        <Icon className="h-8 w-8 text-muted-foreground" />
      </div>
      <h3 className="text-lg font-semibold mb-1">{title}</h3>
      <p className="text-muted-foreground mb-4 max-w-sm">{description}</p>
      {action && (
        <Button onClick={action.onClick}>{action.label}</Button>
      )}
    </div>
  );
}
```

### Loading Skeleton

```typescript
// src/components/shared/loading-skeleton.tsx
import { Skeleton } from '@/components/ui/skeleton';

export function CardSkeleton() {
  return (
    <div className="rounded-lg border p-4 space-y-3">
      <Skeleton className="h-4 w-1/4" />
      <Skeleton className="h-8 w-1/2" />
      <Skeleton className="h-4 w-3/4" />
    </div>
  );
}

export function TableRowSkeleton({ columns = 5 }: { columns?: number }) {
  return (
    <div className="flex items-center space-x-4 py-3">
      {Array.from({ length: columns }).map((_, i) => (
        <Skeleton key={i} className="h-4 flex-1" />
      ))}
    </div>
  );
}

export function TableSkeleton({ rows = 5, columns = 5 }) {
  return (
    <div className="space-y-2">
      {Array.from({ length: rows }).map((_, i) => (
        <TableRowSkeleton key={i} columns={columns} />
      ))}
    </div>
  );
}

export function PageSkeleton() {
  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <Skeleton className="h-8 w-1/4" />
        <Skeleton className="h-4 w-1/2" />
      </div>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <CardSkeleton />
        <CardSkeleton />
        <CardSkeleton />
      </div>
      <TableSkeleton />
    </div>
  );
}
```

---

## Package.json

```json
{
  "name": "finn-lens-frontend",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "format": "prettier --write \"src/**/*.{ts,tsx,css}\"",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage",
    "e2e": "playwright test",
    "e2e:ui": "playwright test --ui"
  },
  "dependencies": {
    "react": "^18.3.1",
    "react-dom": "^18.3.1",

    "@tanstack/react-router": "^1.45.0",
    "@tanstack/router-devtools": "^1.45.0",
    "@tanstack/react-query": "^5.51.0",
    "@tanstack/react-query-devtools": "^5.51.0",
    "@tanstack/react-table": "^8.19.0",
    "@tanstack/react-virtual": "^3.8.0",

    "zustand": "^4.5.4",
    "immer": "^10.1.1",

    "@radix-ui/react-accordion": "^1.2.0",
    "@radix-ui/react-alert-dialog": "^1.1.1",
    "@radix-ui/react-avatar": "^1.1.0",
    "@radix-ui/react-checkbox": "^1.1.1",
    "@radix-ui/react-dialog": "^1.1.1",
    "@radix-ui/react-dropdown-menu": "^2.1.1",
    "@radix-ui/react-label": "^2.1.0",
    "@radix-ui/react-popover": "^1.1.1",
    "@radix-ui/react-select": "^2.1.1",
    "@radix-ui/react-separator": "^1.1.0",
    "@radix-ui/react-slot": "^1.1.0",
    "@radix-ui/react-switch": "^1.1.0",
    "@radix-ui/react-tabs": "^1.1.0",
    "@radix-ui/react-toast": "^1.2.1",
    "@radix-ui/react-tooltip": "^1.1.2",

    "tailwindcss-animate": "^1.0.7",
    "class-variance-authority": "^0.7.0",
    "clsx": "^2.1.1",
    "tailwind-merge": "^2.4.0",

    "react-hook-form": "^7.52.1",
    "@hookform/resolvers": "^3.9.0",
    "zod": "^3.23.8",

    "recharts": "^2.12.7",

    "date-fns": "^3.6.0",
    "numeral": "^2.0.6",
    "lucide-react": "^0.408.0",
    "sonner": "^1.5.0"
  },
  "devDependencies": {
    "@types/react": "^18.3.3",
    "@types/react-dom": "^18.3.0",
    "@types/numeral": "^2.0.5",

    "typescript": "^5.5.3",
    "vite": "^5.4.0",
    "@vitejs/plugin-react": "^4.3.1",
    "@tanstack/router-vite-plugin": "^1.45.0",

    "tailwindcss": "^3.4.6",
    "postcss": "^8.4.39",
    "autoprefixer": "^10.4.19",

    "eslint": "^8.57.0",
    "eslint-plugin-react-hooks": "^4.6.2",
    "eslint-plugin-react-refresh": "^0.4.8",
    "@typescript-eslint/eslint-plugin": "^7.16.0",
    "@typescript-eslint/parser": "^7.16.0",

    "prettier": "^3.3.3",
    "prettier-plugin-tailwindcss": "^0.6.5",

    "vitest": "^2.0.3",
    "@testing-library/react": "^16.0.0",
    "@testing-library/jest-dom": "^6.4.6",
    "@vitest/ui": "^2.0.3",
    "@vitest/coverage-v8": "^2.0.3",

    "@playwright/test": "^1.45.2"
  }
}
```

---

## Summary

This architecture provides:

1. **Type-Safe Routing** - TanStack Router with file-based routes and typed params
2. **Efficient Data Fetching** - TanStack Query with caching, prefetching, and optimistic updates
3. **Powerful Tables** - TanStack Table with sorting, filtering, pagination, and virtualization
4. **Simple State Management** - Zustand stores with Immer for immutable updates
5. **Beautiful UI** - shadcn/ui components with Tailwind CSS
6. **Type-Safe Forms** - React Hook Form with Zod validation
7. **Feature Flags** - First-class support for conditional rendering
8. **Modular Structure** - Organized by feature for scalability

The stack is modern, performant, and maintainable.
