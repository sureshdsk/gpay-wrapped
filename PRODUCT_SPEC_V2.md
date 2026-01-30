# finn-lens v2: Personal Finance Platform

## Overview

**finn-lens v2** is a modular personal finance platform built with **Rust (Loco framework)** backend and **React** frontend. The application uses a **feature flag system** allowing users to enable/disable modules based on their needs.

---

## Technology Stack

### Backend
- **Framework**: [Loco](https://loco.rs) (Rust web framework, Rails-like)
- **ORM**: SeaORM
- **Database**: PostgreSQL (production) / SQLite (development/local)
- **Authentication**: JWT with Argon2id password hashing
- **Background Jobs**: Loco workers (Sidekiq-like)
- **Validation**: validator crate + custom validators

### Frontend
- **Framework**: React 18 + TypeScript + Vite
- **UI**: shadcn/ui + Radix UI + Tailwind CSS
- **State**: Zustand (client) + TanStack Query (server)
- **Router**: TanStack Router
- **Tables**: TanStack Table
- **Charts**: Recharts
- **Forms**: React Hook Form + Zod

### Deployment
- **Single Binary**: Loco serves embedded React static files
- **Containerization**: Docker + Docker Compose
- **Daemon Mode**: systemd (Linux) / launchd (macOS)

---

## Feature Flag System

### Architecture

The feature flag system is designed to be **open and extensible**. Features are modular components that can be enabled/disabled per user or globally.

```
┌─────────────────────────────────────────────────────────────┐
│                    Feature Flag System                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   Core       │  │  Module 1    │  │  Module 2    │  ...  │
│  │   (Always)   │  │  (Optional)  │  │  (Optional)  │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│         │                 │                 │                │
│         ▼                 ▼                 ▼                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Feature Registry                         │   │
│  │  - Registered features with metadata                  │   │
│  │  - Dependencies between features                      │   │
│  │  - User-level and global toggles                     │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Feature Definition

Each feature is defined with:

```rust
pub struct FeatureDefinition {
    pub id: String,              // Unique identifier (e.g., "personal_finance")
    pub name: String,            // Display name
    pub description: String,     // User-facing description
    pub version: String,         // Feature version
    pub category: FeatureCategory,
    pub dependencies: Vec<String>, // IDs of required features
    pub default_enabled: bool,   // Enabled by default for new users
    pub is_core: bool,           // Core features cannot be disabled
    pub settings_schema: Option<serde_json::Value>, // Optional feature-specific settings
}

pub enum FeatureCategory {
    Core,           // Always enabled
    Finance,        // Personal finance features
    Investment,     // Investment tracking
    Trading,        // Algo trading
    Integration,    // External integrations
    Analytics,      // Analytics and insights
    Automation,     // Automated workflows
}
```

### Built-in Features

| Feature ID | Name | Category | Core | Default | Description |
|------------|------|----------|------|---------|-------------|
| `core_auth` | Authentication | Core | Yes | Yes | User authentication, sessions, 2FA |
| `core_settings` | Settings | Core | Yes | Yes | User preferences and settings |
| `personal_finance` | Personal Finance | Finance | No | Yes | Bank accounts, transactions, statements |
| `family_sharing` | Family Sharing | Finance | No | No | Multi-user family support |
| `budgeting` | Budgeting | Finance | No | No | Budget creation and tracking |
| `insights` | Financial Insights | Analytics | No | Yes | Spending analytics, trends |
| `reports` | Reports | Analytics | No | No | Generate financial reports |
| `investment_portfolio` | Investment Portfolio | Investment | No | No | Track stocks, bonds, crypto |
| `investment_analytics` | Investment Analytics | Investment | No | No | Portfolio performance analysis |
| `algo_trading` | Algo Trading | Trading | No | No | Automated trading strategies |
| `trading_backtest` | Backtesting | Trading | No | No | Test trading strategies |
| `gmail_integration` | Gmail Integration | Integration | No | No | Auto-import statements from email |
| `plaid_integration` | Plaid Integration | Integration | No | No | Direct bank connections |
| `multi_currency` | Multi-Currency | Finance | No | No | Support multiple currencies |
| `custom_categories` | Custom Categories | Finance | No | Yes | User-defined categories |

### Feature Dependencies

```yaml
investment_analytics:
  depends_on: [investment_portfolio]

trading_backtest:
  depends_on: [algo_trading]

algo_trading:
  depends_on: [investment_portfolio]

family_sharing:
  depends_on: [personal_finance]

budgeting:
  depends_on: [personal_finance]

insights:
  depends_on: [personal_finance]
```

### Database Schema for Features

```sql
-- Feature definitions (seeded, managed by system)
CREATE TABLE feature_definitions (
    id VARCHAR(100) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50) NOT NULL,
    category VARCHAR(50) NOT NULL,
    dependencies JSONB DEFAULT '[]',
    default_enabled BOOLEAN DEFAULT FALSE,
    is_core BOOLEAN DEFAULT FALSE,
    settings_schema JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Global feature flags (admin-controlled)
CREATE TABLE global_feature_flags (
    feature_id VARCHAR(100) PRIMARY KEY REFERENCES feature_definitions(id),
    enabled BOOLEAN DEFAULT TRUE,
    rollout_percentage INTEGER DEFAULT 100, -- For gradual rollouts
    config JSONB DEFAULT '{}',
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User-level feature flags
CREATE TABLE user_feature_flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    feature_id VARCHAR(100) NOT NULL REFERENCES feature_definitions(id),
    enabled BOOLEAN NOT NULL,
    config JSONB DEFAULT '{}', -- User-specific feature config
    enabled_at TIMESTAMPTZ,
    disabled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, feature_id)
);

