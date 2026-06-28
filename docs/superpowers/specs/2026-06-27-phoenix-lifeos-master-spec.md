# Phoenix LifeOS — Master Engineering Specification

**Status:** Foundation Phase COMPLETE — Feature modules 7–12 also collected. Ready for implementation.
**Remaining Sections:** 7–37 (feature modules, received during build phase)
**Last Updated:** 2026-06-27

---

## Sections Collected

- [x] Section 1 — Master System Directive
- [x] Section 2 — Enterprise Software Architecture
- [x] Section 3 — Repository + Project Bootstrap Structure
- [x] Section 4 — Rust Core Architecture Implementation (Domain + Services + Events)
- [x] Section 5 — Frontend Life Hub + Dashboard Architecture
- [x] Section 6 — Database Design + Migration System
- [ ] Section 7 — Authentication & Security
- [ ] Section 8 — AI Architecture
- [ ] Section 9 — Life Hub
- [ ] Section 10 — Finance Engine
- [ ] Section 11 — Credit Engine
- [ ] Section 12 — Budget Engine
- [ ] Section 13 — Debt Engine
- [ ] Section 14 — Mortgage Planner
- [ ] Section 15 — Vehicle Planner
- [ ] Section 16 — Investments
- [ ] Section 17 — Retirement
- [ ] Section 18 — Tax Estimator
- [ ] Section 19 — Business Finance
- [ ] Section 20 — Home Maintenance
- [ ] Section 21 — Vehicle Maintenance
- [ ] Section 22 — Document Vault
- [ ] Section 23 — OCR
- [ ] Section 24 — Email Receipt Import
- [ ] Section 25 — Password Vault
- [ ] Section 26 — Personal CRM
- [ ] Section 27 — Automation Engine
- [ ] Section 28 — Voice Assistant
- [ ] Section 29 — Local AI
- [ ] Section 30 — Plugin SDK
- [ ] Section 31 — Secure Synchronization
- [ ] Section 32 — Reporting Engine
- [ ] Section 33 — UI Design System
- [ ] Section 34 — Testing
- [ ] Section 35 — CI/CD
- [ ] Section 36 — Deployment
- [ ] Section 37 — Future Roadmap

---

# SECTION 1 — MASTER SYSTEM DIRECTIVE

## Role

You are the Lead Principal Software Architect responsible for building Phoenix LifeOS, a production-quality personal life and financial operating system.

This project must be treated exactly like a commercial enterprise application.

Do not produce prototype-quality code.
Do not produce tutorial-quality code.
Do not generate placeholder implementations unless explicitly marked as temporary.

Every design decision should favor long-term maintainability, modularity, extensibility, security, testability, and performance.

The project should be capable of supporting many years of future development without requiring architectural rewrites.

---

## Development Philosophy

Build software that another senior engineer would enjoy maintaining.

Follow these principles throughout the project:

- SOLID
- DRY
- KISS where appropriate
- Clean Architecture
- Domain-Driven Design where beneficial
- Feature-first organization
- Strong typing
- Explicit interfaces
- Composition over inheritance
- Test-driven mindset
- Security by design
- Privacy by default

Avoid unnecessary complexity while designing for future growth.

---

## Project Vision

Phoenix LifeOS is a complete desktop operating system for a person's financial and organizational life.

The application should help users:

- Understand their finances
- Build wealth
- Improve credit
- Eliminate debt
- Buy a home
- Buy a vehicle
- Track investments
- Prepare for retirement
- Estimate taxes
- Organize important documents
- Maintain a home
- Maintain vehicles
- Manage business finances
- Secure passwords
- Manage relationships with trusted professionals
- Receive AI-powered planning assistance

Everything should work offline by default. Online services should always be optional. The user owns their data.

---

## Engineering Standards

Every pull request must satisfy:

- Clean compilation
- Zero warnings
- Passing unit tests
- Passing integration tests
- Passing UI tests
- Static analysis
- Documentation updates
- Security review
- Performance review

Never merge code that weakens architecture.

---

## Coding Standards

**Rust:**
- Idiomatic Rust
- No unnecessary cloning
- Proper ownership and lifetimes
- Comprehensive error handling
- Strong typing
- Async using Tokio
- SQLx, Serde, tracing

**Frontend:**
- React + TypeScript
- Functional components and hooks
- No class components
- Strict TypeScript
- Reusable UI
- Feature folders
- Strong form validation
- Accessibility-first

---

## Architecture Constraints

- Modular architecture; each module independently testable
- Modules communicate only through public interfaces
- No circular dependencies
- No business logic inside UI components
- No database logic inside presentation code

---

## AI Development Strategy

Build using parallel sub-agents with the following responsibilities:

| Agent | Responsibility |
|---|---|
| Architecture Agent | Project layout, architecture, module boundaries |
| Backend Agent | Rust services, repositories, database |
| Frontend Agent | React components, routing, UX |
| Security Agent | Encryption, authentication, secure storage, threat modeling |
| AI Agent | Ollama, OpenAI, Anthropic, Gemini, prompt routing, memory |
| Database Agent | Schema, migrations, indexing, optimization |
| Documentation Agent | Architecture docs, API docs, diagrams, README |
| QA Agent | Tests, coverage, regression testing |
| UX Agent | Layout, accessibility, themes, responsiveness |
| Finance Agent | Budgeting, debt, savings, investments |
| Life Hub Agent | Dashboard, tasks, goals, timeline |
| Business Agent | LLC, invoices, bookkeeping |
| Vault Agent | Encrypted documents, passwords, receipts, OCR |

**Integration Rules:** Only the Lead Architect Agent may approve merges between modules. Every merge must verify architecture consistency, database compatibility, UI consistency, security review, documentation updates, and passing tests.

---

## Quality Goals

- Startup time: < 2 seconds
- Memory: < 300 MB during normal operation
- Platforms: Fedora Linux (primary), Windows, macOS
- Offline-first, encrypted local database
- Automatic backups, graceful error recovery, crash-safe writes

---

## Security Requirements

- No plaintext secrets or passwords
- Encrypt sensitive user data at rest
- Secure password hashing (Argon2)
- Biometric authentication where available
- Hardware security key support in future versions
- No telemetry by default
- No tracking or analytics without explicit opt-in

---

## User Experience

Premium commercial software feel with:
- Smooth animations and fast transitions
- Responsive layouts, modern typography, professional iconography
- Keyboard shortcuts and command palette
- Global search and customizable dashboard
- Beautiful charts, dark and light mode, glassmorphism
- Full accessibility support

---

## Development Workflow

Build one complete module at a time. Each completed module must include: tests, documentation, database migrations, UI, backend, AI integration where appropriate, error handling, and logging. Do not leave unfinished modules.

---

# SECTION 2 — ENTERPRISE SOFTWARE ARCHITECTURE

## High-Level Architecture

```
┌───────────────────────────────────────────────┐
│              React UI Layer                   │
│         Components, Pages, Hooks              │
└───────────────────────────────────────────────┘
                    │
                    ▼
┌───────────────────────────────────────────────┐
│             Tauri IPC Layer                   │
│      Typed Commands & Event Bus               │
└───────────────────────────────────────────────┘
                    │
                    ▼
┌───────────────────────────────────────────────┐
│          Application Service Layer            │
│ Business Workflows & Use Cases                │
└───────────────────────────────────────────────┘
                    │
                    ▼
┌───────────────────────────────────────────────┐
│              Domain Layer                     │
│ Entities, Value Objects, Business Rules       │
└───────────────────────────────────────────────┘
                    │
                    ▼
┌───────────────────────────────────────────────┐
│          Infrastructure Layer                 │
│ SQLite, AI Providers, File Storage, Sync      │
└───────────────────────────────────────────────┘
```

**Layer Rules:**
- UI never directly accesses the database
- Business logic never lives in React components
- Database code never leaks into presentation
- Modules communicate through interfaces, not implementation details

