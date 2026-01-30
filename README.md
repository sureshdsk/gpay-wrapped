# finn-lens v2

A modular personal finance platform built with **Rust (Loco framework)** and **React**. Features an extensible **feature flag system** allowing users to enable/disable modules like Personal Finance, Investment Portfolio Management, and Algo Trading.

## Documentation

### Core Specifications

1. **[PRODUCT_SPEC_V2.md](./PRODUCT_SPEC_V2.md)** - Complete v2 product specification
   - Loco (Rust) backend architecture
   - Feature flag system design
   - Database schema (PostgreSQL/SQLite)
   - API endpoints
   - Implementation phases
   - Deployment configuration

2. **[PRODUCT_SPEC.md](./PRODUCT_SPEC.md)** - Original specification (reference)
   - Features and user flows
   - Original system architecture
   - Detailed implementation tasks

3. **[AUTH_DECISION.md](./AUTH_DECISION.md)** - Authentication & Security Architecture
   - JWT implementation (adapted for Rust)
   - Token architecture (access + refresh tokens)
   - Password security (Argon2id)
   - TOTP/2FA implementation
   - Session management
   - Permission system integration

4. **[UI_REQUIREMENTS_V2.md](./UI_REQUIREMENTS_V2.md)** - Phased UI/UX Requirements
   - Phase 1: Core UI (Auth, Dashboard, Accounts, Transactions, Features)
   - Phase 2: Enhanced UI (Family, Insights, Budgeting, 2FA)
   - Phase 3: Investment & Trading UI
   - Phase 4: Polish & Production
   - Design system, components, responsive design, accessibility

5. **[UI_REQUIREMENTS.md](./UI_REQUIREMENTS.md)** - Original UI specification (reference)

6. **[FRONTEND_ARCHITECTURE.md](./FRONTEND_ARCHITECTURE.md)** - Frontend Technical Architecture
   - TanStack ecosystem (Router, Query, Table, Virtual)
   - Zustand state management with Immer
   - shadcn/ui + Radix UI + Tailwind CSS
   - React Hook Form + Zod validation
   - Project structure, configurations, code examples

## Technology Stack