-- Feature usage analytics (optional)
CREATE TABLE feature_usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    feature_id VARCHAR(100) NOT NULL,
    action VARCHAR(100) NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### API Endpoints

```
GET  /api/v1/features                    # List all available features
GET  /api/v1/features/:id                # Get feature details
GET  /api/v1/user/features               # Get user's enabled features
POST /api/v1/user/features/:id/enable    # Enable a feature
POST /api/v1/user/features/:id/disable   # Disable a feature
PUT  /api/v1/user/features/:id/config    # Update feature config

# Admin endpoints
GET  /api/v1/admin/features              # List all features with stats
PUT  /api/v1/admin/features/:id          # Update global feature settings
```

### Backend Implementation

```rust
// src/features/mod.rs

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRegistry {
    features: HashMap<String, FeatureDefinition>,
}

impl FeatureRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            features: HashMap::new(),
        };

        // Register core features
        registry.register(core_features::auth());
        registry.register(core_features::settings());

        // Register optional features
        registry.register(finance_features::personal_finance());
        registry.register(finance_features::family_sharing());
        registry.register(finance_features::budgeting());
        registry.register(analytics_features::insights());
        registry.register(investment_features::portfolio());
        registry.register(trading_features::algo_trading());

        registry
    }

    pub fn register(&mut self, feature: FeatureDefinition) {
        self.features.insert(feature.id.clone(), feature);
    }

    pub fn get(&self, id: &str) -> Option<&FeatureDefinition> {
        self.features.get(id)
    }

    pub fn can_enable(&self, feature_id: &str, enabled_features: &[String]) -> Result<(), FeatureError> {
        let feature = self.get(feature_id)
            .ok_or(FeatureError::NotFound)?;

        // Check all dependencies are enabled
        for dep in &feature.dependencies {
            if !enabled_features.contains(dep) {
                return Err(FeatureError::DependencyNotEnabled(dep.clone()));
            }
        }

        Ok(())
    }

    pub fn can_disable(&self, feature_id: &str, enabled_features: &[String]) -> Result<(), FeatureError> {
        // Check no other enabled features depend on this one
        for (id, feature) in &self.features {
            if enabled_features.contains(id) && feature.dependencies.contains(&feature_id.to_string()) {
                return Err(FeatureError::RequiredByOther(id.clone()));
            }
        }

        Ok(())
    }
}

// Feature guard middleware
pub struct FeatureGuard {
    required_feature: String,
}

impl FeatureGuard {
    pub fn new(feature: &str) -> Self {
        Self {
            required_feature: feature.to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for FeatureGuard
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = get_current_user(parts, state).await?;

        if !user.has_feature(&self.required_feature).await? {
            return Err(AppError::FeatureNotEnabled(self.required_feature.clone()));
        }

        Ok(self)
    }
}

// Usage in controllers
#[get("/portfolio")]
async fn get_portfolio(
    _feature: FeatureGuard::new("investment_portfolio"),
    auth: Auth,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    // Only accessible if investment_portfolio feature is enabled
}
```

### Frontend Implementation