---

## Rust Workspace Layout

```
phoenix-lifeos/
├── apps/
│   └── desktop/
├── crates/
│   ├── core/
│   ├── finance/
│   ├── credit/
│   ├── budgeting/
│   ├── debt/
│   ├── investments/
│   ├── retirement/
│   ├── tax/
│   ├── ai/
│   ├── sync/
│   ├── documents/
│   ├── vault/
│   ├── crm/
│   ├── automation/
│   ├── reports/
│   ├── notifications/
│   ├── security/
│   ├── settings/
│   ├── plugins/
│   ├── analytics/
│   └── shared/
```

Each crate: one responsibility, independently testable, minimal cross-crate dependencies.

---

## Frontend Structure

```
src/
├── app/
├── components/
├── layouts/
├── pages/
├── hooks/
├── stores/
├── features/
│   ├── budget/
│   ├── credit/
│   ├── investments/
│   ├── dashboard/
│   ├── lifehub/
│   ├── home/
│   ├── vehicle/
│   ├── vault/
│   ├── crm/
│   └── settings/
├── services/
├── routes/
├── contexts/
├── assets/
├── styles/
├── types/
├── utils/
├── lib/
├── generated/
└── tests/
```

Each feature contains: `components/`, `pages/`, `hooks/`, `api/`, `types/`, `store/`, `tests/`

---

## Module Boundaries

Every module owns its UI, Services, Models, Database Repository, Validation, and Tests.

No module may directly manipulate another module's database tables. Interaction occurs through public service interfaces only.

---

## Dependency Injection

Services are provided through explicit constructors or DI containers — no global state.

Core services: `FinanceService`, `BudgetService`, `CreditService`, `AIService`, `NotificationService`, `VaultService`

Services depend on interfaces, not concrete implementations.

---

## Database Layer

SQLx with SQLite. Requirements:
- Versioned migrations
- Foreign keys enabled
- WAL mode
- Prepared statements and transactions
- Indexed queries
- Repository abstraction with domain-specific methods
- Strong typing (no raw SQL leaking out)

---

## Application Services

Orchestrate business workflows. Each service: validates inputs, calls repositories, publishes events, handles transactions, returns domain models.

Examples: `CreateBudget()`, `ImportTransactions()`, `CalculateNetWorth()`, `AnalyzeCredit()`, `GenerateMonthlyReport()`, `SyncDevices()`, `EstimateMortgage()`

---

## Domain Layer

**Entities:** User, Account, Transaction, Budget, Goal, Loan, Investment, Vehicle, HomeAsset, Warranty, InsurancePolicy, Document, PasswordEntry, Contact

**Value Objects:** Money, Percentage, DateRange, Address, PhoneNumber, EmailAddress, VIN, CreditScore

**Business Rules:** Budget validation, debt calculations, interest calculations, credit utilization, retirement projections, tax calculations

Domain layer never depends on UI or database code.

---

## Event System

Internal event bus with typed events:

`TransactionCreated`, `BudgetExceeded`, `CreditUpdated`, `GoalReached`, `DocumentImported`, `ReceiptScanned`, `InsuranceExpired`, `VehicleServiceDue`, `PasswordCompromised`, `SyncCompleted`

Modules subscribe to events rather than calling each other directly where practical.

---

## AI Architecture

**Providers:** OpenAI, Anthropic, Gemini, Ollama, extensible to future providers

**Shared Interface:** Chat, Embeddings, Streaming, Vision, Tools, Memory, Context

**Components:** Prompt Builder, Conversation Manager, Knowledge Retrieval, Financial Analyzer, Document Analyzer

The rest of the application never depends on a specific AI provider.

---

## Plugin System

Every plugin implements: Metadata, Permissions, Settings, Commands, Events

**Lifecycle:** Install → Enable → Disable → Upgrade → Remove

Plugins are sandboxed where possible and use stable extension points.

---

## Synchronization Layer

Abstraction supporting: Local only, Encrypted backup, Encrypted device sync, Future cloud providers, Future self-hosted server

Features: Conflict resolution, incremental sync, offline queue

Sync logic stays separate from business modules.

---

## File Storage

Separate storage roots for: Database, Documents, Receipts, Images, OCR, Exports, Logs, Backups, Plugins, Temporary files

Never hardcode paths. Use platform-specific directories.

---

## Logging

Structured logging via `tracing`. Log all: database migrations, sync events, AI requests, import jobs, errors, security events, startup, shutdown. Configurable log levels.

---

## Configuration

Layered configuration: Defaults → Environment → User Settings → Feature Flags → Future enterprise config. Never hardcode secrets.

---

## Background Jobs

Scheduler for: Backups, Receipt processing, OCR, AI indexing, Reminder generation, Maintenance notifications, Sync, Report generation

Jobs survive application restarts where appropriate.

---

## Notification System

Central service supporting: Desktop notifications, In-app notifications, Reminders, Warnings, Informational messages, AI insights (future: email, SMS)

---

## Reporting Engine

Separate from UI. Formats: PDF, CSV, Excel, JSON, future plugins. Reports generated by dedicated services.

---

## Search System

Global full-text search across: Transactions, Contacts, Documents, Receipts, Budgets, Goals, Passwords, Properties, Vehicles, Investments, Insurance, Business records

Features: Filters, tags, saved searches, recent searches

---

## Command Palette

Global command palette with keyboard-first navigation.

Examples: Create transaction, Scan receipt, Open budget, Pay bill, Run AI analysis, Search documents, Generate report, Import CSV, Open settings

---

## State Management

Separate server state, UI state, feature state, form state, and cache state. Avoid one giant global store. Keep state local whenever practical.

---

## Error Handling

Error categories: Validation, Database, AI, Network, Filesystem, Authentication, Authorization, Synchronization, Unexpected

User-friendly messages with detailed logs preserved.

---

## Security Architecture

- Encrypt sensitive data at rest
- Argon2 password hashing
- OS keychain integration where available
- Never log secrets or PII
- Validate all IPC inputs
- Least-privilege design for plugins

Security review required when any new module is added.

---

## Performance Goals

- App startup < 2 seconds on typical desktop
- Responsive UI during long-running operations
- Background tasks for OCR, AI processing, indexing, sync, report generation
- Avoid unnecessary database queries
- Profile before optimizing

---

## Documentation Requirements

Every module must document: Purpose, Public interfaces, Dependencies, Sequence of operations, Database interactions, Events published, Events consumed, Testing strategy

Maintain Architecture Decision Records (ADRs) for significant design choices.

---

## Foundation Completion Criteria

Before implementing any features, the following must exist:

1. Complete Rust workspace
2. Frontend scaffold
3. Shared coding standards
4. Dependency graph
5. Database migration framework
6. IPC layer
7. Event bus
8. Logging framework
9. Configuration system
10. Testing infrastructure
11. Plugin framework skeleton
12. Background job scheduler
13. Documentation templates

---

# SECTION 3 — REPOSITORY + PROJECT BOOTSTRAP STRUCTURE

## Primary Objective

Create a fully initialized monorepo containing:
- Rust backend workspace (multi-crate architecture)
- Tauri 2 desktop application shell
- React + TypeScript frontend
- Shared types and contracts
- Database migration system
- IPC communication layer
- Event system foundation
- Testing infrastructure
- Linting + formatting rules
- CI/CD pipeline scaffolding
- Development scripts
- Documentation structure

## Monorepo Structure