### Backend (v2 - Rust)
- **Framework**: [Loco](https://loco.rs) (Rust web framework)
- **Database**: PostgreSQL (production) / SQLite (development)
- **ORM**: SeaORM
- **Authentication**: Custom JWT with Argon2id
- **Background Jobs**: Loco workers
- **Validation**: validator crate

### Frontend
- **Framework**: React 18 + TypeScript 5.4 + Vite 5.4
- **Routing**: TanStack Router (type-safe, file-based)
- **Data Fetching**: TanStack Query v5 (caching, prefetching)
- **Tables**: TanStack Table v8 (sorting, filtering, pagination)
- **Virtualization**: TanStack Virtual (long lists)
- **State**: Zustand v4 + Immer (client state)
- **UI Components**: shadcn/ui + Radix UI + Tailwind CSS
- **Forms**: React Hook Form + Zod validation
- **Charts**: Recharts
- **Icons**: Lucide React
- **Notifications**: Sonner (toast)

### Deployment
- **Single Binary**: Loco serves embedded React static files
- **Containerization**: Docker + Docker Compose
- **Daemon Mode**: systemd (Linux) / launchd (macOS)

## Key Features

### Core
- 🔐 **Secure Authentication** - JWT + optional TOTP/2FA
- 🎛️ **Feature Flags** - Enable/disable modules per user
- 👨‍👩‍👧‍👦 **Family Support** - Multi-user with customizable roles & permissions

### Personal Finance (Module)
- 🏦 **Multi-Bank** - Support multiple bank accounts per user
- 💱 **Multi-Currency** - Track finances across different currencies
- 📊 **Financial Insights** - Spending analytics, trends, reports
- 📄 **Statement Parsing** - Pluggable parser system for different bank formats
- 🎯 **Budgeting** - Track spending against budgets

### Investment Portfolio (Module - Future)
- 📈 **Portfolio Tracking** - Stocks, bonds, crypto, mutual funds
- 📉 **Performance Analytics** - Returns, allocation, benchmarks
- 👁️ **Watchlist** - Track potential investments

### Algo Trading (Module - Future)
- 🤖 **Strategy Builder** - Create automated trading strategies
- 📊 **Backtesting** - Test strategies against historical data
- 📝 **Paper Trading** - Simulate without real money

## Project Structure

```
finn-lens/
├── Cargo.toml              # Rust workspace
├── config/                 # Loco configuration
│   ├── development.yaml
│   └── production.yaml
├── src/                    # Rust backend
│   ├── controllers/        # API route handlers
│   ├── models/            # SeaORM entities
│   ├── services/          # Business logic
│   ├── features/          # Feature flag system
│   │   ├── registry.rs    # Feature registry
│   │   ├── guard.rs       # Feature middleware
│   │   └── definitions/   # Feature definitions
│   ├── parsers/           # Bank statement parsers
│   ├── middleware/        # Auth, permissions
│   └── workers/           # Background jobs
├── migration/             # SeaORM migrations
├── frontend/              # React application
│   ├── src/
│   │   ├── components/
│   │   │   ├── features/  # Feature-gated components
│   │   │   ├── accounts/
│   │   │   ├── portfolio/ # Investment module
│   │   │   └── trading/   # Algo trading module
│   │   ├── routes/        # TanStack Router
│   │   ├── hooks/
│   │   │   └── useFeatures.ts
│   │   └── stores/
│   └── package.json
├── assets/static/         # Built React app (embedded)
└── docker/
```

## Getting Started

### Prerequisites
- Rust 1.75+ (with cargo)
- Node.js 20+
- PostgreSQL 15+ (or SQLite for development)
- Docker (optional)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/finn-lens.git
   cd finn-lens
   ```

2. **Install Loco CLI**
   ```bash
   cargo install loco-cli
   ```

3. **Database setup**
   ```bash
   # PostgreSQL
   createdb finn_lens_dev

   # Or use SQLite (update config/development.yaml)
   # database:
   #   uri: sqlite://finn_lens.db?mode=rwc
   ```

4. **Run migrations**
   ```bash
   cargo loco db migrate
   ```

5. **Seed feature definitions**
   ```bash
   cargo loco task seed_features
   ```

6. **Frontend setup**
   ```bash
   cd frontend
   npm install
   npm run build
   cp -r dist/* ../assets/static/
   ```

7. **Start the application**
   ```bash
   cargo loco start
   ```

8. **Access the application**
   - Application: http://localhost:3000
   - API: http://localhost:3000/api/v1

### Development Mode

Run backend and frontend separately for hot-reloading:

```bash
# Terminal 1: Backend
cargo loco start

# Terminal 2: Frontend (Vite dev server)
cd frontend
npm run dev
```

Configure Vite to proxy API requests to Loco backend.

### Docker Deployment

```bash
docker-compose up -d
```

## Development Phases

### Phase 1: Foundation (MVP)
- Loco project setup with SeaORM
- Feature flag system implementation
- User authentication (JWT + Argon2id)
- Personal finance core (accounts, transactions, statements)
- React frontend with feature settings UI

### Phase 2: Enhanced Features
- TOTP/2FA authentication
- Family sharing with RBAC
- Financial insights & analytics
- Budgeting system
- Additional bank parsers

### Phase 3: Investment & Trading
- Investment portfolio tracking
- Portfolio analytics
- Algo trading foundation
- Backtesting system

### Phase 4: Production
- Performance optimization
- Security audit
- Docker & single-binary deployment
- CI/CD pipeline
- Monitoring & logging

## Security

finn-lens takes security seriously:

- ✅ Argon2id password hashing
- ✅ JWT access + refresh tokens with rotation
- ✅ HttpOnly cookies (XSS protection)
- ✅ Rate limiting (brute force prevention)
- ✅ TOTP/2FA with backup codes
- ✅ Session management (revocable)
- ✅ Audit logging
- ✅ Security headers (CSP, HSTS, etc.)
- ✅ Input validation (validator crate)
- ✅ SQL injection prevention (SeaORM)
- ✅ Rust memory safety

See [AUTH_DECISION.md](./AUTH_DECISION.md) for complete security architecture.

## Feature Flag System

Users can enable/disable features from Settings:

| Feature | Category | Default |
|---------|----------|---------|
| Personal Finance | Finance | Enabled |
| Family Sharing | Finance | Disabled |
| Budgeting | Finance | Disabled |
| Financial Insights | Analytics | Enabled |
| Investment Portfolio | Investment | Disabled |
| Algo Trading | Trading | Disabled |

Features have dependencies (e.g., Algo Trading requires Investment Portfolio).

See [PRODUCT_SPEC_V2.md](./PRODUCT_SPEC_V2.md) for complete feature flag documentation.

## Contributing

Contributions are welcome! Please read the contributing guidelines before submitting PRs.

## License

[Choose License - MIT/Apache 2.0/GPL]

## Support

- Documentation: See `docs/` folder
- Issues: GitHub Issues
- Discussions: GitHub Discussions