```typescript
// src/features/FeatureContext.tsx
import { createContext, useContext, ReactNode } from 'react';
import { useQuery } from '@tanstack/react-query';

interface Feature {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  category: string;
  dependencies: string[];
}

interface FeatureContextType {
  features: Feature[];
  isFeatureEnabled: (featureId: string) => boolean;
  enableFeature: (featureId: string) => Promise<void>;
  disableFeature: (featureId: string) => Promise<void>;
  isLoading: boolean;
}

const FeatureContext = createContext<FeatureContextType | null>(null);

export function FeatureProvider({ children }: { children: ReactNode }) {
  const { data: features = [], isLoading } = useQuery({
    queryKey: ['user-features'],
    queryFn: () => api.get('/user/features'),
  });

  const isFeatureEnabled = (featureId: string): boolean => {
    return features.some(f => f.id === featureId && f.enabled);
  };

  const enableFeature = async (featureId: string) => {
    await api.post(`/user/features/${featureId}/enable`);
    queryClient.invalidateQueries(['user-features']);
  };

  const disableFeature = async (featureId: string) => {
    await api.post(`/user/features/${featureId}/disable`);
    queryClient.invalidateQueries(['user-features']);
  };

  return (
    <FeatureContext.Provider value={{
      features,
      isFeatureEnabled,
      enableFeature,
      disableFeature,
      isLoading
    }}>
      {children}
    </FeatureContext.Provider>
  );
}

export const useFeatures = () => {
  const context = useContext(FeatureContext);
  if (!context) throw new Error('useFeatures must be used within FeatureProvider');
  return context;
};

// FeatureGate component
interface FeatureGateProps {
  feature: string;
  children: ReactNode;
  fallback?: ReactNode;
}

export function FeatureGate({ feature, children, fallback = null }: FeatureGateProps) {
  const { isFeatureEnabled, isLoading } = useFeatures();

  if (isLoading) return null;
  if (!isFeatureEnabled(feature)) return fallback;

  return <>{children}</>;
}

// Usage
function Dashboard() {
  return (
    <div>
      <SummaryCards />

      <FeatureGate feature="personal_finance">
        <TransactionList />
        <AccountOverview />
      </FeatureGate>

      <FeatureGate feature="investment_portfolio">
        <PortfolioSummary />
      </FeatureGate>

      <FeatureGate feature="algo_trading">
        <TradingStrategies />
      </FeatureGate>
    </div>
  );
}
```

### Feature Settings UI