```
phoenix-lifeos/
├── apps/
│   └── desktop/                 # Tauri 2 app entrypoint
├── crates/                      # Rust workspace
│   ├── core/
│   ├── finance/
│   ├── credit/
│   ├── budgeting/
│   ├── debt/
│   ├── ai/
│   ├── sync/
│   ├── documents/
│   ├── vault/
│   ├── crm/
│   ├── automation/
│   ├── reports/
│   ├── security/
│   ├── settings/
│   ├── plugins/
│   ├── shared/
│   └── db/
├── packages/
│   ├── ui/                     # Shared React component library
│   ├── types/                  # Shared TypeScript types
│   ├── api/                    # IPC contract definitions
│   └── config/                 # Shared configuration
├── src-tauri/                  # Tauri backend bridge
├── docs/
│   ├── architecture/
│   ├── adr/
│   ├── api/
│   ├── guides/
│   └── specs/
├── scripts/
│   ├── dev/
│   ├── build/
│   ├── db/
│   ├── test/
│   └── release/
├── tests/
│   ├── integration/
│   ├── e2e/
│   └── performance/
├── .github/
│   └── workflows/
├── .env.example
├── Cargo.toml
├── package.json
├── pnpm-workspace.yaml
├── tauri.conf.json
├── README.md
└── Makefile
```

## Rust Workspace Requirements

- Each crate compiles independently
- Shared logic → `shared/`
- Database logic → `db/`
- Core domain logic → `core/`
- Feature crates must NOT depend on UI
- Root `Cargo.toml` defines workspace members clearly

## Tauri Setup Requirements

- React frontend build pipeline
- Secure IPC commands
- File system access control
- Plugin-safe architecture
- Window: modern chrome, resizable, high DPI, multi-monitor, system tray (future)

## Frontend Stack

- Vite + Strict TypeScript
- Tailwind CSS + shadcn/ui
- Framer Motion
- TanStack Query (React Query)
- React Hook Form + Zod
- Recharts

## Frontend Structure

```
src/
├── app/
├── components/
├── features/
├── layouts/
├── pages/
├── routes/
├── hooks/
├── services/
├── stores/
├── lib/
├── utils/
├── styles/
├── types/
└── tests/
```

Feature-first architecture mandatory. Each feature self-contained.

## Shared Types System

- All IPC communication uses strongly typed contracts
- No untyped JSON blobs allowed
- Shared types must be versioned
- Rust structs must map cleanly to TypeScript interfaces

## IPC Layer

Frontend → Backend must be: typed, validated, versioned, logged, permission-checked.
No direct database access from frontend. All operations through Rust commands.

## Database Bootstrap

SQLite requirements:
- WAL mode enabled
- Foreign keys enabled
- Migration-based schema (`migrations/001_init.sql`, `migrations/002_core_tables.sql`, etc.)
- Transaction-safe writes
- Repository abstraction layer

## Core Initial Tables (Minimum Bootstrap)

`users`, `accounts`, `transactions`, `categories`, `budgets`, `goals`, `debts`, `credit_reports`, `documents`, `vault_items`, `contacts`, `vehicles`, `properties`, `businesses`, `settings`, `events_log`

## Event System Bootstrap

- Async-safe, type-safe events
- Domain events only (no UI events in backend)
- Events: `TransactionCreated`, `BudgetUpdated`, `GoalProgressUpdated`, `CreditScoreChanged`, `DocumentUploaded`

## Logging System

- `tracing` crate, JSON output support
- Levels: debug, info, warn, error
- Must include: module name, event type, correlation IDs
- Never log secrets

## Configuration System

Layered: Defaults → Environment → User Settings → Feature Flags
Must support: AI providers, sync settings, UI preferences, security settings

## Testing Infrastructure

- Rust: unit tests + integration tests
- Frontend: Vitest + Playwright
- E2E: full workflow tests
- Performance: benchmark framework

## CI/CD Pipeline (GitHub Actions)

Workflows: linting, formatting, testing, building, security scanning
Pipeline must fail on: failed tests, lint violations, type errors

## Development Scripts

- `dev` — start frontend + backend
- `build` — release build
- `test` — run all tests
- `migrate` — run migrations
- `db:reset` — reset database
- `db:seed` — seed test data
- `lint:fix` — fix lint issues

## Documentation

- README.md (project overview)
- Architecture overview
- Setup guide
- Development guide
- Contribution guide
- Module documentation templates

## Quality Gate

Before this section is complete, verify:
- [ ] Project compiles
- [ ] Tauri app launches
- [ ] Frontend renders
- [ ] SQLite initializes
- [ ] Migrations run successfully
- [ ] IPC communication works
- [ ] Logging system works
- [ ] Basic UI shell loads

---

# SECTION 4 — RUST CORE ARCHITECTURE IMPLEMENTATION

## Architecture Enforcement

**Domain Layer (pure logic only):** No DB access, no async, no UI, no external calls.
Contains: Entities, Value Objects, Business Rules, Domain Errors.

**Application Layer (services):** Orchestrates workflows, uses repositories, emits events, handles transactions, contains async logic.

**Infrastructure Layer:** SQLite impl, external integrations, AI providers, filesystem, sync engine.

## Core Domain Entities

### Financial
- `Account` — id, name, type (checking/savings/credit/loan/cash/investment), balance, currency, institution, created_at
- `Transaction` — id, account_id, amount (Money), category_id, description, merchant, date, type (income/expense/transfer), recurring, tags
- `Category` — id, name, type (income/expense), parent_category
- `Budget` — id, name, period (weekly/monthly/yearly), total_limit, category_limits, spent_amount, remaining_amount
- `Goal` — id, name, target_amount, current_amount, deadline, type (savings/debt/home/vehicle/custom)
- `Debt` — id, name, principal, interest_rate, minimum_payment, remaining_balance, due_date, type
- `CreditReport` — id, score, utilization, accounts, inquiries, collections, report_date
- `Investment` — id, asset_type, symbol, quantity, purchase_price, current_value

### Life & Property
- `Vehicle` — id, make, model, year, vin, mileage, insurance_info
- `Property` — id, address, value, mortgage, insurance
- `HomeAsset` — id, name, category, purchase_date, warranty_end, cost, notes

### Business & Records
- `BusinessEntity` — id, name, type (LLC/sole prop), income, expenses
- `Document` — id, name, type, encrypted_path, metadata, tags
- `VaultItem` — id, type (password/key/note), encrypted_value, metadata
- `Contact` — id, name, role, organization, notes, related_entities

## Value Objects

Strict domain types — no raw primitives for financial logic:

`Money` (currency-safe), `Percentage`, `DateRange`, `Email`, `PhoneNumber`, `Address`, `VIN`, `CreditScore`, `InterestRate`

## Business Rules Engine

| Domain | Rules |
|---|---|
| Budget | Prevent overspend, track allocation, compute variance, detect anomalies |
| Debt | Interest accrual, minimum payment enforcement, snowball vs avalanche |
| Credit | Utilization calculation, score trend, impact simulation |
| Net Worth | Assets − Liabilities, auto-recalculate via events |
| Goals | Progress tracking, deadline forecasting, monthly contribution suggestions |

## Application Services

| Service | Responsibilities |
|---|---|
| `FinanceService` | Create transactions, update balances, categorize spending, generate summaries |
| `BudgetService` | Create budgets, track spending vs limits, generate alerts, forecast overages |
| `DebtService` | Calculate payoff strategies, generate schedules, compute interest projections |
| `CreditService` | Process credit reports, calculate utilization, track score changes, generate insights |
| `GoalService` | Update progress, recalculate projections, trigger milestone events |
| `ReportService` | Generate financial reports, aggregate data, export (CSV/PDF/JSON) |
| `AIAnalysisService` | Prepare sanitized context, generate prompts, interpret AI responses |

## Repository Layer

```rust
trait TransactionRepository {
    fn create(&self, tx: Transaction) -> Result<()>;
    fn get_by_id(&self, id: Uuid) -> Result<Transaction>;
    fn list_by_account(&self, account_id: Uuid) -> Result<Vec<Transaction>>;
}
```

Rules: no raw SQL in service layer, all persistence through repositories, repositories are injectable.

