# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

finn-lens is a modular personal finance platform with a Rust backend (Loco framework) and React frontend. It features an extensible feature flag system for modules like Personal Finance, Investment Portfolio, and Algo Trading.

## Development Commands

### Backend (Rust/Loco)

```bash
# Start the backend server (port 5150)
cargo loco start

# Run database migrations
cargo loco db migrate

# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests in a specific file
cargo test --test parsers_integration

# Generate SeaORM entities from database
cargo loco db entities

# Seed feature definitions
cargo loco task seed_features
```

### Frontend (React/Vite)

```bash
cd frontend

# Start dev server with hot reload (port 5173, proxies /api to backend)
npm run dev

# Build for production
npm run build

# Lint
npm run lint

# Generate TanStack Router routes
npm run routes
```

### Development Workflow

Run backend and frontend in separate terminals for hot-reloading:
- Terminal 1: `cargo loco start`
- Terminal 2: `cd frontend && npm run dev`

Access frontend at http://localhost:5173, API at http://localhost:5150/api

## Architecture

### Backend Structure (src/)

- **controllers/**: API route handlers - auth, accounts, transactions, categories, statements, features, insights
- **models/**: SeaORM entities and business logic
  - `_entities/`: Auto-generated SeaORM entity definitions
  - Model files contain ActiveModel implementations and domain logic
- **parsers/**: Bank statement parsing system
  - `banks/`: Bank-specific parsers
  - `formats/`: File format handlers (Excel, CSV)
  - `detector.rs`: Auto-detects bank format from file content
  - `registry.rs`: Parser registration and lookup
- **app.rs**: Loco app configuration, routes registration, hooks

### Frontend Structure (frontend/src/)

- **routes/**: TanStack Router file-based routing
  - `__root.tsx`: Root layout
  - `_authenticated/`: Protected routes requiring auth
- **stores/**: Zustand state management
  - `auth-store.ts`: Authentication state
  - `feature-store.ts`: Feature flags
  - `ui-store.ts`: UI state
- **components/**: React components organized by domain (accounts, auth, layout, ui)
- **lib/**: Utilities (uses `@/` path alias)

### Database

- SQLite for development (`finn_lens.db`)
- PostgreSQL for production
- Migrations in `migration/src/` (SeaORM)
- Auto-migrate enabled in development

### Key Patterns

- **Feature Flags**: Users can enable/disable modules. Check `user_feature_flags` table and `feature_definitions`.
- **Statement Parsing**: Pluggable parser system in `src/parsers/`. Add new banks by implementing `BankParser` trait.
- **API Routes**: All under `/api` prefix. Frontend Vite config proxies `/api` to backend.
- **Auth**: JWT-based with tokens stored in Zustand. Uses Argon2id for password hashing.

## Configuration

- Backend config: `config/development.yaml`, `config/production.yaml`
- Environment variables: `.env` (see `.env.example`)
- Frontend Vite config: `frontend/vite.config.ts`

## Testing

Backend tests are in `tests/` directory:
- `tests/parsers_integration.rs`: Parser tests
- `tests/requests/`: API request tests
- `tests/models/`: Model tests