```
┌────────────────────────────────────────────────────────────┐
│ Features & Modules                                         │
├────────────────────────────────────────────────────────────┤
│                                                            │
│ Core (Always Enabled)                                      │
│ ┌────────────────────────────────────────────────────┐    │
│ │ ✅ Authentication               🔒 Core            │    │
│ │ ✅ Settings                     🔒 Core            │    │
│ └────────────────────────────────────────────────────┘    │
│                                                            │
│ Personal Finance                                           │
│ ┌────────────────────────────────────────────────────┐    │
│ │ [✓] Personal Finance                               │    │
│ │     Track bank accounts, transactions, statements  │    │
│ │                                      [Configure]   │    │
│ ├────────────────────────────────────────────────────┤    │
│ │ [ ] Family Sharing        (requires Personal Finance)   │
│ │     Share finances with family members             │    │
│ ├────────────────────────────────────────────────────┤    │
│ │ [✓] Financial Insights                             │    │
│ │     Spending analytics, trends, reports            │    │
│ ├────────────────────────────────────────────────────┤    │
│ │ [ ] Budgeting            (requires Personal Finance)    │
│ │     Create and track budgets                       │    │
│ └────────────────────────────────────────────────────┘    │
│                                                            │
│ Investment                                                 │
│ ┌────────────────────────────────────────────────────┐    │
│ │ [ ] Investment Portfolio                           │    │
│ │     Track stocks, bonds, crypto, mutual funds      │    │
│ │     Coming in v2.1                        [Beta]   │    │
│ ├────────────────────────────────────────────────────┤    │
│ │ [ ] Investment Analytics  (requires Portfolio)     │    │
│ │     Portfolio performance, allocation analysis     │    │
│ │     Coming in v2.2                                 │    │
│ └────────────────────────────────────────────────────┘    │
│                                                            │
│ Trading                                                    │
│ ┌────────────────────────────────────────────────────┐    │
│ │ [ ] Algo Trading         (requires Portfolio)      │    │
│ │     Create automated trading strategies            │    │
│ │     Coming in v3.0                      [Preview]  │    │
│ ├────────────────────────────────────────────────────┤    │
│ │ [ ] Backtesting          (requires Algo Trading)   │    │
│ │     Test strategies against historical data        │    │
│ │     Coming in v3.0                                 │    │
│ └────────────────────────────────────────────────────┘    │
│                                                            │
│ Integrations                                               │
│ ┌────────────────────────────────────────────────────┐    │
│ │ [ ] Gmail Integration                              │    │
│ │     Auto-import bank statements from email         │    │
│ ├────────────────────────────────────────────────────┤    │
│ │ [ ] Plaid Integration                              │    │
│ │     Connect bank accounts directly                 │    │
│ │     Coming in v2.2                                 │    │
│ └────────────────────────────────────────────────────┘    │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

---

## Project Structure

```
finn-lens/
├── Cargo.toml                    # Workspace configuration
├── config/                       # Loco configuration
│   ├── development.yaml
│   ├── production.yaml
│   └── test.yaml
│
├── src/
│   ├── main.rs                   # Application entry point
│   ├── app.rs                    # Loco app configuration
│   ├── lib.rs                    # Library exports
│   │
│   ├── controllers/              # API route handlers
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── accounts.rs
│   │   ├── transactions.rs
│   │   ├── statements.rs
│   │   ├── insights.rs
│   │   ├── features.rs
│   │   ├── family.rs
│   │   └── admin/
│   │       ├── mod.rs
│   │       └── features.rs
│   │
│   ├── models/                   # SeaORM entities
│   │   ├── mod.rs
│   │   ├── _entities/            # Auto-generated
│   │   ├── users.rs
│   │   ├── bank_accounts.rs
│   │   ├── transactions.rs
│   │   ├── statements.rs
│   │   ├── categories.rs
│   │   ├── families.rs
│   │   ├── roles.rs
│   │   ├── feature_definitions.rs
│   │   ├── user_feature_flags.rs
│   │   └── global_feature_flags.rs
│   │
│   ├── services/                 # Business logic
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── account.rs
│   │   ├── transaction.rs
│   │   ├── statement.rs
│   │   ├── parser.rs
│   │   ├── insights.rs
│   │   ├── feature.rs
│   │   └── family.rs
│   │
│   ├── features/                 # Feature flag system
│   │   ├── mod.rs
│   │   ├── registry.rs
│   │   ├── guard.rs
│   │   ├── definitions/
│   │   │   ├── mod.rs
│   │   │   ├── core.rs
│   │   │   ├── finance.rs
│   │   │   ├── investment.rs
│   │   │   ├── trading.rs
│   │   │   └── integrations.rs
│   │   └── middleware.rs
│   │
│   ├── parsers/                  # Bank statement parsers
│   │   ├── mod.rs
│   │   ├── base.rs
│   │   ├── generic_csv.rs
│   │   ├── chase.rs
│   │   ├── bank_of_america.rs
│   │   └── registry.rs
│   │
│   ├── middleware/               # Custom middleware
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── feature_gate.rs
│   │   ├── permission.rs
│   │   └── rate_limit.rs
│   │
│   ├── workers/                  # Background jobs
│   │   ├── mod.rs
│   │   ├── statement_processor.rs
│   │   ├── email_importer.rs
│   │   └── analytics_calculator.rs
│   │
│   ├── mailers/                  # Email templates
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── family.rs
│   │
│   └── views/                    # Response views/serializers
│       ├── mod.rs
│       ├── user.rs
│       ├── account.rs
│       ├── transaction.rs
│       └── feature.rs
│
├── migration/                    # SeaORM migrations
│   └── src/
│       ├── lib.rs
│       ├── m20240101_000001_create_users.rs
│       ├── m20240101_000002_create_accounts.rs
│       ├── m20240101_000003_create_transactions.rs
│       ├── m20240101_000004_create_features.rs
│       └── ...
│
├── tests/                        # Integration tests
│   ├── mod.rs
│   ├── requests/
│   │   ├── auth.rs
│   │   ├── accounts.rs
│   │   ├── transactions.rs
│   │   └── features.rs
│   └── fixtures/
│
├── frontend/                     # React application
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── tailwind.config.js
│   ├── index.html
│   │
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   │
│   │   ├── components/           # React components
│   │   │   ├── ui/               # shadcn/ui components
│   │   │   ├── layout/
│   │   │   │   ├── Header.tsx
│   │   │   │   ├── Sidebar.tsx
│   │   │   │   └── Layout.tsx
│   │   │   ├── features/
│   │   │   │   ├── FeatureGate.tsx
│   │   │   │   ├── FeatureSettings.tsx
│   │   │   │   └── FeatureCard.tsx
│   │   │   ├── accounts/
│   │   │   ├── transactions/
│   │   │   ├── insights/
│   │   │   ├── portfolio/        # Investment feature
│   │   │   └── trading/          # Algo trading feature
│   │   │
│   │   ├── routes/               # TanStack Router
│   │   │   ├── index.tsx
│   │   │   ├── __root.tsx
│   │   │   ├── dashboard.tsx
│   │   │   ├── accounts.tsx
│   │   │   ├── transactions.tsx
│   │   │   ├── insights.tsx
│   │   │   ├── settings/
│   │   │   │   ├── index.tsx
│   │   │   │   ├── features.tsx
│   │   │   │   └── profile.tsx
│   │   │   ├── portfolio/        # Conditional route
│   │   │   └── trading/          # Conditional route
│   │   │
│   │   ├── lib/                  # Utilities
│   │   │   ├── api.ts
│   │   │   ├── utils.ts
│   │   │   └── constants.ts
│   │   │
│   │   ├── stores/               # Zustand stores
│   │   │   ├── auth.ts
│   │   │   ├── ui.ts
│   │   │   └── features.ts
│   │   │
│   │   ├── hooks/                # Custom hooks
│   │   │   ├── useAuth.ts
│   │   │   ├── useFeatures.ts
│   │   │   └── usePermissions.ts
│   │   │
│   │   └── types/                # TypeScript types
│   │       ├── index.ts
│   │       ├── user.ts
│   │       ├── account.ts
│   │       ├── transaction.ts
│   │       └── feature.ts
│   │
│   └── public/
│       └── assets/
│
├── assets/                       # Static assets for Loco
│   └── static/                   # Built React app copied here
│
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
│
└── docs/
    ├── README.md
    ├── PRODUCT_SPEC_V2.md        # This file
    ├── AUTH_DECISION.md
    ├── UI_REQUIREMENTS.md
    └── API.md