## Event System

**Requirements:** async-safe, thread-safe, type-safe, non-blocking, replayable (future)

**Core Events:** `TransactionCreated`, `TransactionUpdated`, `BudgetExceeded`, `DebtPaymentMade`, `CreditScoreUpdated`, `GoalProgressUpdated`, `NetWorthRecalculated`, `DocumentAdded`, `VaultItemCreated`

**Rules:** Services publish, others subscribe. No direct cross-service coupling. No UI dependency.

## Financial Calculation Engine

- **Interest:** simple interest, compound interest, amortization schedules
- **Debt Payoff:** snowball, avalanche, hybrid optimization
- **Budget Forecasting:** predict overspend, monthly projection, category trends
- **Net Worth:** real-time recalculation, historical tracking

## Error Handling

Use `thiserror`: `InsufficientFunds`, `InvalidTransaction`, `BudgetExceeded`, `InvalidDebtState`, `CalculationError`, `RepositoryError`

## Implementation Order

1. Value Objects → 2. Core Entities → 3. Repository Traits → 4. Event System → 5. FinanceService → 6. BudgetService → 7. DebtService → 8. CreditService → 9. GoalService → 10. ReportService → 11. AIAnalysisService → 12. Financial Calculation Engine

## Quality Gate

- [ ] All unit tests pass
- [ ] Financial calculations validated
- [ ] Event system functional
- [ ] Repositories implemented (in-memory stub acceptable initially)
- [ ] No circular dependencies
- [ ] Clean compilation
- [ ] Domain logic fully isolated


---

# SECTION 5 — FRONTEND LIFE HUB + DASHBOARD ARCHITECTURE

## Architecture Model

```
src/
├── app/                     # App bootstrap
├── routes/                  # Route definitions
├── layouts/                 # Shell layouts
├── features/                # ALL business features (feature-first)
├── components/              # Shared UI components
├── hooks/                   # Shared hooks
├── services/                # IPC + API calls
├── stores/                  # State management
├── lib/                     # Utilities
├── styles/                  # Tailwind + themes
├── types/                   # Shared TS types
└── tests/
```

Each feature is self-contained:
```
features/budget/
├── components/ ├── pages/ ├── hooks/ ├── services/
├── store/ ├── types/ ├── utils/ └── tests/
```

## Core UI System Components

Button, Card, Input, Modal, Drawer, Tabs, Table, Badge, Tooltip, Dropdown, Toast, Command Palette, Sidebar, Navbar, Widget Container

## Design Language

- Glassmorphism (subtle, not excessive)
- Soft shadows, rounded corners, high contrast typography
- Dark mode default, light mode optional
- Smooth Framer Motion transitions (page transitions, modal open/close, widget drag/drop)
- Animations must be subtle, never block interaction

## Life Hub Layout

```
┌─────────────────────────────────────────────┐
│ Top Bar: Search / Command Palette / Profile │
├─────────────────────────────────────────────┤
│ Sidebar | Main Dashboard Grid               │
│         | Widgets (Draggable Grid)          │
└─────────────────────────────────────────────┘
```

## Life Hub Widgets (Draggable, Resizable, Persistent)

**Financial:** Net Worth, Cash Flow, Spending Breakdown, Budget Progress, Debt Overview, Credit Score, Savings Goals, Investment Summary

**Life:** Tasks, Calendar, Life Timeline, Notifications, Opportunities, AI Insights, Dream Board, Wellness Score

## Dashboard System Requirements

- Drag-and-drop layout (dnd-kit or react-grid-layout)
- Save user layout to persistent store
- Resize, add, remove widgets
- Per-user configuration

## Global Navigation (Sidebar)

Life Hub → Dashboard → Budget → Debt → Credit → Investments → Goals → Vault → CRM → Maintenance → Business → Reports → Settings

## Command Palette

Global shortcut: `Ctrl+K`

Capabilities: Navigate anywhere, run actions, search data, trigger AI commands

Examples: "Add transaction", "Scan receipt", "Open budget", "Show debt payoff plan", "Run credit analysis", "Generate report", "Search documents"

## State Management

| State Type | Solution |
|---|---|
| Server state | TanStack Query (cached IPC responses) |
| UI state | Zustand or lightweight equivalent |
| Feature state | Local feature stores |
| Form state | React Hook Form + Zod |

## IPC Service Layer

All backend comms through typed services:
```ts
financeService.createTransaction()
budgetService.getBudgets()
creditService.getScore()
aiService.analyzeSpending()
```
Rules: no direct Rust calls from components, all responses typed, all errors handled.

## AI Assistant UI

- Chat interface with streaming responses
- Context-aware prompts with financial data
- Quick action buttons
- Voice input (future-ready)

## Data Visualization (Recharts)

Net worth charts, spending trends, debt payoff timelines, budget usage, credit score history, investment performance. All charts: responsive, interactive, tooltips, filterable.

## Performance

- Load < 2 seconds
- Lazy load features, code split by feature
- Memoization where appropriate
- Optimize for 10k+ transactions

## Accessibility

- Keyboard navigation, screen readers, ARIA labels, high contrast mode, scalable fonts
- WCAG AA minimum

## Implementation Order

1. App Shell → 2. Layout System → 3. Sidebar Navigation → 4. Life Hub Base UI → 5. Widget System → 6. IPC Service Layer → 7. Command Palette → 8. Core Dashboard Widgets → 9. Charts System → 10. AI Assistant UI → 11. Feature Pages → 12. Settings System → 13. Layout persistence

## Quality Gate

- [ ] UI renders without errors
- [ ] Widgets draggable
- [ ] IPC communication works
- [ ] Command palette functional
- [ ] Life Hub loads aggregated data
- [ ] Charts render correctly
- [ ] No TypeScript errors
- [ ] Feature isolation enforced
- [ ] Layout persistence works

---

# SECTION 6 — DATABASE DESIGN + MIGRATION SYSTEM

## Database Principles

- SQLite as single source of truth (local-first)
- WAL mode mandatory
- Foreign keys always enabled
- All writes transactional
- No schema-less storage for core entities
- No duplication of financial truth
- No business logic in SQL beyond constraints
- No raw SQL in frontend

## Migration System

```
migrations/
├── 0001_init.sql
├── 0002_finance_core.sql
├── 0003_credit.sql
├── 0004_budget.sql
├── 0005_debt.sql
├── 0006_investments.sql
├── 0007_goals.sql
├── 0008_documents.sql
├── 0009_vault.sql
├── 0010_crm.sql
├── 0011_maintenance.sql
├── 0012_business.sql
├── 0013_ai_logs.sql
├── 0014_sync.sql
└── 0015_events.sql
```

Every migration: append-only after release, include rollback SQL, must be reversible where possible.

## Universal Column Rules

Every table includes: `id` (UUID/ULID), `created_at`, `updated_at`, `deleted_at` (soft delete support).

## Schema: Core Financial

**accounts:** id, name, type (checking/savings/credit/loan/cash/investment), institution, balance, currency, is_active

**transactions:** id, account_id (FK), amount (INTEGER cents), currency, type (income/expense/transfer), category_id, merchant, description, date, tags (JSON), is_recurring
*Indexes:* account_id, date, category_id

**categories:** id, name, type (income/expense), parent_id

**budgets:** id, name, period_type (weekly/monthly/yearly), total_limit, start_date, end_date

**budget_items:** id, budget_id, category_id, limit_amount, spent_amount

**goals:** id, name, target_amount, current_amount, deadline, type

**debts:** id, name, principal, remaining_balance, interest_rate, minimum_payment, due_date, type

**credit_reports:** id, score, utilization, report_date, total_accounts, open_accounts, inquiries, collections

**investments:** id, asset_type, symbol, quantity, purchase_price, current_price

## Schema: Life Hub

**events** (system-wide audit log): id, event_type, entity_type, entity_id, metadata (JSON), timestamp

**tasks:** id, title, description, due_date, priority, status, category, related_entity_type, related_entity_id

## Schema: Document System

**documents:** id, name, file_path (encrypted reference), type, mime_type, tags, ocr_text, created_at
*FTS5 index on ocr_text*

## Schema: Vault

**vault_items:** id, type (password/secret/note), title, encrypted_data, metadata (JSON)
Rule: NEVER store plaintext secrets.

## Schema: CRM

**contacts:** id, name, role, organization, email, phone, notes

**contact_interactions:** id, contact_id, type, notes, date

## Schema: Home & Vehicle

**vehicles:** id, make, model, year, vin, mileage, purchase_price

**vehicle_service_records:** id, vehicle_id, service_type, cost, mileage, notes, date

**properties:** id, address, value, mortgage_balance, insurance_cost

**home_assets:** id, name, category, purchase_date, warranty_end, cost, notes

## Schema: Business Finance

**businesses:** id, name, type, tax_id (optional, encrypted), income_total, expense_total

**business_transactions:** id, business_id, amount, category, description, date

## Schema: System Tables

**ai_logs:** id, provider, prompt_hash (NOT raw prompt), response_summary, tokens_used, timestamp

**sync_state:** id, entity_type, entity_id, last_synced_at, sync_hash

## Indexing Strategy

Indexes required on: transaction.date, account_id, category_id, goal_id, debt_id, event_type, created_at

## Full-Text Search

SQLite FTS5 across: transactions, documents, contacts, vault metadata, notes

## Data Integrity Rules

- No orphaned transactions
- Debt cannot go below zero
- Goals cannot exceed target unless flagged
- All FK constraints enforced

## Performance Requirements

- Handle 100k+ transactions efficiently
- Indexed queries for all dashboard views
- Avoid N+1 patterns
- Batch writes for imports
- Prepared statements throughout

## Security Requirements

- Sensitive columns encrypted at rest (vault, credentials)
- No plaintext secrets in database
- Application-layer encryption for vault items
- Database file protected within app sandbox

## Quality Gate

- [ ] All migrations run cleanly
- [ ] Schema normalized
- [ ] Foreign keys enforced
- [ ] Indexes validated
- [ ] FTS working
- [ ] CRUD operations tested
- [ ] No schema duplication
- [ ] No missing relationships

---

# SECTION 7 — SECURITY + ENCRYPTION ARCHITECTURE

## Threat Model (Zero-Trust Assumptions)

- Machine may be compromised
- Database file may be exposed
- Logs may be inspected
- Memory may be analyzed
- Plugins may be malicious
- Sync endpoints may be untrusted

## Security Principles

Zero trust by default · Least privilege everywhere · No plaintext secrets at rest · No sensitive logs · Explicit encryption boundaries · Defense in depth · Secure by default

## Encryption Algorithms

| Use Case | Algorithm |
|---|---|
| General encryption | AES-256-GCM |
| High-performance secure encryption | XChaCha20-Poly1305 |
| Password hashing | Argon2id |
| Key derivation | HKDF |

Never use: MD5, SHA1, weak PBKDFs.

## Key Management System (Master Key Architecture)

```
User Password
   ↓ Argon2id
Master Key (UMK)
   ↓ HKDF per domain
├── Vault Key
├── Document Key
├── Sync Key
├── AI Context Key
└── Backup Key
```

**Key Storage Rules:**
- NEVER store master key in plaintext
- NEVER store derived keys unencrypted
- Keys exist in memory only, temporarily
- Clear memory on logout
- Rotate keys on password change

## Vault Security

Each vault item: `encrypted_data`, `nonce`, `key_version`, `integrity_hash`

Vault items encrypted individually, authenticated (tamper-proof), indexed without exposing plaintext.

## Document Security

- Encrypted at rest, decrypted on demand
- Never stored in plaintext cache
- Per-file encryption keys
- OCR text also encrypted

## IPC Security (Tauri Boundary)

All frontend → backend: strictly typed, validated, rate-limited (future), permission-checked, routed through domain services only.

## Input Validation

- Rust type system (backend)
- Zod schemas (frontend)
- Domain validation layer
- Reject: malformed IDs, invalid money values, injection attempts, oversized payloads

## Authentication System

Single-user initially, multi-user ready. Login: password-based, optional biometric (future), optional hardware key (future).

**Session Security:**
- Session tokens in memory only
- Auto-lock after inactivity
- Secure session timeout
- Re-auth required for sensitive operations

## Sensitive Operations Requiring Re-Auth

Viewing vault items · Exporting data · Changing password · Syncing devices · Accessing document storage · Viewing private CRM notes

## Logging Security Policy

**NEVER log:** passwords, vault contents, encryption keys, full documents, sensitive CRM notes

**MAY log:** event types, IDs, timestamps, non-sensitive metadata

## AI Security Sandbox

AI is a controlled subsystem, not a trusted actor:
- AI never accesses raw vault keys
- AI receives sanitized, minimal context only
- Prompt injection protection required
- All AI requests through `AIService` gateway

**AI Sanitization Pipeline (before sending to AI):**
1. Remove secrets → 2. Remove encryption keys → 3. Strip identifiers where possible → 4. Summarize financial data → 5. Limit scope to requested task

## Sync Security (Design-Ready)

E2E encryption · Client-side only · Zero-knowledge sync servers · Conflict-safe merging · Device trust model

## Backup Security

Fully encrypted · Password protected · Versioned · Integrity checked · No plaintext backups under any condition.

## Plugin Security Model

Plugins must: declare permissions explicitly, be sandboxed, never access vault/keys directly, use restricted API surface, be signed in future versions.

## Access Control Capabilities

`read_finance`, `write_finance`, `read_vault`, `write_vault`, `export_data`, `run_ai`, `sync_devices`

## Data Exposure Rules

**Frontend may receive:** sanitized DTOs, derived metrics, aggregated financial data

**Frontend must NEVER receive:** encryption keys, raw vault data, raw secrets, internal system keys

## Security Testing Requirements

- Encryption/decryption round-trip tests
- Key rotation tests
- Unauthorized access tests
- IPC abuse tests
- AI prompt injection tests
- Vault access control tests
- File tampering detection tests

## Quality Gate

- [ ] All sensitive fields encrypted
- [ ] Vault fully secure
- [ ] Keys never persisted in plaintext
- [ ] AI fully sandboxed
- [ ] IPC validated
- [ ] Logs sanitized
- [ ] Access control enforced
- [ ] All security tests passing

---

# SECTION 8 — AI ARCHITECTURE + LOCAL LLM INTEGRATION

## System Purpose

This is a **financial reasoning and decision-support engine**, NOT a chatbot. It uses structured personal data to analyze, forecast, and advise. AI is advisory only — never a source of financial authority or legal advice.

## Multi-Provider Architecture

| Type | Providers |
|---|---|
| Cloud (optional) | OpenAI, Anthropic, Gemini |
| Local (preferred) | Ollama (primary local LLM runtime) |

## AI Abstraction Layer

All AI calls through a unified interface — no feature may call a provider directly:

```
AIService
├── chat()
├── analyze_finances()
├── summarize_data()
├── generate_report()
├── explain_transaction()
├── forecast()
└── optimize_budget()
```

## AI Orchestration (Router System)

Responsibilities: select model by task, route local vs cloud, enforce privacy, manage context size, handle streaming + retry + fallback.

## Tool-Using System (Critical Design)

AI operates as a **tool-using system**. All tool calls go through Rust backend services — AI cannot directly access DB.

**Finance Tools:** `get_transactions()`, `get_budget_status()`, `get_debt_summary()`, `get_net_worth()`, `get_credit_report()`

**Planning Tools:** `simulate_debt_payoff()`, `forecast_cash_flow()`, `estimate_mortgage()`, `estimate_vehicle_affordability()`