```

---

## Database Schema

### Core Tables

```sql
-- Users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255),

    -- TOTP
    totp_secret VARCHAR(64),
    totp_enabled BOOLEAN DEFAULT FALSE,
    backup_codes JSONB,

    -- Settings
    primary_currency VARCHAR(3) DEFAULT 'USD',
    timezone VARCHAR(50) DEFAULT 'UTC',
    locale VARCHAR(10) DEFAULT 'en-US',

    -- Family
    family_id UUID REFERENCES families(id),
    role_id UUID REFERENCES roles(id),
    is_family_owner BOOLEAN DEFAULT FALSE,

    -- Metadata
    email_verified_at TIMESTAMPTZ,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Families
CREATE TABLE families (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL,
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Sessions (for refresh tokens)
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash VARCHAR(255) NOT NULL,
    device_name VARCHAR(255),
    device_type VARCHAR(50),
    ip_address VARCHAR(45),
    user_agent TEXT,
    last_used_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    is_revoked BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Feature flags (see Feature Flag System section above)
```

### Personal Finance Tables

```sql
-- Bank Accounts
CREATE TABLE bank_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Account details
    bank_name VARCHAR(255) NOT NULL,
    account_name VARCHAR(255),
    account_type VARCHAR(50) NOT NULL, -- checking, savings, credit, investment
    account_number_masked VARCHAR(20),
    currency VARCHAR(3) DEFAULT 'USD',

    -- Status
    current_balance DECIMAL(15, 2),
    balance_updated_at TIMESTAMPTZ,
    opening_date DATE,
    closing_date DATE,
    is_active BOOLEAN DEFAULT TRUE,

    -- Sharing
    is_shared BOOLEAN DEFAULT FALSE,

    -- Metadata
    color VARCHAR(7), -- Hex color for UI
    icon VARCHAR(50),
    notes TEXT,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Statements
CREATE TABLE statements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES bank_accounts(id) ON DELETE CASCADE,

    -- File info
    file_name VARCHAR(255) NOT NULL,
    file_path VARCHAR(512) NOT NULL,
    file_size INTEGER,
    file_hash VARCHAR(64), -- For duplicate detection

    -- Processing
    parser_type VARCHAR(100) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending', -- pending, processing, completed, failed
    error_message TEXT,

    -- Period
    period_start DATE,
    period_end DATE,

    -- Stats
    transaction_count INTEGER DEFAULT 0,
    import_count INTEGER DEFAULT 0,
    duplicate_count INTEGER DEFAULT 0,

    -- Metadata
    uploaded_by UUID REFERENCES users(id),
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Transactions
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES bank_accounts(id) ON DELETE CASCADE,
    statement_id UUID REFERENCES statements(id) ON DELETE SET NULL,

    -- Transaction details
    transaction_date DATE NOT NULL,
    posted_date DATE,
    description TEXT NOT NULL,
    original_description TEXT, -- Before normalization

    -- Amount
    amount DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL,

    -- Categorization
    category_id UUID REFERENCES categories(id),
    tags TEXT[],
    notes TEXT,

    -- Status
    is_pending BOOLEAN DEFAULT FALSE,
    is_duplicate BOOLEAN DEFAULT FALSE,
    is_excluded BOOLEAN DEFAULT FALSE, -- Exclude from insights

    -- External reference
    external_id VARCHAR(255), -- Bank's transaction ID

    -- Metadata
    source VARCHAR(50) DEFAULT 'import', -- import, manual, api
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(account_id, external_id) -- Prevent duplicates
);

-- Categories
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    parent_id UUID REFERENCES categories(id),

    -- Ownership
    user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- NULL for system categories
    family_id UUID REFERENCES families(id) ON DELETE CASCADE,
    is_system BOOLEAN DEFAULT FALSE,

    -- Display
    color VARCHAR(7),
    icon VARCHAR(50),

    -- Categorization rules
    keywords TEXT[], -- For auto-categorization

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(slug, user_id, family_id)
);

-- Budgets (feature: budgeting)
CREATE TABLE budgets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    family_id UUID REFERENCES families(id),

    name VARCHAR(255) NOT NULL,
    category_id UUID REFERENCES categories(id),

    -- Budget configuration
    amount DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    period_type VARCHAR(20) NOT NULL, -- monthly, weekly, yearly, custom

    -- Custom period
    start_date DATE,
    end_date DATE,

    -- Alerts
    alert_threshold INTEGER DEFAULT 80, -- Percentage
    alert_enabled BOOLEAN DEFAULT TRUE,

    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Investment Tables (feature: investment_portfolio)