**Analysis Tools:** `detect_spending_anomalies()`, `categorize_transactions()`, `identify_subscriptions()`, `calculate_utilization()`

**Document Tools:** `search_documents()`, `summarize_document()`, `extract_receipt_data()`

## Context Engine

**Context Types:** Financial snapshot, recent transactions, budget status, debt summary, credit summary, goals status, user preferences, time window filters

**Rules:** Minimize tokens, summarize large datasets, remove sensitive identifiers, aggregate where possible, prioritize recent data.

## Local AI (Ollama Integration)

- Detect installed models, load dynamically, switch at runtime
- Stream responses, cache embeddings locally
- Small models → classification/categorization
- Medium models → budgeting/summarization
- Large models → financial reasoning

## Embeddings System

Store embeddings for: transactions, documents, CRM notes, vault metadata (non-sensitive), financial insights

Use for: semantic search, spending cluster detection, anomaly detection, AI memory retrieval

*Resolution note: embeddings stored in SQLite using blob columns; SQLite-VSS extension considered for future vector search capability.*

## Financial AI Capabilities

| Engine | Capabilities |
|---|---|
| Spending Analysis | Overspending patterns, subscription waste, unusual transactions, category drift, seasonal behavior |
| Debt Optimization | Snowball, avalanche, hybrid, interest minimization, payoff simulation |
| Budget Generation | Auto-generate from history, income adjustment, category caps, scenario simulation |
| Credit Coach | Utilization reduction, payment timing, credit mix, inquiry impact |
| Home/Vehicle Planning | Affordability, monthly payments, insurance, maintenance, TCO |
| Retirement Engine | Savings growth, investment projections, retirement readiness, contribution optimization |

## AI Memory System

| Memory Type | Rules |
|---|---|
| Short-term context | Session only |
| Long-term insights | Summarized only |
| User preferences | Structured |
| Financial patterns | Aggregated only |

Rules: never store raw sensitive data, summaries only, user-deletable, user-inspectable, encrypted.

## Prompt Engineering Layer

All prompts: templated, versioned, structured, testable.

```
ROLE: Financial Analyst AI
GOAL: {task_description}
DATA: {structured_financial_context}
RULES:
  - No financial advice claims
  - Use numerical reasoning
  - Be concise
  - Provide actionable insights
```

## AI Safety Rules

**NEVER:** request passwords, access raw vault data, access raw encryption keys, expose system internals, bypass tool layer, hallucinate account balances

**ALWAYS:** use tools for truth, explain uncertainty, avoid financial guarantees, remain advisory only

## Streaming

Token streaming, partial UI rendering, cancelation, retry.

## Observability

Log: prompt version, model used, token count, tool calls, response time, error states. Never log sensitive financial raw data.

## Error Handling

Handle: model failure, timeout, malformed tool response, missing context, rate limiting. Fallback: switch model → degrade gracefully → use local if cloud fails.

## Quality Gate

- [ ] Tool system functional
- [ ] Ollama integration working
- [ ] Multi-provider routing tested
- [ ] Embeddings operational
- [ ] Financial analysis validated
- [ ] Memory system secure
- [ ] Prompt templates versioned
- [ ] Streaming stable

---

# SECTION 9 — LIFE HUB INTELLIGENCE SYSTEM

## Core Principle

The Life Hub does NOT own data. It visualizes and orchestrates it. All logic remains in backend services.

## System Architecture

```
┌──────────────────────────────────────────────┐
│              UI LAYER (React)                │
├──────────────────────────────────────────────┤
│        LIFE HUB ORCHESTRATION ENGINE         │
│   (State Aggregator + Event Listener)        │
├──────────────────────────────────────────────┤
│         DOMAIN EVENTS STREAM (Rust)          │
├──────────────────────────────────────────────┤
│     SERVICES + AI + DATABASE LAYER           │
└──────────────────────────────────────────────┘
```

## Orchestration Engine Responsibilities

Subscribe to backend event stream · Maintain aggregated state snapshot · Merge multiple domain states · Trigger UI updates efficiently · Coordinate AI insights injection

## Event Sources

`TransactionCreated`, `BudgetUpdated`, `DebtPaymentMade`, `CreditScoreUpdated`, `GoalProgressUpdated`, `DocumentAdded`, `VaultAccessed`, `VehicleServiceDue`, `HomeMaintenanceDue`, `AIInsightGenerated`

## Life Hub State Model

```typescript
interface LifeHubState {
  finance: FinanceSnapshot;
  credit: CreditSnapshot;
  debt: DebtSnapshot;
  goals: GoalsSnapshot;
  crm: CRMSnapshot;
  maintenance: MaintenanceSnapshot;
  aiInsights: AIInsight[];
  notifications: Notification[];
  events: EventStream[];
}
```

## Widget System

Dynamic grid with resizable/draggable widgets, saved layouts, responsive breakpoints.

**Financial widgets:** Net Worth, Cash Flow, Spending Breakdown, Budget Health, Debt Payoff Timeline, Credit Score Trend, Investment Summary

**Life widgets:** Tasks & Deadlines, Calendar Events, Maintenance Alerts, CRM Reminders, Notifications Feed, AI Insights Panel, Life Score Summary

**System widgets (dev mode):** Event Stream, Sync Status, System Health, Background Jobs

## AI Insight Types

`ALERT` (urgent financial issue) · `SUGGESTION` (optimization opportunity) · `FORECAST` (future prediction) · `EXPLANATION` (why something happened) · `REMINDER` (action needed)

Rules: contextual, reference real data, no hallucinated numbers, explainable, include confidence level.

## Life Score Engine

```
LifeScore = f(financial_health, debt_ratio, savings_rate, credit_score, goal_progress, spending_stability)
```

Computed in backend, displayed prominently.

## Financial Snapshot (backend-computed)

Net worth · Monthly cash flow · Burn rate · Debt ratio · Savings rate · Emergency fund coverage

## Performance

- Render < 1 second after load
- Handle 100k+ transactions
- Batch UI updates, avoid unnecessary re-renders
- Lazy-load widgets

## Implementation Order

1. Life Hub shell layout → 2. Event subscription system → 3. State aggregation engine → 4. Widget grid system → 5. Core financial widgets → 6. AI insight panel → 7. Command system → 8. Notification system → 9. Life Score engine → 10. Event stream UI

---

# SECTION 10 — FINANCE ENGINE UI

## Module Structure

```
features/
├── accounts/
├── transactions/
├── budgets/
├── debt/
├── credit/
├── investments/
├── goals/
└── reports/
```

Each module: pages/, components/, hooks/, services/, store/, types/, tests/

## Accounts Module

List view, detail page, balance history chart, manual adjustments. Actions: Create, Edit, Archive, Reconcile, Sync transactions.

## Transactions Module

Virtualized table (100k+ rows), filters (date/category/account/amount), full-text search, bulk edit, split transactions, categorization UI.

Actions: Add, Edit, Delete (soft only), Auto-categorize, Import CSV

AI: auto-categorization suggestions, anomaly detection, subscription detection, spending explanation

## Budget Module

Overview dashboard, category breakdown, progress bars, monthly/weekly views, forecasting chart.

AI: auto-budget generation, overspending prediction, optimization suggestions, savings opportunities

## Debt Module

Debt list, payoff timeline chart, interest accumulation visualization, snowball vs avalanche selector.

AI: optimal payoff strategy, interest minimization, timeline simulation, restructuring recommendations

## Credit Module

Score history chart, utilization breakdown, account impact view, inquiry tracking, risk indicators.

AI: improvement plan, utilization optimization, score drop explanation, recovery forecasting

## Investments Module

Portfolio overview, asset allocation chart, performance timeline, gains/losses breakdown.

AI: diversification suggestions, risk analysis, growth forecasting