```sql
-- Investment Accounts
CREATE TABLE investment_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    name VARCHAR(255) NOT NULL,
    account_type VARCHAR(50) NOT NULL, -- brokerage, ira, 401k, crypto_exchange
    institution VARCHAR(255),
    account_number_masked VARCHAR(20),
    currency VARCHAR(3) DEFAULT 'USD',

    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Holdings
CREATE TABLE holdings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES investment_accounts(id) ON DELETE CASCADE,

    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(255),
    asset_type VARCHAR(50) NOT NULL, -- stock, etf, bond, mutual_fund, crypto, cash

    quantity DECIMAL(20, 8) NOT NULL,
    cost_basis DECIMAL(15, 2),
    current_price DECIMAL(15, 4),
    current_value DECIMAL(15, 2),

    price_updated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(account_id, symbol)
);

-- Investment Transactions
CREATE TABLE investment_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES investment_accounts(id) ON DELETE CASCADE,
    holding_id UUID REFERENCES holdings(id),

    transaction_type VARCHAR(50) NOT NULL, -- buy, sell, dividend, split, transfer
    transaction_date DATE NOT NULL,

    symbol VARCHAR(20),
    quantity DECIMAL(20, 8),
    price DECIMAL(15, 4),
    amount DECIMAL(15, 2) NOT NULL,
    fees DECIMAL(15, 2) DEFAULT 0,

    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Watchlist
CREATE TABLE watchlist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(255),
    asset_type VARCHAR(50),
    notes TEXT,

    created_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(user_id, symbol)
);
```

### Trading Tables (feature: algo_trading)

```sql
-- Trading Strategies
CREATE TABLE trading_strategies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Strategy configuration
    strategy_type VARCHAR(50) NOT NULL, -- custom, template
    config JSONB NOT NULL, -- Strategy parameters

    -- Execution settings
    is_active BOOLEAN DEFAULT FALSE,
    is_paper_trading BOOLEAN DEFAULT TRUE, -- Paper vs live trading
    max_position_size DECIMAL(15, 2),
    stop_loss_percent DECIMAL(5, 2),
    take_profit_percent DECIMAL(5, 2),

    -- Stats
    total_trades INTEGER DEFAULT 0,
    winning_trades INTEGER DEFAULT 0,
    total_pnl DECIMAL(15, 2) DEFAULT 0,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Strategy Signals
CREATE TABLE trading_signals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES trading_strategies(id) ON DELETE CASCADE,

    signal_type VARCHAR(20) NOT NULL, -- buy, sell, hold
    symbol VARCHAR(20) NOT NULL,

    -- Signal details
    price_at_signal DECIMAL(15, 4),
    quantity_suggested DECIMAL(20, 8),
    confidence DECIMAL(5, 2), -- 0-100

    -- Execution
    executed BOOLEAN DEFAULT FALSE,
    executed_at TIMESTAMPTZ,
    execution_price DECIMAL(15, 4),

    -- Analysis
    reasoning JSONB, -- Why signal was generated

    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Backtests
CREATE TABLE backtests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES trading_strategies(id) ON DELETE CASCADE,

    -- Test period
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,

    -- Initial conditions
    initial_capital DECIMAL(15, 2) NOT NULL,

    -- Results
    final_value DECIMAL(15, 2),
    total_return DECIMAL(10, 4),
    sharpe_ratio DECIMAL(10, 4),
    max_drawdown DECIMAL(10, 4),
    total_trades INTEGER,
    winning_trades INTEGER,

    -- Detailed results
    results_data JSONB, -- Full backtest data

    status VARCHAR(20) DEFAULT 'pending', -- pending, running, completed, failed
    error_message TEXT,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);
```

---

## API Endpoints

### Authentication

```
POST /api/v1/auth/register          # Register new user
POST /api/v1/auth/login             # Login
POST /api/v1/auth/logout            # Logout
POST /api/v1/auth/refresh           # Refresh tokens
GET  /api/v1/auth/me                # Get current user
POST /api/v1/auth/forgot-password   # Request password reset
POST /api/v1/auth/reset-password    # Reset password

# TOTP/2FA
POST /api/v1/auth/totp/setup        # Generate TOTP secret
POST /api/v1/auth/totp/verify       # Verify TOTP code
POST /api/v1/auth/totp/disable      # Disable TOTP
GET  /api/v1/auth/backup-codes      # Get backup codes
POST /api/v1/auth/backup-codes/regenerate
```

### Features

```
GET  /api/v1/features               # List all features
GET  /api/v1/features/:id           # Get feature details
GET  /api/v1/user/features          # Get user's feature flags
POST /api/v1/user/features/:id/enable
POST /api/v1/user/features/:id/disable
PUT  /api/v1/user/features/:id/config
```

### Personal Finance (feature: personal_finance)

```
# Accounts
GET    /api/v1/accounts             # List accounts
POST   /api/v1/accounts             # Create account
GET    /api/v1/accounts/:id         # Get account
PUT    /api/v1/accounts/:id         # Update account
DELETE /api/v1/accounts/:id         # Delete account
GET    /api/v1/accounts/:id/balance # Get balance history

# Transactions
GET    /api/v1/transactions         # List transactions (with filters)
POST   /api/v1/transactions         # Create transaction
GET    /api/v1/transactions/:id     # Get transaction
PUT    /api/v1/transactions/:id     # Update transaction
DELETE /api/v1/transactions/:id     # Delete transaction
POST   /api/v1/transactions/bulk    # Bulk operations

# Statements
GET    /api/v1/statements           # List statements
POST   /api/v1/statements/upload    # Upload statement
GET    /api/v1/statements/:id       # Get statement details
DELETE /api/v1/statements/:id       # Delete statement
POST   /api/v1/statements/:id/reprocess

# Categories
GET    /api/v1/categories           # List categories
POST   /api/v1/categories           # Create category
PUT    /api/v1/categories/:id       # Update category
DELETE /api/v1/categories/:id       # Delete category
```

### Insights (feature: insights)

```
GET /api/v1/insights/summary        # Dashboard summary
GET /api/v1/insights/spending       # Spending by category
GET /api/v1/insights/trends         # Spending trends
GET /api/v1/insights/income-expense # Income vs expense
GET /api/v1/insights/merchants      # Top merchants
GET /api/v1/insights/forecast       # Spending forecast
```

### Budgets (feature: budgeting)

```
GET    /api/v1/budgets              # List budgets
POST   /api/v1/budgets              # Create budget
GET    /api/v1/budgets/:id          # Get budget with progress
PUT    /api/v1/budgets/:id          # Update budget
DELETE /api/v1/budgets/:id          # Delete budget
GET    /api/v1/budgets/progress     # All budgets progress
```

### Investment Portfolio (feature: investment_portfolio)

```
# Accounts
GET    /api/v1/investments/accounts
POST   /api/v1/investments/accounts
GET    /api/v1/investments/accounts/:id
PUT    /api/v1/investments/accounts/:id
DELETE /api/v1/investments/accounts/:id

# Holdings
GET    /api/v1/investments/holdings
POST   /api/v1/investments/holdings
PUT    /api/v1/investments/holdings/:id
DELETE /api/v1/investments/holdings/:id
POST   /api/v1/investments/holdings/refresh-prices

# Transactions
GET    /api/v1/investments/transactions
POST   /api/v1/investments/transactions
GET    /api/v1/investments/transactions/:id

# Portfolio
GET    /api/v1/investments/portfolio/summary
GET    /api/v1/investments/portfolio/allocation
GET    /api/v1/investments/portfolio/performance

# Watchlist
GET    /api/v1/investments/watchlist
POST   /api/v1/investments/watchlist
DELETE /api/v1/investments/watchlist/:symbol
```

### Algo Trading (feature: algo_trading)

```
# Strategies
GET    /api/v1/trading/strategies
POST   /api/v1/trading/strategies
GET    /api/v1/trading/strategies/:id
PUT    /api/v1/trading/strategies/:id
DELETE /api/v1/trading/strategies/:id
POST   /api/v1/trading/strategies/:id/activate
POST   /api/v1/trading/strategies/:id/deactivate

# Signals
GET    /api/v1/trading/signals
GET    /api/v1/trading/strategies/:id/signals

# Backtests
POST   /api/v1/trading/strategies/:id/backtest
GET    /api/v1/trading/backtests/:id
GET    /api/v1/trading/strategies/:id/backtests
```

### Family (feature: family_sharing)

```
GET    /api/v1/family               # Get family info
POST   /api/v1/family               # Create family
PUT    /api/v1/family               # Update family

GET    /api/v1/family/members       # List members
POST   /api/v1/family/invite        # Invite member
DELETE /api/v1/family/members/:id   # Remove member
PUT    /api/v1/family/members/:id/role

GET    /api/v1/family/roles         # List roles
POST   /api/v1/family/roles         # Create custom role
PUT    /api/v1/family/roles/:id     # Update role
DELETE /api/v1/family/roles/:id     # Delete role
```

---

## Development Phases

### Phase 1: Foundation (MVP)
**Goal**: Core app with personal finance and feature flag system

**Tasks**:
1. **Project Setup**
   - Initialize Loco project
   - Set up database (PostgreSQL + SQLite support)
   - Configure SeaORM migrations
   - Set up React frontend with Vite

2. **Authentication**
   - User registration/login
   - JWT tokens with refresh
   - Password hashing (Argon2id)
   - Basic session management

3. **Feature Flag System**
   - Feature registry
   - Feature definitions table
   - User feature flags
   - Feature guard middleware
   - Feature settings UI

4. **Personal Finance Core**
   - Bank accounts CRUD
   - Transaction management
   - Statement upload
   - Generic CSV parser
   - Basic categorization