## Goals Module

Goal list, progress bars, contribution calculator, timeline forecast.

AI: acceleration plan, savings optimization, feasibility analysis

## Reports Module

Types: Monthly summary, Net Worth, Spending Breakdown, Debt Progress, Credit Health

Export: PDF, CSV, JSON

## Critical Rules

Frontend MUST NOT compute: net worth, interest, payoff schedules, credit utilization, budget totals. All calculations from backend.

No hard deletes (soft delete only). All changes logged in events table. All edits auditable.

## Performance

- Virtualized transaction table (100k+)
- Indexed filtering
- Memoized charts
- Async/cancellable AI calls

## Implementation Order

1. Accounts → 2. Transactions → 3. Budget → 4. Debt → 5. Credit → 6. Goals → 7. Investments → 8. Reports → 9. AI panel → 10. Visualization system

---

# SECTION 11 — VAULT + DOCUMENTS + OCR + EMAIL IMPORT SYSTEM

## Security-First Design

Handles the most sensitive data in the system. All ingestion encrypted immediately. No raw external data to frontend.

## Vault System

**VaultItem:** id, type (password/note/key/secret), title, encrypted_payload, nonce, key_version, created_at, updated_at

**Rules:** NEVER store plaintext · Decryption in memory only · Re-auth required · Auto-lock on inactivity · All access audited

**Features:** Secure search (encrypted index), tag system, folder grouping, access logs, emergency lock mode

## Document Management System

**Document:** id, name, file_type, encrypted_file_path, ocr_text_encrypted, extracted_metadata, tags, source (upload/email/scan)

Features: drag-and-drop upload, encrypted storage, tagging, on-demand decrypted preview, version tracking

## OCR Pipeline

1. File ingestion → 2. Preprocessing (cleanup, grayscale) → 3. OCR execution (Tesseract/leptess) → 4. Text extraction → 5. Structured parsing → 6. Encryption → 7. Storage

**Receipt extraction:** merchant, date, total_amount, tax, items[], category_guess

**AI enhancement:** classify document type, extract structured fields, detect anomalies, suggest categories

## Email Import Pipeline

Sources: Gmail (future), IMAP (optional), manual upload

Pipeline: 1. Fetch → 2. Sanitize (strip headers, tracking) → 3. Extract attachments → 4. OCR if needed → 5. Financial parsing → 6. Suggest transaction → **7. User approval (mandatory)** → 8. Transaction created

Security: no raw email stored unencrypted, only structured extracted data retained

## File Ingestion Rules

- All files encrypted immediately on receipt
- Temporary files deleted after processing
- File hashing for integrity
- Deduplication system

## Search System

SQLite FTS5 for structured text · Embeddings for semantic search (future)

Rules: encrypted fields excluded from plaintext search, access-controlled results

## Receipt Processing Engine

`Receipt → OCR → Structured Data → AI Validation → Suggested Transaction → User Approval → Transaction Created`

NEVER auto-create financial transactions without user confirmation.

## Audit Log (All Ingestion Actions)

Log: file uploads, OCR runs, email imports, vault access, document decryptions

Never log sensitive raw content.

## OCR Implementation Note

Use `leptess` Rust crate (Tesseract bindings) for initial implementation. Background thread execution, async pipeline.

## Implementation Order

1. Vault system → 2. Document storage → 3. File ingestion pipeline → 4. OCR engine → 5. Receipt parser → 6. Email import → 7. Search indexing → 8. AI enrichment → 9. Audit logging

---

# SECTION 12 — PERSONAL CRM + HOME + VEHICLE + MAINTENANCE SYSTEM

## Personal CRM

**Contact model:** id, name, role, organization, email, phone, address, notes, importance_level, tags

**ContactInteraction:** id, contact_id, type (call/email/meeting), notes, timestamp, follow_up_date

Features: contact directory, relationship tagging, interaction history, reminder scheduling, linked financial relationships, vault document association

AI: summarize interactions, suggest follow-ups, detect inactive relationships, recommend contacts by need

## Home Management

**Property:** id, address, type, value, mortgage_balance, insurance_cost, tax_rate, purchase_date

**HomeAsset:** id, name, category (appliance/furniture/system), purchase_date, cost, warranty_end_date, condition_status, notes

Finance integration: maintenance costs → transactions, insurance → recurring budget, mortgage → debt system

## Vehicle Management

**Vehicle:** id, make, model, year, vin, mileage, purchase_price, insurance_provider, registration_expiry, loan_balance

**VehicleServiceRecord:** id, vehicle_id, service_type, cost, mileage, service_provider, notes, date

AI: predict maintenance needs, estimate repair costs, suggest schedules, detect abnormal cost patterns, calculate TCO

## Maintenance System

**MaintenanceTask:** id, entity_type (home/vehicle/asset), entity_id, task_type, description, due_date, recurrence, status, estimated_cost

Features: recurring schedules, smart reminders, overdue detection, cost tracking, AI-generated maintenance plans

## Warranty Tracking

**Warranty:** id, entity_type, entity_id, provider, start_date, end_date, coverage_details, claim_contact

Alerts: expiration warnings, claim reminders, vault document linkage

## Events

`ContactAdded`, `ContactUpdated`, `VehicleServiceDue`, `MaintenanceScheduled`, `WarrantyExpiring`, `HomeAssetAdded`, `PropertyUpdated`

## Financial Integration Rules

Every real-world event maps to finance: vehicle repair → transaction, home repair → transaction, insurance → recurring expense, contractor payment → transaction

## Performance

Support: 1,000+ contacts, 100+ assets, long maintenance histories. Fast filtering + FTS search.

## Implementation Order

1. CRM system → 2. Vehicle system → 3. Home/property system → 4. Maintenance system → 5. Warranty system → 6. Event integration → 7. AI integration → 8. Finance linking → 9. Notification system

---

# SECTION 13 — AUTOMATION ENGINE + BACKGROUND JOB SYSTEM

## System Architecture

```
EVENT BUS → AUTOMATION ENGINE CORE (Workflow Orchestrator) → JOB EXECUTION SYSTEM (Workers + Schedulers) → DOMAIN SERVICES LAYER
```

## Core Concepts

**Job** — single unit of work (recalculate budget, OCR document, generate report, etc.)

**Workflow** — multi-step automation sequence (e.g., Receipt: Upload → OCR → AI classify → Suggest transaction → User approval → Finance update → Event)

**Trigger types:** Event-based, Time-based, Manual, AI-suggested (suggestions only, never direct execution)

## Job Model

id, type, payload, status (pending/running/completed/failed), retry_count, max_retries, scheduled_at, created_at

**Execution rules:** idempotent, no duplicate execution, exponential backoff retry, failure logged, partial progress tracked

## Workflow Engine

**Workflow:** id, name, steps[], trigger_type, status, context

**Step:** id, type, service, input_mapping, output_mapping, retry_policy, compensation_action

**Execution rules:** step-by-step, each step validated, failure triggers rollback if defined, state persisted after each step

## Event → Automation Mapping

- `BudgetExceeded` → AIInsight + Notification
- `ReceiptScanned` → OCR Workflow
- `DebtPaymentMade` → Debt Recalculation Job
- `CreditScoreUpdated` → AI Credit Analysis Workflow
- `WarrantyExpiring` → Reminder Notification

## Scheduler System

Cron-like scheduling, timezone-aware, persistent across restarts, job recovery after crash.

Scheduled jobs: daily budget summary, weekly financial report, monthly credit review, maintenance reminders, backup operations

## Worker System

Stateless workers, controlled concurrency, queue-based execution, backpressure handling.

## Compensation System

Each workflow step defines rollback: `transaction created → rollback = delete transaction`

## Persistence Tables

`jobs`, `workflows`, `workflow_runs`, `job_logs`, `execution_state`

## AI Constraints

AI CAN: suggest workflows, trigger analysis jobs, request re-processing, generate reports

AI CANNOT: directly mutate financial state, bypass workflow engine, skip validation steps

## Performance

Support 1,000+ jobs/hour · Background processing must not block UI · Minimal latency for event-triggered jobs

## Implementation Order

1. Job system core → 2. Worker execution engine → 3. Scheduler → 4. Workflow engine → 5. Event-trigger integration → 6. Finance automation jobs → 7. AI automation hooks → 8. Compensation system → 9. Observability

---

# SECTION 14 — REPORTING ENGINE + DATA ANALYTICS + EXPORT SYSTEM

## Architecture

```
DATA AGGREGATION LAYER (Finance + CRM + AI + Events)
    ↓
ANALYTICS ENGINE CORE (Metrics + Computations)
    ↓
REPORT GENERATION ENGINE (Templates + AI + Formatting)
    ↓
EXPORT + VISUALIZATION LAYER (PDF / CSV / UI Charts)
```

**Core principle:** All metrics derived from database. AI may only interpret, never invent data. Reports must be reproducible and auditable.

## Analytics Engine — Core Metrics

**Financial:** net worth, monthly cash flow, savings rate, expense ratio, debt-to-income ratio, credit utilization, burn rate, emergency fund coverage

**Behavioral:** spending volatility, category drift, subscription growth rate, savings consistency score

**Credit:** score trend, utilization trend, inquiry impact, account age distribution

**Debt:** payoff trajectory, interest accumulation, optimization efficiency score

## Report Types

1. **Monthly Financial Report** — income summary, expense breakdown, net savings, category analysis, anomalies, AI insights
2. **Net Worth Report** — assets, liabilities, trend, portfolio breakdown, historical comparison
3. **Debt Analysis Report** — debt breakdown, interest projection, payoff timeline, optimization suggestions
4. **Credit Health Report** — score history, utilization trends, risk factors, improvement actions
5. **Spending Behavior Report** — category trends, overspending analysis, subscription detection
6. **Life Score Report** — composite life score, financial health index, risk indicators, improvement roadmap

## Template System

Structured templates for each report type. Templated, versioned sections.

## AI Role in Reporting

**MAY:** summarize trends, explain anomalies, suggest improvements, generate natural language insights

**MAY NOT:** generate financial numbers, override computed values, invent missing data

## Export Formats

PDF (primary) · CSV (data export) · JSON (system integration) · HTML (viewable)

Rules: deterministic outputs, include metadata version + timestamp + data range + system version + data hash.

## Auditability

Each report includes: report ID, generation timestamp, data sources used, logic version, dataset snapshot hash.

## Performance

Reports generate < 2 seconds for typical datasets · Large datasets stream/batch · No UI blocking · Caching for repeated reports.

## Implementation Order

1. Data aggregation layer → 2. Analytics engine → 3. Report templates → 4. Report generator → 5. Visualization → 6. Export system → 7. Scheduling integration → 8. AI insights integration

---

# SECTION 15 — SYNC SYSTEM + MULTI-DEVICE ARCHITECTURE + BACKUP/RESTORE

## Core Principles

Local-first always · Sync is asynchronous · No data loss · Deterministic conflict resolution · End-to-end encryption · Devices are independent nodes

## Architecture

```
DEVICE LAYER (Tauri Desktop App)
    ↓
LOCAL SYNC ENGINE (Change Tracking + Queue)
    ↓
SYNC TRANSPORT LAYER (Encrypted Transfer Protocol)
    ↓
REMOTE SYNC TARGET (Optional Server / Peer Node)
```

## Change Event Log (CEL)

Every write generates a CEL entry: id, entity_type, entity_id, operation (create/update/delete), before_state_hash, after_state_hash, timestamp, device_id, sync_status

Rules: ALL writes generate events, events are immutable and replayable, events queued for sync.

## Device Trust Model

Device: device_id, public_key, trust_level, last_seen, sync_state

Rules: new devices require approval, keys rotate periodically, lost devices can be revoked

## Conflict Resolution

1. Latest timestamp wins (default)
2. Financial data follows backend authority rules
3. User-defined overrides allowed
4. Field-level merge for non-conflicting fields

All conflicts logged, visible to user, manually resolvable.

## Backup System

Types: full, incremental, encrypted snapshot

Contents: database snapshot, vault encrypted data, document metadata, config state, sync logs

Rules: E2E encrypted, versioned, integrity hash included, optional remote storage

## Restore System

Capabilities: full system, partial by module, point-in-time, device migration

Rules: validate integrity hashes, require authentication, no partial corruption allowed

## Offline-First

System fully operates offline. Sync queued in background. UI unaffected by connectivity. AI degrades gracefully to local models.

## Eventual Consistency

Local device is source of truth until sync occurs. Remote merges asynchronously. System converges deterministically.

## Transport Requirements

E2E encryption · No plaintext transit · Mutual device authentication · Replay attack prevention · Signed change events

## Implementation Order

1. Change event logging → 2. Local sync engine → 3. Encryption transport → 4. Device trust → 5. Conflict resolution → 6. Backup → 7. Restore → 8. Offline-first support → 9. Observability

---

# SECTION 16 — FINAL SYSTEM INTEGRATION + PRODUCTION HARDENING

## Application Startup Flow

1. Initialize security layer → 2. Load encryption keys → 3. Open DB (WAL mode) → 4. Initialize sync engine → 5. Start automation engine → 6. Load event system → 7. Initialize AI services → 8. Load UI shell → 9. Hydrate Life Hub state

## Full Stack Integration Model

```
TAURI DESKTOP APP
├── FRONTEND (React LifeOS UI)
├── DOMAIN LAYER (Finance / CRM / AI / Vault)
├── AUTOMATION ENGINE
├── SYNC ENGINE
├── DATABASE LAYER (SQLite + Encryption)
└── SECURITY LAYER (Keys + Vault + Crypto)
```

## Performance Optimization

**Frontend:** React memoization, virtualized lists, deferred widget rendering, lazy feature loading, batched state updates

**Backend (Rust):** async job execution, thread pool optimization, query indexing, prepared statements only, minimal locking

**Database:** WAL mode, aggressive indexing, query caching, pagination on all large queries

## Performance Benchmark Targets

| Operation | Target |
|---|---|
| App startup | < 2 seconds |
| Transaction query | < 100ms |
| Dashboard render | < 1 second |
| AI response (local) | < 2–5 seconds |
| Sync batch | < 3 seconds typical |

## Memory Management

No long-lived sensitive objects in memory · Encrypted data decrypted on demand · Immediate cleanup after use · No memory leaks in AI response streams

## Error Resilience

All errors: logged securely, categorized, recoverable where possible, provide fallback UI state. System never crashes silently.

## Observability

Track: system performance, query latency, sync latency, AI response time, job queue health, memory usage, error rates. Structured logging with correlation IDs. No sensitive data in logs.

## Security Hardening

All vault data encrypted · No plaintext secrets in memory longer than necessary · All IPC validated · All inputs sanitized · AI sandbox enforced · Sync fully encrypted · Attack surface minimized.

## Build Pipeline

1. TypeScript build → 2. Rust compilation → 3. Asset bundling → 4. Encryption key checks → 5. Tauri packaging → 6. Integrity hash generation

**Release modes:** Development (debug + logging) · Production (optimized + secure) · Safe mode (minimal features for recovery)

## Production Readiness Gate

- [ ] Zero critical bugs
- [ ] All systems integrated
- [ ] Security validated
- [ ] Performance benchmarks passed
- [ ] Sync stable across devices
- [ ] Automation reliable
- [ ] AI constrained correctly
- [ ] No data leakage paths

---

> "Every feature is useless unless it survives scale, failure, and time."

---

# END OF CORE SPECIFICATION (Sections 1–16)