5. **Frontend Core**
   - Layout (sidebar, header)
   - Dashboard
   - Accounts page
   - Transactions page
   - Settings with features UI

### Phase 2: Enhanced Features
**Goal**: Full personal finance + family support

**Tasks**:
1. **Enhanced Auth**
   - TOTP/2FA
   - Session management UI
   - Backup codes

2. **Family Sharing**
   - Family creation
   - Member invitations
   - Role-based permissions
   - Shared accounts

3. **Insights**
   - Spending analytics
   - Charts and visualizations
   - Trends over time

4. **Budgeting**
   - Budget CRUD
   - Progress tracking
   - Alerts

5. **Additional Parsers**
   - Bank-specific parsers
   - PDF parsing

### Phase 3: Investment & Trading
**Goal**: Investment portfolio + algo trading foundation

**Tasks**:
1. **Investment Portfolio**
   - Investment accounts
   - Holdings tracking
   - Price fetching
   - Portfolio analytics

2. **Algo Trading**
   - Strategy builder
   - Paper trading
   - Backtesting
   - Signal generation

3. **Integrations**
   - Gmail integration
   - Exchange rate API
   - Stock price API

### Phase 4: Production
**Goal**: Production-ready deployment

**Tasks**:
1. Performance optimization
2. Security audit
3. Docker deployment
4. CI/CD pipeline
5. Monitoring setup
6. Documentation

---

## Configuration

### Loco Configuration

```yaml
# config/development.yaml
logger:
  enable: true
  level: debug
  format: compact

server:
  port: 3000
  host: 0.0.0.0

database:
  uri: postgres://localhost:5432/finn_lens_dev
  enable_logging: true
  min_connections: 1
  max_connections: 10
  connect_timeout: 5000
  idle_timeout: 10000

auth:
  jwt:
    secret: dev-secret-key-change-in-production
    expiration: 900  # 15 minutes
  refresh_token:
    expiration: 2592000  # 30 days

features:
  default_enabled:
    - personal_finance
    - insights
    - custom_categories
```

```yaml
# config/production.yaml
logger:
  enable: true
  level: info
  format: json

server:
  port: {{ get_env(name="PORT", default="3000") }}
  host: 0.0.0.0

database:
  uri: {{ get_env(name="DATABASE_URL") }}
  enable_logging: false
  min_connections: 5
  max_connections: 50

auth:
  jwt:
    secret: {{ get_env(name="JWT_SECRET") }}
    expiration: 900
  refresh_token:
    expiration: 2592000

# Serve static files (built React app)
static:
  enable: true
  folder: assets/static
  fallback: index.html
```

---

## Deployment

### Single Binary Deployment

```bash
# Build frontend
cd frontend && npm run build

# Copy built files to Loco assets
cp -r frontend/dist/* assets/static/

# Build Rust binary (release)
cargo build --release

# Run
./target/release/finn-lens start
```

### Docker Deployment

```dockerfile
# Dockerfile
FROM rust:1.75 AS builder

WORKDIR /app
COPY . .

# Build frontend
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    cd frontend && npm ci && npm run build && \
    cp -r dist/* ../assets/static/

# Build Rust
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/finn-lens /usr/local/bin/
COPY --from=builder /app/config /app/config
COPY --from=builder /app/assets /app/assets

WORKDIR /app
ENV RUST_LOG=info
EXPOSE 3000

CMD ["finn-lens", "start", "-e", "production"]
```

```yaml
# docker-compose.yml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgres://finn:finn@db:5432/finn_lens
      - JWT_SECRET=${JWT_SECRET}
      - RUST_LOG=info
    depends_on:
      - db

  db:
    image: postgres:16
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_USER=finn
      - POSTGRES_PASSWORD=finn
      - POSTGRES_DB=finn_lens

volumes:
  postgres_data:
```

---

## Success Metrics

### Phase 1
- [ ] User can register, login, logout
- [ ] Feature flag system works
- [ ] User can enable/disable features
- [ ] Bank accounts CRUD works
- [ ] Statement upload and parsing works
- [ ] Transactions display correctly

### Phase 2
- [ ] TOTP/2FA works
- [ ] Family sharing works
- [ ] Insights dashboard shows analytics
- [ ] Budgets work with alerts

### Phase 3
- [ ] Investment portfolio tracking works
- [ ] Algo trading strategies work
- [ ] Backtesting produces results
- [ ] Paper trading simulates correctly

### Phase 4
- [ ] Single binary deployment works
- [ ] Docker deployment works
- [ ] Performance meets targets
- [ ] Security audit passed

---

## Future Enhancements

1. **Mobile Apps**: React Native or Flutter
2. **AI Insights**: ML-based spending predictions
3. **Real-time Data**: WebSocket for live prices
4. **Plaid Integration**: Direct bank connections
5. **Multi-language**: i18n support
6. **Public API**: For third-party integrations
7. **Plugin System**: Custom feature development
