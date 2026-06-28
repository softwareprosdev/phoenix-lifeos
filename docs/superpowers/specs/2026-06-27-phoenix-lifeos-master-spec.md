# Phoenix LifeOS — Master Engineering Specification

**Status:** COMPLETE — All 37 sections collected. Ready for implementation.
**Last Updated:** 2026-06-27

---

## Sections Collected

- [x] Section 1 — Master System Directive
- [x] Section 2 — Enterprise Software Architecture
- [x] Section 3 — Repository + Project Bootstrap Structure
- [x] Section 4 — Rust Core Architecture Implementation (Domain + Services + Events)
- [x] Section 5 — Frontend Life Hub + Dashboard Architecture
- [x] Section 6 — Database Design + Migration System
- [x] Section 7 — Security + Encryption Architecture
- [x] Section 8 — AI Architecture + Local LLM Integration
- [x] Section 9 — Life Hub Intelligence System
- [x] Section 10 — Finance Engine UI
- [x] Section 11 — Vault + Documents + OCR + Email Import System
- [x] Section 12 — Personal CRM + Home + Vehicle + Maintenance System
- [x] Section 13 — Automation Engine + Background Job System
- [x] Section 14 — Reporting Engine + Data Analytics + Export System
- [x] Section 15 — Sync System + Multi-Device Architecture + Backup/Restore
- [x] Section 16 — Final System Integration + Production Hardening
- [x] Section 17 — Retirement Planning Engine
- [x] Section 18 — Tax Estimation Engine
- [x] Section 19 — Business Finance Module
- [x] Section 20 — Mortgage & Home Purchase Planner
- [x] Section 21 — Vehicle Finance & Purchase Planner
- [x] Section 22 — Credit Engine (Deep Dive)
- [x] Section 23 — Debt Payoff Engine (Deep Dive)
- [x] Section 24 — Investment & Portfolio Engine
- [x] Section 25 — Financial Goals Engine
- [x] Section 26 — Voice & Natural Language Interface
- [x] Section 27 — Plugin SDK & Extension System
- [x] Section 28 — UI Design System & Component Library
- [x] Section 29 — Testing Strategy & Quality Assurance
- [x] Section 30 — CI/CD Pipeline & Build System
- [x] Section 31 — Deployment & Distribution
- [x] Section 32 — Data Import & Migration Tools
- [x] Section 33 — Localization & Accessibility
- [x] Section 34 — Performance Engineering
- [x] Section 35 — Developer Mode & Debug Tools
- [x] Section 36 — Security Audit & Compliance
- [x] Section 37 — Future Roadmap & Evolution Strategy

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

# SECTION 17 — RETIREMENT PLANNING ENGINE

## Purpose

Help users project, simulate, and optimize their path to retirement. All calculations run in the Rust backend; the frontend visualizes and explains results.

## Domain Model

**RetirementProfile:** id, target_retirement_age, current_age, current_savings, monthly_contribution, expected_annual_return, inflation_rate, desired_monthly_income

**RetirementAccount:** id, user_id, type (401k/IRA/Roth_IRA/Roth_401k/SEP_IRA/pension), balance, annual_contribution, employer_match_rate, employer_match_cap, vesting_schedule

**RetirementProjection:** id, profile_id, computed_at, projected_balance_at_retirement, monthly_income_sustainable, shortfall_amount, years_until_retirement, confidence_level

**SocialSecurityEstimate:** id, estimated_monthly_benefit, full_retirement_age, early_claim_amount, delayed_claim_amount

## Calculation Engine

**Core Computations:**
- Future value with compound growth: `FV = PV × (1 + r)^n + PMT × ((1 + r)^n - 1) / r`
- Inflation-adjusted projection
- Required monthly contribution to hit target
- Safe withdrawal rate (4% rule as baseline, configurable)
- Retirement runway: `balance / monthly_spend`

**Monte Carlo Simulation:**
- Run 1,000+ scenarios with randomized annual return variance
- Output: probability of not outliving funds by age
- Percentile bands: 10th, 50th, 90th outcomes displayed as fan chart

**Tax-Adjusted Projections:**
- Pre-tax vs post-tax account treatment
- Required Minimum Distribution (RMD) calculations after age 73
- Roth conversion optimization surface

## AI Integration

- Analyze contribution gaps and suggest increases
- Explain scenario differences in plain language
- Identify optimization opportunities (Roth vs traditional, contribution timing)
- "Am I on track?" conversational response with supporting data

## UI Components

- **Retirement Dashboard**: current savings, projected balance, readiness score (0–100)
- **Projection Chart**: fan chart showing pessimistic/expected/optimistic trajectories
- **Contribution Optimizer**: slider to see impact of increasing monthly contribution
- **Account Breakdown**: allocation by account type, employer match visualization
- **Social Security Panel**: estimated benefit at 62, 67, 70
- **Scenario Comparison**: side-by-side simulation of 2–4 strategies

## Events

`RetirementProfileUpdated`, `ContributionChanged`, `ProjectionRecalculated`, `RetirementAccountAdded`

## Implementation Order

1. Domain models + repository → 2. Core projection engine → 3. Monte Carlo simulator → 4. Account management UI → 5. Projection chart → 6. Social Security estimator → 7. AI integration → 8. Scenario comparison

## Quality Gate

- [ ] Projection math validated against known formulas
- [ ] Monte Carlo stable across 1,000+ runs
- [ ] Account types correctly modeled
- [ ] AI explanations accurate
- [ ] Charts render without errors

---

# SECTION 18 — TAX ESTIMATION ENGINE

## Purpose

Help users estimate their annual federal and state income tax liability, understand their tax bracket, identify deductions, and plan quarterly estimated payments. Not a tax filing tool — an estimation and planning tool.

## Disclaimer System

Every tax-related output must display: *"This is an estimate for planning purposes only. Consult a tax professional for filing."*

## Domain Model

**TaxProfile:** id, filing_status (single/married_jointly/married_separately/head_of_household), state, federal_withholding_ytd, state_withholding_ytd

**IncomeSource:** id, type (w2/1099_nec/1099_misc/capital_gains/rental/business/other), gross_amount, ytd_amount, tax_year

**Deduction:** id, type (standard/itemized), category (mortgage_interest/state_tax/charitable/medical/business), amount, tax_year

**TaxEstimate:** id, tax_year, estimated_federal_tax, estimated_state_tax, effective_rate, marginal_rate, refund_or_owed, quarterly_payment_schedule

## Tax Calculation Engine

**Federal Income Tax:**
- 2024+ tax bracket tables (updated annually via configuration, not hardcoded)
- Standard deduction by filing status
- AMT detection flag (not full AMT calculation — advisory only)
- FICA (Social Security + Medicare) on earned income
- Self-employment tax for 1099 income (15.3%, with 50% deductible)
- Capital gains tax rates (0/15/20% by bracket)

**State Income Tax:**
- Support major states initially (CA, NY, TX, FL, IL, WA, PA, OH, GA, NC)
- State tax rates configurable — not hardcoded, loaded from a versioned data file
- No-income-tax states handled correctly

**Quarterly Estimated Taxes:**
- IRS safe harbor rule: 100% of prior year tax (or 110% for high earners)
- Quarterly due dates: April 15, June 15, September 15, January 15
- Suggested quarterly payment amount

## AI Integration

- Identify deduction opportunities from transaction data
- Explain marginal vs effective rate
- Suggest tax-advantaged contribution increases (HSA, 401k, IRA) to reduce liability
- Flag unusual income patterns that may affect tax treatment

## UI Components

- **Tax Summary Card**: estimated liability, effective rate, refund/owed estimate
- **Income Breakdown**: all income sources with tax treatment
- **Deduction Optimizer**: standard vs itemized comparison
- **Bracket Visualizer**: waterfall chart showing income through each bracket
- **Quarterly Planner**: upcoming payment dates and amounts
- **W-4 Adjustment Suggestion**: if withholding is significantly over/under

## Data Sources

Tax-bracket data lives in versioned JSON files updated annually — never hardcoded in Rust:
```
data/tax/federal_brackets_2024.json
data/tax/state_brackets_2024.json
```

## Quality Gate

- [ ] Federal tax math validated against IRS tax tables
- [ ] State tax rates sourced accurately
- [ ] Quarterly schedule correct
- [ ] Disclaimer displayed on all estimates
- [ ] AI suggestions grounded in actual transaction data

---

# SECTION 19 — BUSINESS FINANCE MODULE

## Purpose

Support users who operate a sole proprietorship, LLC, or freelance business alongside their personal finances. Keep business and personal finances cleanly separated while allowing integration for net worth and tax estimation.

## Domain Model

**Business:** id, name, entity_type (sole_prop/llc/s_corp/partnership), industry, tax_id_encrypted, fiscal_year_start, formation_date

**BusinessAccount:** id, business_id, name, type (checking/savings/credit/payroll), balance, institution

**BusinessTransaction:** id, business_id, account_id, amount, type (income/expense/transfer), category, vendor, description, date, is_deductible, receipt_document_id

**Invoice:** id, business_id, client_contact_id, invoice_number, issue_date, due_date, line_items (JSON), subtotal, tax_amount, total, status (draft/sent/paid/overdue/cancelled), payment_date

**Expense:** id, business_id, amount, category (office/travel/meals/software/equipment/marketing/contractor/other), vendor, description, date, is_deductible, receipt_id, mileage

**BusinessSummary:** revenue_ytd, expenses_ytd, net_profit_ytd, accounts_receivable, accounts_payable, tax_reserve_suggested

## Business Logic

**Profit & Loss:** Aggregated by month, quarter, year. Category breakdown. YoY comparison.

**Tax Reserve Estimation:** `net_profit × self_employment_rate × 0.5 + net_profit × estimated_income_tax_rate`. Suggested reserve held in a "tax bucket."

**Invoice Aging:** Outstanding invoices grouped by: current (< 30 days), late (30–60), overdue (60–90), collections risk (> 90).

**Mileage Tracking:** Track business mileage, apply IRS standard rate (loaded from config, not hardcoded).

## Separation Rules

- Business accounts never merged with personal net worth calculations by default
- Business income flows into tax estimation module via explicit link
- Personal → Business transfers logged as owner contributions
- Business → Personal transfers logged as owner draws

## AI Integration

- Categorize business expenses automatically
- Flag non-deductible expenses miscategorized as deductible
- Suggest tax-saving actions (equipment timing, retirement contributions)
- Identify cash flow gaps between invoice issue and payment

## UI Components

- **Business Dashboard**: P&L summary, cash flow chart, outstanding invoices, tax reserve gauge
- **Invoice Manager**: create, send (future), track, mark paid
- **Expense Tracker**: expense list, receipt attachment, deductibility flag
- **P&L Report**: monthly/quarterly/annual view, exportable
- **Tax Reserve Calculator**: real-time suggestion based on profit

## Events

`InvoiceCreated`, `InvoicePaymentReceived`, `ExpenseAdded`, `BusinessProfitUpdated`

## Quality Gate

- [ ] P&L calculations correct
- [ ] Business/personal data fully separated
- [ ] Tax reserve estimate reasonable
- [ ] Invoice aging accurate
- [ ] AI categorization grounded in business transaction data

---

# SECTION 20 — MORTGAGE & HOME PURCHASE PLANNER

## Purpose

Help users plan for a home purchase, understand their affordability, compare loan options, and track their path to homeownership. Purely a planning and simulation tool — not a lender or application platform.

## Domain Model

**HomeAffordabilityProfile:** id, annual_income, monthly_debts, credit_score_estimate, down_payment_saved, target_purchase_timeline, target_location (optional)

**MortgageScenario:** id, profile_id, loan_amount, down_payment, interest_rate, loan_term_years, loan_type (conventional/FHA/VA/USDA), monthly_payment, total_interest_paid, total_cost

**AffordabilityResult:** max_home_price, max_monthly_payment, front_end_ratio, back_end_ratio, recommended_price_range, down_payment_gap

**MortgageComparison:** scenario_a_id, scenario_b_id, monthly_difference, total_cost_difference, break_even_years (for points)

## Calculation Engine

**Affordability Rules:**
- Front-end ratio (housing costs / gross income): recommended ≤ 28%
- Back-end ratio (all debts / gross income): recommended ≤ 36%
- Maximum loan: use conservative multiplier (3–4× annual income as guideline only)

**Monthly Payment:**
```
M = P × [r(1+r)^n] / [(1+r)^n - 1]
```
Where P = principal, r = monthly rate, n = total payments

**Total Cost of Ownership (annual):**
mortgage payment + property tax estimate + homeowner's insurance + HOA + maintenance reserve (1% of home value/year)

**PMI Calculation:** If down payment < 20%, add PMI estimate (0.5–1.5% of loan value annually, loaded from config range).

**FHA vs Conventional:** MIP (mortgage insurance premium) for FHA modeled correctly including upfront and annual.

**Points Analysis:** Break-even calculation for paying discount points.

## Down Payment Planner

- Link to Goals Engine: create "Home Down Payment" goal
- Track progress toward 3.5% (FHA minimum), 10%, 20% targets
- Project date of reaching target based on current savings rate

## Pre-Approval Readiness Score

Score based on: credit score estimate, DTI ratio, down payment %, employment stability indicator (user-provided). Score: 0–100 with actionable gaps.

## AI Integration

- Explain trade-offs between 15-year vs 30-year loans
- Suggest optimal purchase timing based on savings trajectory
- Identify which debts to pay off first to improve affordability
- Explain how credit score changes affect rate

## UI Components

- **Affordability Calculator**: input income, debts, credit score → output price range
- **Loan Comparison Tool**: side-by-side scenarios (rate, term, type)
- **Amortization Schedule**: full schedule, visualized as principal vs interest chart
- **Down Payment Progress**: linked to Goals Engine, progress ring
- **Total Cost Visualizer**: true 30-year cost including all ownership costs
- **Pre-Approval Readiness**: score card with improvement actions

## Quality Gate

- [ ] Payment formula validated against mortgage calculators
- [ ] Front-end/back-end ratios correct
- [ ] PMI calculation accurate
- [ ] FHA/VA/USDA rules modeled
- [ ] Amortization schedule matches known tables

---

# SECTION 21 — VEHICLE FINANCE & PURCHASE PLANNER

## Purpose

Help users evaluate vehicle affordability, compare loan vs lease, calculate total cost of ownership, and plan a vehicle purchase. Connects to the Vehicle Management system in Section 12.

## Domain Model

**VehicleAffordabilityProfile:** id, annual_income, monthly_debts, down_payment, trade_in_value, credit_score_estimate, target_monthly_payment

**VehicleScenario:** id, vehicle_price, down_payment, trade_in_value, loan_amount, interest_rate, loan_term_months, monthly_payment, total_interest_paid

**LeaseScenario:** id, vehicle_price, capitalized_cost, residual_value, money_factor, lease_term_months, monthly_payment, total_lease_cost, purchase_option_end_of_term

**VehicleTCO:** id, vehicle_id, annual_depreciation, annual_insurance_estimate, annual_fuel_estimate, annual_maintenance_estimate, annual_registration_estimate, total_annual_cost, cost_per_mile

## Calculation Engine

**Loan Payment:** Standard amortization (same formula as mortgage, monthly term).

**Lease Payment:**
```
Monthly depreciation = (cap_cost - residual) / term
Monthly finance charge = (cap_cost + residual) × money_factor
Monthly payment = depreciation + finance_charge + taxes_fees
```

**Loan vs Lease Comparison:**
- Break-even analysis
- 5-year total cost (loan: ownership value remains; lease: no asset)
- Mileage penalty projection for leases

**Total Cost of Ownership (5-year):**
- Depreciation (first 5 years, curve-modeled: ~15%/year average)
- Fuel: `(miles/year / mpg) × fuel_price_per_gallon`
- Insurance estimate (user-provided or historical average from config)
- Maintenance: scheduled intervals + unexpected reserve
- Registration: estimated per state (configurable)

**Affordability Rule:** Vehicle payment ≤ 10–15% of monthly take-home pay (guideline shown, not enforced).

## AI Integration

- Compare specific loan vs lease trade-offs with user's usage pattern
- Identify if current vehicle is TCO-efficient vs replacing
- Suggest optimal down payment to minimize interest without over-depleting savings

## UI Components

- **Affordability Calculator**: income → recommended max monthly payment → max vehicle price
- **Loan Calculator**: payment, total interest, amortization schedule
- **Lease Calculator**: monthly payment, total lease cost, purchase option analysis
- **Loan vs Lease Comparison**: side-by-side, 5-year cost comparison
- **TCO Dashboard**: annual and per-mile cost for owned vehicles
- **Trade-In Analyzer**: impact of trade-in on financing

## Quality Gate

- [ ] Loan payment formula validated
- [ ] Lease money factor formula correct
- [ ] TCO includes all cost categories
- [ ] Loan vs lease comparison deterministic

---

# SECTION 22 — CREDIT ENGINE (DEEP DIVE)

## Purpose

Go beyond displaying a credit score — build a full credit analysis, simulation, and coaching engine that helps users understand and systematically improve their credit profile.

## Domain Model

**CreditReport:** id, score, score_model (FICO_8/FICO_9/VantageScore_3/VantageScore_4), report_date, bureau (Equifax/Experian/TransUnion), utilization_rate, on_time_payment_rate, derogatory_marks, total_accounts, open_accounts, oldest_account_age_months, average_account_age_months, hard_inquiries_last_12m

**CreditAccount:** id, report_id, account_type (revolving/installment/mortgage/auto), name, credit_limit, balance, payment_status, opened_date, closed_date, payment_history (JSON array: on_time/late_30/late_60/late_90)

**CreditInquiry:** id, report_id, creditor_name, inquiry_date, inquiry_type (hard/soft)

**CreditDerogatory:** id, report_id, type (collection/bankruptcy/judgment/foreclosure/charge_off), amount, opened_date, status, expected_removal_date

**ScoreSimulation:** id, report_id, action_type, projected_score_change, confidence

## Credit Scoring Factors (FICO model approximation)

| Factor | Weight | Engine Logic |
|---|---|---|
| Payment History | 35% | On-time rate across all accounts |
| Credit Utilization | 30% | Total balance / total limit, per-card utilization |
| Credit Age | 15% | Average account age, oldest account age |
| Credit Mix | 10% | Variety of account types |
| New Credit | 10% | Hard inquiries in last 12 months |

## Score Simulation Engine

Given a proposed action, compute projected score delta:
- Pay off revolving balance → utilization decrease → score estimate
- Open new card → age decrease + inquiry hit → score estimate
- Close oldest card → age decrease → score estimate
- Dispute and remove collection → derogatory removal → score estimate

Rules: Simulation results displayed with confidence range (not exact — inherently approximate). Never claim to predict exact score — advisory only.

## Utilization Optimizer

- Per-card and aggregate utilization
- Recommend target: ≤ 30% per card, ≤ 10% for maximum score benefit
- Show dollar amount to pay to reach each threshold
- Alert when utilization spikes

## AI Integration

- Personalized improvement plan ranked by impact
- Explain why score changed since last report
- Identify which derogatory marks are disputable vs permanent
- Forecast expected score in 6/12/24 months if current trends continue

## UI Components

- **Score History Chart**: trend line per bureau
- **Factor Breakdown**: 5-factor gauge visualization
- **Account Health Grid**: each account with status, utilization, payment history timeline
- **Score Simulator**: action → estimated score change
- **Derogatory Tracker**: items with removal dates, dispute status
- **Inquiry Log**: hard inquiries with 12-month count and impact estimate
- **Improvement Plan**: AI-generated ranked action list

## Quality Gate

- [ ] Score factors modeled correctly
- [ ] Utilization calculations accurate
- [ ] Simulation shows correct directional changes
- [ ] No claims of exact score prediction
- [ ] Derogatory removal dates calculated correctly

---

# SECTION 23 — DEBT PAYOFF ENGINE (DEEP DIVE)

## Purpose

Go beyond listing debts — build a full debt elimination strategy engine with multiple payoff methods, simulation, interest projection, and acceleration tools.

## Domain Model

**Debt:** id, name, type (credit_card/student_loan/personal_loan/medical/auto/other), principal, remaining_balance, interest_rate (APR), minimum_payment, due_date, creditor, promotional_rate_end_date (for 0% APR), is_in_collections

**DebtPayoffPlan:** id, strategy (snowball/avalanche/hybrid/custom), extra_monthly_payment, created_at, projected_payoff_date, total_interest_saved_vs_minimum, payoff_order (JSON array of debt_ids)

**DebtPayoffSchedule:** id, plan_id, debt_id, month, payment_amount, interest_charged, principal_paid, remaining_balance

**BalanceTransferScenario:** id, source_debt_id, transfer_amount, promotional_rate, promotional_period_months, transfer_fee, monthly_payment_required, interest_savings

## Payoff Strategies

**Snowball:** Sort by balance ascending. Pay minimums on all, attack lowest balance first. Psychological wins, faster payoffs on individual debts.

**Avalanche:** Sort by interest rate descending. Pay minimums on all, attack highest rate first. Minimizes total interest paid.

**Hybrid:** Score each debt: `weight = (interest_rate × 0.6) + (1 / balance × 0.4)`. Balance between psychological and mathematical optimization.

**Custom:** User-defined payoff order, drag-and-drop reordering.

**Extra Payment Allocation:** When any debt is paid off, roll that payment into the next target (debt cascade / snowball roll).

## Calculation Engine

Monthly simulation loop:
```
for each month:
  for each debt (ordered by strategy):
    interest = balance × (apr / 12)
    payment = min(balance + interest, scheduled_payment)
    balance = balance + interest - payment
  remaining = extra_payment
  apply remaining to target debt
```

Run until all balances = 0. Record full schedule.

## Interest Projection

- Total interest paid with current minimums only
- Total interest paid with each strategy
- Interest saved = `minimum_only_total - strategy_total`
- Months saved = `minimum_only_payoff - strategy_payoff`

## Balance Transfer Analysis

- Input: transfer fee, promo rate, promo period
- Calculate: monthly payment required to clear before promo ends
- Compare: interest cost with transfer vs without
- Alert: if transfer fee exceeds projected interest savings

## AI Integration

- Recommend optimal strategy based on user profile (debt-to-income, income stability, psychological patterns inferred from spending)
- Explain why avalanche saves more but snowball may be better for this user
- Identify refinancing or consolidation opportunities
- Flag promotional rate expiry dates with urgency

## UI Components

- **Debt Overview**: total debt, total interest projected, payoff date
- **Strategy Selector**: snowball / avalanche / hybrid / custom side-by-side comparison
- **Payoff Timeline Chart**: stacked bar chart showing each debt disappearing over time
- **Interest Meter**: running total of interest saved vs minimum payments
- **Extra Payment Slider**: drag to see impact on payoff date and interest
- **Balance Transfer Calculator**: analyze specific transfer scenarios
- **Debt Cascade Visualizer**: animated payoff order with rolling payments

## Quality Gate

- [ ] All three strategy calculations produce correct schedules
- [ ] Interest math validated against known debt calculators
- [ ] Balance transfer analysis correct
- [ ] Extra payment cascade works correctly
- [ ] Edge cases: 0% APR, promotional rate expiry, minimum-only scenario

---

# SECTION 24 — INVESTMENT & PORTFOLIO ENGINE

## Purpose

Track investment portfolios, visualize asset allocation, monitor performance, and provide AI-assisted analysis. Manual tracking only in v1 (no brokerage API integration). Brokerage API integration is a future roadmap item.

## Domain Model

**InvestmentAccount:** id, name, type (brokerage/roth_ira/traditional_ira/401k/hsa/529/crypto/other), institution, balance, cost_basis

**Holding:** id, account_id, asset_type (stock/etf/mutual_fund/bond/crypto/real_estate/commodity/other), symbol, name, quantity, purchase_price, current_price, purchase_date

**PriceSnapshot:** id, holding_id, price, captured_at (manual entry or future API)

**DividendRecord:** id, holding_id, amount, dividend_date, dividend_type (cash/reinvested)

**PortfolioSnapshot:** id, captured_at, total_value, total_cost_basis, total_gain_loss, asset_allocation (JSON)

## Performance Calculations

**Per Holding:**
- Cost basis: `quantity × purchase_price + fees`
- Current value: `quantity × current_price`
- Gain/Loss: `current_value - cost_basis`
- Return %: `(gain_loss / cost_basis) × 100`
- Annualized return: CAGR formula: `((current / cost_basis)^(1/years)) - 1`

**Portfolio Level:**
- Total value, total cost basis, total gain/loss
- Asset allocation by type, sector (future), geography (future)
- Weighted average return

**Benchmark Comparison (manual entry baseline):**
- User inputs benchmark return for period (S&P 500 etc.)
- Compare portfolio CAGR vs benchmark

## Rebalancing Engine

- User defines target allocation (e.g., 60% stocks, 30% bonds, 10% other)
- Engine computes current vs target deviation per category
- Output: which holdings are over/underweight, suggested rebalancing actions
- Never automatically execute trades — advisory only

## Dividend Tracking

- Record dividend payments
- Project annual dividend income from current holdings
- Dividend yield: `annual_dividend / current_price × 100`

## AI Integration

- Analyze concentration risk (over-reliance on single holdings/sectors)
- Suggest diversification improvements
- Explain performance relative to benchmarks
- Identify tax-loss harvesting opportunities (flagging positions with unrealized losses)
- Retirement integration: project portfolio growth to retirement date

## UI Components

- **Portfolio Dashboard**: total value, gain/loss, allocation donut chart
- **Holdings Table**: sortable by gain/loss, return %, allocation weight
- **Performance Chart**: portfolio value over time (user-entered snapshots)
- **Allocation Analyzer**: current vs target allocation, rebalancing suggestions
- **Dividend Calendar**: upcoming and historical dividend payments
- **Gain/Loss Summary**: realized vs unrealized, tax implications flag

## Quality Gate

- [ ] CAGR formula validated
- [ ] Allocation percentages sum to 100%
- [ ] Rebalancing suggestions mathematically correct
- [ ] Dividend yield calculation accurate
- [ ] No automatic trade execution possible

---

# SECTION 25 — FINANCIAL GOALS ENGINE

## Purpose

Enable users to define, track, and achieve financial goals of any type. Goals are first-class citizens — linked to accounts, budgets, and debt plans. Progress updates drive events across the system.

## Domain Model

**Goal:** id, name, type (savings/debt_payoff/home_purchase/vehicle/emergency_fund/education/retirement/vacation/custom), target_amount, current_amount, monthly_contribution, deadline, status (active/paused/achieved/abandoned), priority (1–5), linked_account_id, linked_debt_id

**GoalMilestone:** id, goal_id, name, target_amount, achieved_at

**GoalContribution:** id, goal_id, amount, contribution_date, source (manual/auto_transfer/bonus/other)

**GoalProjection:** id, goal_id, computed_at, projected_achievement_date, monthly_required_to_meet_deadline, shortfall_amount, on_track (bool)

## Goal Types

| Type | Logic |
|---|---|
| Savings | Track balance toward target |
| Emergency Fund | Target = N months of expenses (computed from transaction history) |
| Debt Payoff | Mirror linked debt's remaining balance → target = 0 |
| Home Purchase | Track down payment savings, integrates with Mortgage Planner |
| Vehicle | Track savings, integrates with Vehicle Planner |
| Retirement | Mirror Retirement Engine projection readiness |
| Education (529) | Project college cost estimate, track savings |
| Custom | User-defined target and timeline |

## Projection Engine

- `months_remaining = (target - current) / monthly_contribution`
- `projected_date = today + months_remaining`
- `shortfall = if projected_date > deadline → (deadline_months - months_remaining) × monthly_contribution`
- `monthly_required = (target - current) / months_until_deadline`

## Priority & Conflict Resolution

When multiple goals compete for the same monthly budget surplus:
- Rank by user priority (1 = highest)
- Emergency fund always funded first (system rule, overridable)
- AI can suggest optimal allocation across competing goals
- Show trade-off: if user funds Goal A faster, Goal B slips by N months

## Milestone System

- Milestones auto-generated at 25%, 50%, 75%, 100% of target
- User can define custom milestones
- `GoalMilestoneReached` event triggers notification + celebration UI

## AI Integration

- Suggest goals based on financial profile (no emergency fund → create suggestion)
- Auto-generate monthly contribution plan across all goals
- Identify which goals are at risk of missing deadline
- Surface "unexpected savings opportunity" when monthly surplus exceeds plan

## UI Components

- **Goals Dashboard**: goal cards with progress rings, days remaining
- **Goal Detail**: contribution history, projection chart, milestone timeline
- **Contribution Planner**: monthly surplus allocation across all goals
- **Goal Conflict Visualizer**: trade-off chart when goals compete
- **Achievement Screen**: celebration UI when goal reached
- **Goal Creation Wizard**: guided flow with type selection, smart defaults

## Events

`GoalCreated`, `GoalContributionAdded`, `GoalMilestoneReached`, `GoalAchieved`, `GoalAtRisk`, `GoalProjectionUpdated`

## Quality Gate

- [ ] All projection calculations correct
- [ ] Emergency fund goal uses actual expense data
- [ ] Conflict resolution handles edge cases
- [ ] Events fire correctly on milestone/achievement
- [ ] Goal types correctly modeled

---

# SECTION 26 — VOICE & NATURAL LANGUAGE INTERFACE

## Purpose

Allow users to interact with Phoenix LifeOS using voice commands or natural language text. Commands execute through the same service layer as all other operations — voice is an input channel, not a privileged path.

## Architecture

```
Voice Input (Microphone) / Text Input
    ↓
Speech-to-Text Engine (local first)
    ↓
Natural Language Parser
    ↓
Intent Classifier
    ↓
Command Router → Domain Services (same as UI calls)
    ↓
Response Generator → Text-to-Speech / Screen Response
```

## Speech-to-Text

**Primary (local):** Whisper.cpp via Rust bindings (`whisper-rs`) — runs fully offline, no data sent externally.

**Fallback (opt-in):** OS native speech API (if user grants permission and opts into cloud).

**Activation:** Push-to-talk (not always-on by default), keyboard shortcut, or voice wake word (opt-in).

## Intent Classification

**Intent types:**
- `NavigateTo` — "Open budget", "Go to credit score"
- `CreateEntity` — "Add a transaction for $45 at Walmart"
- `QueryData` — "How much did I spend on food this month?"
- `TriggerAction` — "Generate my monthly report", "Run debt payoff simulation"
- `AskAI` — "Why did my credit score drop?", "Am I on track to retire at 60?"
- `VaultAccess` — requires re-auth before execution

**Extraction:** Use local LLM (Ollama) or structured regex for entity extraction from transcribed text.

## Command Safety Rules

- Voice commands **never** bypass authorization checks
- Vault access commands require re-authentication (PIN or password) before execution
- Destructive commands (delete, export all data) require confirmation
- Voice commands are logged (transcript stored, not raw audio)
- Raw audio **never** persisted to disk

## Response System

- Short responses: text displayed in overlay + optional TTS readback
- Long responses: display on screen, TTS reads summary only
- TTS engine: system TTS or local TTS model (configurable)
- Voice feedback disabled by default — opt-in

## UI Components

- **Voice Activation Button**: mic icon in command bar, push-to-talk
- **Transcript Preview**: shows recognized text before execution (with cancel option)
- **Voice Response Overlay**: brief response card
- **Voice History**: log of recent voice commands (text only, no audio)
- **Voice Settings**: enable/disable, TTS toggle, wake word config (future)

## Implementation Order

1. Whisper.cpp Rust integration → 2. Basic intent classification → 3. Command router → 4. Navigation commands → 5. Query commands → 6. Create commands → 7. AI delegation → 8. TTS response → 9. Voice settings UI

## Quality Gate

- [ ] Whisper.cpp processes audio offline correctly
- [ ] No audio persisted to disk
- [ ] Intent classification handles 80%+ of common commands
- [ ] Authorization not bypassed
- [ ] Vault access requires re-auth

---

# SECTION 27 — PLUGIN SDK & EXTENSION SYSTEM

## Purpose

Allow third-party developers (and advanced users) to extend Phoenix LifeOS with custom capabilities, data sources, UI widgets, and automations — while maintaining strict security boundaries.

## Plugin Architecture

```
Plugin Bundle (.plx file)
├── manifest.json       — metadata, permissions, version
├── plugin.wasm         — sandboxed WASM business logic
├── ui/                 — optional React components
├── migrations/         — optional DB schema extensions
└── assets/             — icons, resources
```

**Execution model:** WASM sandbox via `wasmtime`. Plugins cannot access the filesystem, network, or system APIs directly — all access is through declared, permission-gated API calls.

## Plugin Manifest

```json
{
  "id": "com.example.my-plugin",
  "name": "My Plugin",
  "version": "1.0.0",
  "permissions": ["read_finance", "create_transaction", "add_widget"],
  "entry": "plugin.wasm",
  "ui_components": ["MyWidget"],
  "min_app_version": "1.0.0"
}
```

## Permission System

Declared permissions are user-approved at install time. No silent capability escalation.

| Permission | Allows |
|---|---|
| `read_finance` | Read transactions, balances (no vault) |
| `create_transaction` | Create transactions (user-confirmed) |
| `read_goals` | Read goal progress |
| `add_widget` | Register a dashboard widget |
| `schedule_job` | Register background jobs |
| `read_documents_metadata` | Access document names/tags (not contents) |
| `write_settings` | Store plugin-specific settings |

Vault, encryption keys, raw documents, and passwords are **never** accessible to plugins under any permission.

## Plugin API (Rust Host Functions exposed to WASM)

```
phoenix_api::finance::get_summary() → FinanceSummary
phoenix_api::transactions::list(filter) → Vec<TransactionDTO>
phoenix_api::transactions::create(draft) → Result<TransactionId>
phoenix_api::goals::get_all() → Vec<GoalDTO>
phoenix_api::storage::get(key) → Option<String>
phoenix_api::storage::set(key, value)
phoenix_api::notifications::send(title, body)
```

## Plugin Lifecycle

`Discovered → Verified → Installed → Enabled → [Running] → Disabled → Removed`

State transitions require user confirmation for install, enable, and remove. Upgrades show permission diff.

## UI Extension Points

Plugins may contribute:
- Dashboard widgets (rendered in sandboxed iframe or React component boundary)
- Sidebar nav items
- Command palette entries
- Settings pages

## Plugin Security Rules

- WASM sandboxed: no direct OS access
- Network access: denied by default (future: optional with explicit permission + user approval)
- Storage: isolated per-plugin namespace, never access other plugins' data
- Signed plugins (future): cryptographic signature verification before install
- Audit log: all plugin API calls logged

## Developer Experience

- Plugin CLI tool: `phoenix-plugin-cli init`, `build`, `package`, `test`
- Local sideloading during development
- Plugin testing harness with mock API
- Plugin documentation generated from manifest

## Quality Gate

- [ ] WASM sandbox enforced
- [ ] Permission system blocks undeclared access
- [ ] Plugin install/uninstall cleans up correctly
- [ ] UI extension points render without main app instability
- [ ] Plugin storage isolated

---

# SECTION 28 — UI DESIGN SYSTEM & COMPONENT LIBRARY

## Purpose

Establish a unified, consistent, accessible design language across the entire application. All UI components derive from this system — no ad-hoc styling allowed.

## Design Tokens

All visual properties defined as tokens, consumed via CSS variables and Tailwind config:

**Color:**
```
--color-primary:       hsl(220 90% 56%)
--color-primary-hover: hsl(220 90% 48%)
--color-surface-0:     hsl(220 15% 10%)   /* app background */
--color-surface-1:     hsl(220 15% 14%)   /* card */
--color-surface-2:     hsl(220 15% 18%)   /* elevated card */
--color-border:        hsl(220 15% 22%)
--color-text-primary:  hsl(0 0% 95%)
--color-text-muted:    hsl(0 0% 60%)
--color-success:       hsl(142 72% 45%)
--color-warning:       hsl(38 92% 50%)
--color-danger:        hsl(0 84% 60%)
--color-info:          hsl(196 85% 50%)
```

**Spacing:** 4px base grid. Scale: 4 / 8 / 12 / 16 / 24 / 32 / 48 / 64 / 96

**Typography:**
- Font family: Inter (primary), JetBrains Mono (code/numbers)
- Scale: 11 / 12 / 14 / 16 / 18 / 20 / 24 / 30 / 36 / 48
- Weight: 400 (body), 500 (medium), 600 (semibold), 700 (bold)
- Line height: 1.2 (headings), 1.5 (body), 1.7 (long-form)

**Border Radius:** 4px / 8px / 12px / 16px / full (pill)

**Shadow:**
- `shadow-sm`: 0 1px 3px hsl(220 20% 4% / 0.3)
- `shadow-md`: 0 4px 12px hsl(220 20% 4% / 0.4)
- `shadow-lg`: 0 12px 32px hsl(220 20% 4% / 0.5)

## Glassmorphism Utility

```css
.glass {
  background: hsl(220 15% 14% / 0.7);
  backdrop-filter: blur(12px);
  border: 1px solid hsl(220 15% 22% / 0.6);
}
```

Used sparingly: modal overlays, floating panels, command palette.

## Core Component Catalog

| Component | Variants |
|---|---|
| Button | primary / secondary / ghost / danger / icon-only |
| Input | text / number / date / password / search / textarea |
| Select | single / multi-select / combobox |
| Card | default / elevated / interactive / glass |
| Badge | success / warning / danger / info / neutral |
| Modal | sm / md / lg / fullscreen |
| Drawer | left / right / bottom |
| Tabs | underline / pill / vertical |
| Table | sortable / selectable / virtualized |
| Toast | success / error / warning / info (auto-dismiss) |
| Tooltip | above / below / left / right |
| Dropdown | menu / context menu |
| Progress | bar / ring / segmented |
| Avatar | with/without image, size variants |
| Skeleton | text / card / table row |
| Command Palette | — |
| Sidebar | collapsed / expanded / hover-expand |
| Chart Wrappers | line / bar / donut / area (Recharts base) |

## Dark & Light Mode

Dark mode is default. Light mode fully supported via token swap — no hardcoded colors anywhere in components.

Mode switch: `<html data-theme="dark|light">`. All tokens resolve to correct values per theme.

## Animation Standards (Framer Motion)

- Page transitions: 200ms ease-out fade + slight upward translate
- Modal open/close: 150ms scale from 0.96 → 1 + fade
- Drawer: 200ms slide from edge
- Toast: 200ms slide up + fade, auto-dismiss at 4s
- Loading states: skeleton shimmer, never spinner-only
- Rule: no animation > 300ms on interactive elements (feels sluggish)

## Accessibility Standards

- WCAG 2.1 AA minimum
- All interactive elements keyboard-navigable
- Focus ring always visible (never `outline: none` without replacement)
- Color contrast: ≥ 4.5:1 for text, ≥ 3:1 for large text
- ARIA labels on all icon-only buttons
- Screen reader tested with NVDA / Orca

## Component Documentation

Each component documented with: usage example, prop table, accessibility notes, do/don't examples.

## Quality Gate

- [ ] All tokens defined and consumed (no hardcoded colors in components)
- [ ] Dark and light mode both correct
- [ ] All components keyboard-accessible
- [ ] No contrast violations
- [ ] Animation timings within spec
- [ ] Glassmorphism used sparingly

---

# SECTION 29 — TESTING STRATEGY & QUALITY ASSURANCE

## Philosophy

Tests are a safety net, not a checkbox. Tests must be: fast, deterministic, independent, and meaningful. Test the behavior, not the implementation.

## Test Pyramid

```
        ┌─────┐
        │ E2E │  (few, slow, high-value flows)
       ┌┴─────┴┐
       │ Integ │  (service + DB interactions)
      ┌┴───────┴┐
      │  Unit   │  (domain logic, calculations, value objects)
     └──────────┘
```

**Coverage targets:**
- Domain layer (calculations, rules): 95%+
- Service layer: 80%+
- Infrastructure layer: 60%+
- Frontend components: 70%+ (focused on logic, not rendering)

## Rust Testing

**Unit tests** (inline `#[cfg(test)]` modules):
- All value objects: validation, edge cases, arithmetic
- All financial calculations: known inputs → known outputs
- All business rules: budget limits, debt logic, credit utilization

**Integration tests** (`tests/` directory per crate):
- Repository operations against real SQLite in-memory database
- Service workflows: create → read → update → verify state
- Event publishing and subscription

**Test utilities:**
- `TestDatabase::new()` — in-memory SQLite with all migrations applied
- `DomainFixtures::transaction()`, `::account()`, etc. — consistent test data builders
- `MockEventBus` — capture and assert on published events

## Frontend Testing (Vitest + React Testing Library)

**Unit tests:**
- Pure utility functions, formatters, calculations
- Custom hooks (with `renderHook`)

**Component tests:**
- Render, user interaction, state changes
- NOT snapshot tests (brittle, low signal)
- Mock IPC service layer — never call real Rust from frontend tests

**Rules:**
- No testing implementation details (avoid testing internal state)
- Use `userEvent` not `fireEvent` for realistic interaction simulation
- Accessible queries first: `getByRole`, `getByLabelText` over `getByTestId`

## End-to-End Tests (Playwright)

**Scope:** Critical golden paths only. Not exhaustive coverage.

**E2E test cases:**
1. App launch → dashboard renders
2. Create account → add transaction → verify balance
3. Create budget → add transactions → verify overspend alert
4. Add debt → select payoff strategy → verify schedule
5. Upload document → verify OCR result
6. Lock screen → re-authenticate
7. Export report → verify file created

**Environment:** E2E runs against the full Tauri app in headless mode.

## Test Data Management

- Financial test fixtures use realistic but clearly synthetic data
- Never use real personal financial data in tests
- Database seeding scripts: `scripts/test/seed_test_data.sh`
- Deterministic random data via seeded RNG in test utilities

## Performance Tests

Benchmark critical paths:
- 100k transaction query < 100ms
- Dashboard render time
- Report generation time
- Encryption/decryption round-trip

Use Rust's `criterion` for benchmarks.

## Continuous Testing

- All tests run on every commit (GitHub Actions)
- Tests must pass before any merge
- Test failures never silenced — investigate, don't skip

## Quality Gate

- [ ] Domain calculation tests achieve 95%+ coverage
- [ ] Integration tests use real SQLite (not mocks)
- [ ] E2E golden paths all passing
- [ ] No flaky tests in CI
- [ ] Performance benchmarks established

---

# SECTION 30 — CI/CD PIPELINE & BUILD SYSTEM

## Purpose

Automated, reproducible, fast pipeline that enforces quality on every commit and produces reliable release artifacts.

## Pipeline Overview

```
Push / PR
   ↓
[Validate] lint + type-check + format-check
   ↓
[Test] Rust unit + integration + Frontend unit
   ↓
[Build] Rust compilation + Frontend build
   ↓
[Security] cargo-audit + npm audit + secret scan
   ↓
[E2E] Playwright against built Tauri app (main branch only)
   ↓
[Release] Tauri build for all platforms (tag push only)
```

## GitHub Actions Workflows

**`ci.yml`** — triggers on every push and PR:
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test --workspace`
- `pnpm run lint`
- `pnpm run type-check`
- `pnpm run test`
- `cargo build` (verify compilation)

**`security.yml`** — triggers on push to main:
- `cargo audit` — check for known Rust CVEs
- `pnpm audit` — check for JS CVEs
- `gitleaks detect` — scan for accidentally committed secrets
- Output: security report as PR comment

**`release.yml`** — triggers on `v*` tag push:
- Build for: Linux (x86_64), Windows (x86_64), macOS (x86_64 + arm64)
- Sign binaries (future: code signing certificates)
- Attach artifacts to GitHub Release
- Generate release notes from commit log

## Build Requirements

**Rust:**
- Toolchain: stable (pinned in `rust-toolchain.toml`)
- MSRV: defined and enforced
- All warnings treated as errors in CI

**Frontend:**
- Node.js version pinned in `.nvmrc`
- `pnpm` as package manager
- Type-check with `tsc --noEmit`
- No TypeScript errors allowed

## Caching

- GitHub Actions cache for: `~/.cargo/registry`, `~/.cargo/git`, `target/`, `node_modules/`
- Cache keyed by `Cargo.lock` + `pnpm-lock.yaml`
- Significant CI speed improvement after warm cache

## Branch Protection Rules

Main branch requires:
- All CI checks passing
- At least one reviewer approval (future team workflow)
- No force pushes
- No direct commits

## Quality Gate

- [ ] CI runs in < 10 minutes (warm cache)
- [ ] All checks enforced before merge
- [ ] Security scan runs on main branch
- [ ] Release artifacts build for all platforms
- [ ] No secrets in repository (gitleaks clean)

---

# SECTION 31 — DEPLOYMENT & DISTRIBUTION

## Purpose

Define how Phoenix LifeOS is packaged, distributed, installed, and updated across target platforms.

## Target Platforms

| Platform | Format | Priority |
|---|---|---|
| Fedora / Ubuntu Linux | `.deb`, `.rpm`, AppImage | Primary |
| Windows 10/11 | `.msi`, NSIS installer | Secondary |
| macOS 12+ | `.dmg`, `.pkg` | Secondary |

## Tauri Build System

Tauri handles cross-platform packaging. Configuration in `tauri.conf.json`:

```json
{
  "bundle": {
    "identifier": "com.phoenixlifeos.app",
    "icon": ["icons/32x32.png", "icons/icon.icns", "icons/icon.ico"],
    "targets": ["deb", "rpm", "appimage", "msi", "dmg"]
  }
}
```

Platform-specific signing handled by Tauri's built-in signing support (certificates loaded from environment secrets in CI, never committed).

## Auto-Update System

Tauri's built-in updater (`tauri-plugin-updater`):

- Update server: GitHub Releases (JSON endpoint)
- Check frequency: on startup, then every 24 hours
- Update types: silent background download, prompt user before install
- Differential updates: future (full bundle in v1)
- Rollback: keep previous version archive for 1 version

**Update manifest format:**
```json
{
  "version": "1.2.0",
  "notes": "Release notes",
  "pub_date": "2026-01-15T00:00:00Z",
  "platforms": {
    "linux-x86_64": { "url": "...", "signature": "..." },
    "windows-x86_64": { "url": "...", "signature": "..." },
    "darwin-x86_64": { "url": "...", "signature": "..." }
  }
}
```

## Installation Experience

- **Linux:** AppImage (no install required, single file), plus `.deb`/`.rpm` for package manager install
- **Windows:** NSIS installer with Start Menu shortcut, optional auto-start at login
- **macOS:** DMG with drag-to-Applications install

## Data Migration on Upgrade

- Automatic DB migration on startup
- Migration failures must halt startup with clear error and recovery instructions
- Never silently corrupt data on upgrade
- Pre-migration backup created automatically before running migrations

## Release Naming Convention

`v<major>.<minor>.<patch>` — semantic versioning

- Major: breaking changes (rare)
- Minor: new features
- Patch: bug fixes

## Quality Gate

- [ ] Builds successfully on all 3 platforms
- [ ] Auto-updater endpoint reachable
- [ ] Migration runs successfully on upgrade
- [ ] Pre-migration backup created
- [ ] Binary signing configured

---

# SECTION 32 — DATA IMPORT & MIGRATION TOOLS

## Purpose

Allow users to get their data into Phoenix LifeOS from their existing financial tools. Import is a critical onboarding experience — friction here prevents adoption.

## Supported Import Sources

| Source | Format | Priority |
|---|---|---|
| Bank/Credit Union CSV | Generic CSV | High |
| OFX / QFX | Open Financial Exchange | High |
| Mint | CSV export | High |
| YNAB | JSON / CSV export | Medium |
| Quicken | QIF / OFX | Medium |
| Apple Card | CSV | Medium |
| Chase / BoA / WF | Bank-specific CSV | Medium |
| Manual entry | In-app form | Always |

## Import Pipeline

```
File Upload → Format Detection → Parser → Field Mapping UI → Duplicate Detection → Preview → User Confirmation → Import
```

**Rules:**
- Never import without user review of field mapping
- Always show preview before committing
- Duplicate detection: hash of (date + amount + description) — flag potential duplicates for user decision
- Failed rows logged and shown to user; never silently dropped
- Import is transactional: all-or-nothing per batch

## CSV Parser

Generic CSV parser with configurable field mapping:

```
User maps: date column, amount column, description column, optional category column
```

Parser handles: various date formats (ISO, US, UK), positive/negative amounts, credit vs debit notation, quoted strings, different encodings.

## OFX/QFX Parser

Parse the SGML-like OFX format:
- `<TRNAMT>`, `<DTPOSTED>`, `<MEMO>`, `<NAME>`, `<TRNTYPE>`
- Map to internal Transaction model

## Duplicate Detection

During import, for each incoming transaction:
1. Compute `hash(date, amount, description_normalized)`
2. Check against last 90 days of existing transactions
3. Flag potential duplicates with confidence level (exact match vs fuzzy)
4. User resolves: import anyway / skip / merge

## Field Mapping UI

Drag-and-drop column assignment:
- Upload file → preview first 5 rows
- User assigns: "this column is the date", "this column is the amount"
- System validates: can it parse the detected format?
- Save mapping profile for this institution (reuse next time)

## Category Mapping

On import, if source file has categories:
- Map source categories to Phoenix LifeOS categories
- User can create mapping table (e.g., Mint "Restaurants" → Phoenix "Dining")
- AI can suggest mappings

## Mint Migration Assistant

Dedicated flow for Mint users (large migration use case):
- Import accounts, transactions, budgets, goals from Mint export
- Guided wizard with explanations
- Handles Mint's specific CSV format quirks

## Quality Gate

- [ ] CSV parser handles common bank formats
- [ ] OFX parser correctly extracts transactions
- [ ] Duplicate detection has < 5% false positive rate
- [ ] Failed rows reported, not silently dropped
- [ ] Preview before commit always shown
- [ ] Import is fully transactional

---

# SECTION 33 — LOCALIZATION & ACCESSIBILITY

## Localization (i18n)

**v1 scope:** English only. Architecture must be i18n-ready from the start.

**i18n Architecture:**
- All user-facing strings extracted to translation files: `locales/en/common.json`, `locales/en/finance.json`, etc.
- No hardcoded English strings in component code
- `react-i18next` as i18n framework
- Translation keys follow: `module.feature.label` pattern
- Date formatting: `Intl.DateTimeFormat` — locale-aware, no hardcoded formats
- Number formatting: `Intl.NumberFormat` — currency, percentages
- Currency: user-configurable base currency (USD default), amounts displayed per locale convention

**Future languages:** Spanish, French, German, Portuguese, Japanese. Architecture ready; strings ready for extraction.

## Accessibility (a11y)

**Target standard:** WCAG 2.1 Level AA

**Keyboard Navigation:**
- Full application navigable without mouse
- Logical tab order throughout
- Keyboard shortcuts documented (see Command Palette)
- Skip-to-content link on app load
- No keyboard traps

**Screen Reader Support:**
- All images: descriptive `alt` text
- All icon-only buttons: `aria-label`
- Dynamic content updates: `aria-live` regions for notifications, loading states
- Charts: `aria-describedby` with text summary of key data points
- Modals: `aria-modal="true"`, focus trapped inside, focus returns on close
- Tables: proper `<th scope>`, `<caption>`, `aria-sort` for sortable columns

**Color & Contrast:**
- Text contrast: ≥ 4.5:1 (normal), ≥ 3:1 (large)
- UI component contrast: ≥ 3:1
- Never convey information by color alone (always add icon or text)
- High contrast mode: additional CSS layer for users who need it

**Motion & Animation:**
- Respect `prefers-reduced-motion` media query
- When reduced motion enabled: fade-only transitions, no sliding/scaling

**Font & Sizing:**
- Minimum 14px for body text
- Respect OS font size preferences (no fixed pixel overrides on root)
- Line spacing ≥ 1.5 for body text

## Accessibility Testing

- Automated: `axe-core` via Playwright in CI
- Manual: keyboard-only walkthrough of each module before release
- Screen reader: manual testing with Orca (Linux), NVDA (Windows)

## Quality Gate

- [ ] All strings externalized to locale files
- [ ] Dates and currency use `Intl` APIs
- [ ] Zero axe-core violations in CI
- [ ] Keyboard-only walkthrough passes
- [ ] `prefers-reduced-motion` respected
- [ ] Contrast ratios validated

---

# SECTION 34 — PERFORMANCE ENGINEERING

## Performance Budget

| Metric | Target |
|---|---|
| App cold start (first launch) | < 3 seconds |
| App warm start (subsequent) | < 1.5 seconds |
| Dashboard initial render | < 1 second |
| Transaction table (10k rows) | < 200ms |
| Transaction table (100k rows) | < 500ms |
| Report generation (12 months) | < 2 seconds |
| AI response (local, first token) | < 3 seconds |
| Search query | < 100ms |
| Chart render | < 150ms |

## Frontend Performance

**Bundle Optimization:**
- Vite code splitting: each feature module is a separate chunk
- Dynamic import for non-critical features
- Tree-shaking enabled
- Target bundle size: < 500KB initial load (gzipped)

**Rendering Optimization:**
- `React.memo` for expensive pure components
- `useMemo` / `useCallback` where profiling shows benefit (not preemptively)
- Virtualized lists for all large data sets (`@tanstack/react-virtual`)
- Deferred widget rendering on dashboard (render above-fold widgets first)
- Avoid layout thrashing: batch DOM reads/writes

**State Management:**
- TanStack Query cache prevents redundant IPC calls
- Stale-while-revalidate: show cached data immediately, update in background
- No unnecessary global re-renders: store slices scoped to features

## Backend Performance

**Database:**
- All dashboard queries indexed (explain plan validated)
- Prepared statements only
- WAL mode — concurrent reads during writes
- Avoid `SELECT *` — explicit column selection
- Paginate all large queries (cursor-based pagination for transactions)
- Connection pool: single SQLite connection but properly managed via SQLx

**Rust Async:**
- Tokio multi-thread runtime
- CPU-bound work (OCR, encryption) off the async executor via `spawn_blocking`
- No blocking calls in async context
- Structured concurrency: tasks cancelled cleanly on shutdown

**Memory:**
- Target: < 300MB resident during normal operation
- Large file processing streamed, not fully loaded into memory
- AI context windows managed to avoid memory bloat
- Encryption keys cleared from memory after use

## Profiling Tools

**Frontend:** Chrome DevTools Performance tab, React DevTools Profiler

**Backend:** `tokio-console` for async task inspection, `flamegraph` for CPU profiles, `criterion` for microbenchmarks

**Database:** SQLite `EXPLAIN QUERY PLAN` for all new queries

## Performance Regression Detection

- Benchmark suite in CI: fail if critical benchmark regresses > 20%
- Bundle size tracked: fail if initial bundle exceeds budget
- Startup time monitored in E2E tests

## Quality Gate

- [ ] All performance targets met on reference hardware (mid-range desktop)
- [ ] No render-blocking operations on main thread
- [ ] Database queries explain-plan validated
- [ ] Bundle size within budget
- [ ] Benchmark suite established and passing

---

# SECTION 35 — DEVELOPER MODE & DEBUG TOOLS

## Purpose

Give developers (and power users) visibility into system internals without cluttering the production UI. Debug tools are development-only overlays activated by environment flag or keyboard shortcut.

## Activation

- Environment: `PHOENIX_DEV_MODE=true` (set in `.env.local`)
- Keyboard shortcut: `Ctrl+Shift+D` (only when `PHOENIX_DEV_MODE=true`)
- Never available in production builds without explicit flag

## Debug Overlay Components

**Event Stream Viewer:**
- Live stream of domain events as they fire
- Filter by event type, entity type
- Timestamps, payload preview (no sensitive fields)
- Pause/resume, clear log

**Database Inspector:**
- View table row counts
- Run read-only SQL queries against app database
- View recent queries and execution times
- View migration history and current schema version

**IPC Monitor:**
- Log all Tauri IPC calls: command name, payload (sanitized), response time, success/error
- Highlight slow IPC calls (> 500ms)

**State Snapshot:**
- Dump current TanStack Query cache state
- View feature store states
- Force invalidate cache entries

**Background Job Monitor:**
- View job queue: pending / running / completed / failed
- Retry failed jobs manually
- View job execution logs

**Mock Data Generator:**
- Generate N months of synthetic transaction data
- Generate test accounts, debts, goals with realistic values
- Generate credit score history
- One-click: populate all modules with coherent demo data

**Performance Panel:**
- Real-time render time per component
- IPC call latency histogram
- Memory usage (Tauri process + WebView)

## Production Safety

- Debug overlay code is excluded from production builds via compile-time feature flag: `#[cfg(debug_assertions)]` in Rust, `import.meta.env.DEV` in frontend
- No debug endpoints accessible in production binary
- No sensitive data (keys, vault contents) ever exposed in debug views

## Quality Gate

- [ ] Debug overlay not present in `--release` builds
- [ ] Mock data generator produces coherent data
- [ ] Event stream shows correct events
- [ ] IPC monitor correctly logs all commands

---

# SECTION 36 — SECURITY AUDIT & COMPLIANCE

## Purpose

Establish a systematic process for ongoing security review of the application. Security is not a one-time checklist — it is a recurring discipline embedded in the development lifecycle.

## Threat Model Summary

(Full threat model maintained separately in `docs/architecture/threat-model.md`)

**Attack surfaces:**
- IPC boundary (frontend → Rust backend)
- Plugin execution layer
- File system (database file, document storage)
- Sync transport
- AI provider communication
- Import pipeline (malicious file upload)

**Assets to protect:**
- Vault contents (passwords, keys, secrets)
- Encryption keys (master key, derived keys)
- Financial transaction data
- Personal contact information
- Authentication credentials

## OWASP Checklist (applied to desktop context)

| Category | Mitigation |
|---|---|
| Injection | All DB queries via SQLx prepared statements. All IPC inputs validated via Zod + Rust type system. |
| Broken Authentication | Session tokens in memory only. Re-auth for sensitive operations. Argon2id hashing. |
| Sensitive Data Exposure | AES-256-GCM encryption at rest. No PII in logs. Keys never persisted in plaintext. |
| XXE / SSRF | XML parsers disabled or sandboxed. No outbound HTTP from app except opt-in AI providers. |
| Broken Access Control | Least-privilege plugin API. No direct DB access from frontend. |
| Security Misconfiguration | Config reviewed pre-release. Tauri capabilities locked to minimum. |
| Vulnerable Components | `cargo audit` + `pnpm audit` in CI. Dependency updates tracked. |
| Logging & Monitoring | Structured logs. Security events logged. Sensitive data never logged. |

## Dependency Management

- `cargo audit` run in CI on every push to main
- `pnpm audit` run in CI on every push to main
- Critical vulnerabilities: block release
- High vulnerabilities: addressed within 7 days
- Dependencies reviewed for supply chain risk before adding

## Secret Scanning

- `gitleaks` scans entire commit history on first setup
- Pre-commit hook: scan staged files for secrets before commit
- Never commit: API keys, private keys, `.env` files with real values

## Penetration Testing Guide

Before each major release, manually test:
1. IPC fuzzing: send malformed payloads to all Tauri commands
2. Plugin sandbox escape: attempt to access vault from a test plugin
3. File path traversal: attempt to access files outside app sandbox via IPC
4. SQL injection: attempt injection via all user-input fields
5. Authentication bypass: attempt to access vault without re-auth
6. Memory inspection: verify keys cleared after use (tooling: valgrind / heaptrack)

## Security Incident Response

1. Identify affected versions
2. Assess data exposure
3. Patch and release immediately
4. Notify users via in-app update notification
5. Document in security changelog

## Security Review Triggers

Security review **required** when:
- New IPC command added
- New dependency added
- Vault or encryption code modified
- Plugin API surface changed
- Sync protocol modified
- Authentication flow changed

## Quality Gate

- [ ] `cargo audit` clean
- [ ] `pnpm audit` clean
- [ ] No secrets in repository
- [ ] All IPC inputs validated
- [ ] Vault re-auth enforced
- [ ] Dependency supply chain reviewed

---

# SECTION 37 — FUTURE ROADMAP & EVOLUTION STRATEGY

## Purpose

Define the intended evolution of Phoenix LifeOS beyond the initial release. This is a directional roadmap — not a commitment. Priorities will shift based on user feedback.

## Release Milestones

### v1.0 — Foundation
Core finance, budget, debt, credit, vault, documents, CRM, Life Hub, AI (local), sync (local backup), reports. Desktop only (Linux, Windows, macOS). Single user.

### v1.5 — Intelligence
Expanded AI capabilities: automated insights, proactive alerts, spending pattern detection. Voice interface. Plugin SDK (beta). Performance polish.

### v2.0 — Connected
Encrypted device sync (peer-to-peer or self-hosted). Mobile companion app (iOS/Android — read-only + quick capture). Bank connection (Open Banking API — opt-in). Financial institution OFX import automation.

### v2.5 — Ecosystem
Plugin marketplace. Third-party plugin developer program. API for external integrations. Community-contributed bank formatters.

### v3.0 — Household
Multi-user / household mode. Shared budgets, shared goals. Per-user permissions. Family dashboard.

## Feature Backlog (Priority-Ordered)

**High priority (near-term):**
- Bill due date tracking and payment reminders
- Recurring transaction detection and management
- Net worth historical tracking with asset appreciation inputs
- Insurance policy tracker (home, auto, life, health)
- Subscription manager (detect, track, cancel suggestions)
- Financial calendar (bills, deadlines, tax dates)

**Medium priority:**
- Open Banking API (Plaid/Finicity — opt-in, US)
- Mobile companion app (React Native or Flutter)
- Estate planning module (beneficiaries, asset inventory, will checklist)
- HSA/FSA account tracking
- 529 college savings planner
- Social Security benefit optimizer
- FIRE calculator (Financial Independence, Retire Early)

**Long-term / research:**
- Self-hosted sync server (open source)
- AI model fine-tuning on anonymized spending patterns (opt-in, private)
- Real estate investment analysis
- Small business payroll (if sufficient demand)
- Desktop widget integration (OS-level)

## Architecture Evolution Principles

**Backwards compatibility:** Existing database migrations are immutable after release. Never modify a released migration — add a new one.

**Plugin stability:** Once Plugin SDK v1 is released, breaking changes to the plugin API require a major version bump with deprecation notices.

**AI model agnosticism:** As new local models improve, the AI layer must swap models without feature breakage. Provider abstraction must be maintained.

**Privacy first, always:** No telemetry, no analytics, no phoning home — regardless of pressure or opportunity. This is non-negotiable.

**User data ownership:** Users must always be able to export all their data in a standard format (JSON/CSV). No vendor lock-in.

## Technology Watch List

Track for potential adoption:
- **SQLite vec extension**: native vector search for embeddings (replace blob workaround)
- **Tauri v3**: monitor for IPC and plugin improvements
- **Leptos / Dioxus**: evaluate Rust-native frontend if React performance becomes a bottleneck
- **llama.cpp Rust bindings**: alternative to Ollama for tighter integration
- **WebAssembly System Interface (WASI)**: improved plugin sandboxing when mature
- **Open Banking standards**: monitor PSD2 (EU), FDX (US), CDR (AU) for integration

## Deprecation Policy

When removing a feature:
1. Mark deprecated in changelog for one minor version
2. Show in-app deprecation notice
3. Remove in next minor version (or major if breaking)

Never silently remove user data capabilities.

---

> "Every feature is useless unless it survives scale, failure, and time."

---

# SECTION 38 — CREDIT REPAIR & LEGAL DISPUTE ENGINE

## Purpose

Empower users to understand their legal rights under the FCRA, FDCPA, and related federal consumer protection laws, generate legally grounded dispute letters, track the dispute lifecycle, and systematically build both personal and business credit. This module is an **education and document-generation tool** — not legal advice. Every output must carry the disclaimer: *"This document is for educational and personal use only. Consult a licensed attorney or credit professional for legal advice."*

---

## Legal Framework Reference

The engine is grounded in four federal statutes:

| Statute | Authority | Key Right |
|---|---|---|
| FCRA § 609 | 15 U.S.C. § 1681g | Right to request your complete credit file and method of verification |
| FCRA § 611 | 15 U.S.C. § 1681i | Right to dispute inaccurate or unverifiable information; bureau must respond within 30 days |
| FCRA § 623 | 15 U.S.C. § 1681s-2 | Furnisher's duty to report accurate information; right to dispute directly with the data furnisher |
| FCRA § 605B | 15 U.S.C. § 1681c-2 | Block of information resulting from identity theft within 4 business days |
| FDCPA § 809 | 15 U.S.C. § 1692g | Debt validation — right to written verification of debt within 30 days of first contact |
| FCRA § 605 | 15 U.S.C. § 1681c | Time limits on negative information (7 years, 10 years for bankruptcy) |

---

## Domain Model

**DisputeCase:**
id, negative_item_id, dispute_type (bureau/furnisher/debt_validation/identity_theft/goodwill), status (draft/letter_generated/sent/awaiting_response/resolved/escalated/closed), statute_basis, created_at, resolved_at

**NegativeItem:**
id, credit_report_id, creditor_name, account_number_masked, item_type (collection/late_payment/charge_off/judgment/bankruptcy/inquiry/identity_theft), balance, date_opened, date_reported, expected_removal_date, is_disputed

**DisputeLetter:**
id, case_id, template_id, generated_content_encrypted, recipient_type (bureau_equifax/bureau_experian/bureau_transunion/furnisher/debt_collector), delivery_method (certified_mail/electronic), tracking_number, sent_at, response_due_at, response_received_at

**DisputeTimeline:**
id, case_id, event_type (letter_sent/response_received/item_deleted/item_updated/escalated/closed), notes, occurred_at

**BusinessCreditProfile:**
id, business_id, ein_encrypted, duns_number, experian_biz_id, equifax_biz_id, paydex_score, intelliscore, business_credit_score, profile_established_at

**VendorCreditAccount:**
id, business_credit_profile_id, vendor_name, account_type (net30/net60/net15/revolving), credit_limit, balance, payment_history, reports_to_bureaus (JSON: ["dun_bradstreet","experian_biz","equifax_biz"]), date_opened

---

## Dispute Letter Templates

All templates are versioned, parameterized, and stored as structured data — never hardcoded strings.

### Template Catalog

| ID | Name | Legal Basis | Recipient |
|---|---|---|---|
| `T01` | § 609 Credit File Request | FCRA § 609 | Bureau |
| `T02` | § 611 Inaccuracy Dispute | FCRA § 611 | Bureau |
| `T03` | § 623 Furnisher Direct Dispute | FCRA § 623 | Original Creditor / Furnisher |
| `T04` | Debt Validation Request | FDCPA § 809 | Debt Collector |
| `T05` | § 605B Identity Theft Block | FCRA § 605B | Bureau |
| `T06` | 7-Year Obsolescence Removal | FCRA § 605 | Bureau |
| `T07` | Goodwill Late Payment Removal | None (goodwill request) | Original Creditor |
| `T08` | Pay-for-Delete Agreement Request | None (negotiation) | Debt Collector |
| `T09` | Cease & Desist (Collector) | FDCPA § 805(c) | Debt Collector |
| `T10` | CFPB Complaint Escalation | CFPB Authority | CFPB (consumer.gov) |
| `T11` | § 623 Business Credit Dispute | FCRA § 623 | Business Furnisher |
| `T12` | Experian Business File Request | FCRA / Business | Experian Business |

### Template Variables

Each template accepts: `user_full_name`, `user_address`, `user_ssn_last4`, `user_dob`, `bureau_name`, `item_description`, `account_number_masked`, `creditor_name`, `dispute_reason`, `supporting_document_list`, `certified_mail_number`, `date_sent`, `response_deadline_date`

All generated letters encrypted before storage. SSN last 4 handled via secure input — never stored plaintext.

### § 611 Dispute Reason Codes

Structured reason codes rather than freeform text (better outcomes):
- `NOT_MINE` — Account does not belong to me
- `WRONG_BALANCE` — Balance reported is incorrect
- `WRONG_STATUS` — Account status is incorrect (e.g., shows open when closed)
- `WRONG_DATE` — Date of first delinquency is incorrect
- `OBSOLETE` — Account is beyond the 7-year reporting period
- `DUPLICATE` — Same account reported multiple times
- `FRAUD` — Account is result of identity theft or fraud
- `PAID_IN_FULL` — Account shows balance but was paid in full
- `NEVER_LATE` — Late payment notation is incorrect
- `INCLUDED_IN_BK` — Debt was discharged in bankruptcy

---

## Dispute Workflow Engine

### Guided Dispute Flow

```
1. Import Credit Report (manual or CSV)
    ↓
2. Negative Item Identification
   (auto-flagged: collections, late payments, charge-offs, judgments)
    ↓
3. Legal Strategy Selector
   (bureau dispute, furnisher dispute, debt validation, or goodwill)
    ↓
4. Template Selector + Reason Code Assignment
    ↓
5. Letter Generation (user reviews, edits, approves)
    ↓
6. Send Instructions + Certified Mail Tracking Input
    ↓
7. 30/45-Day Response Timer Starts
    ↓
8. Bureau Response Recorded
    ↓
9. Outcome: Deleted / Updated / Verified (then escalate or close)
```

### Response Deadline Tracking

| Action | Statutory Deadline |
|---|---|
| Bureau reinvestigation (§ 611) | 30 days (45 days if consumer provides additional info) |
| Furnisher investigation (§ 623) | 30 days |
| FDCPA debt validation response | 30 days from first contact |
| § 605B identity theft block | 4 business days |

System generates automatic reminders at: 7 days before deadline, on deadline day.

### Escalation Paths

1. Bureau fails to respond in 30 days → generate CFPB complaint letter (T10)
2. Item verified but user believes it is inaccurate → generate furnisher direct dispute (T03)
3. Debt collector continues contact after Cease & Desist → generate T09 + flag for attorney referral

---

## Business Credit Building System

### Separation Rules (Non-Negotiable)

- Business credit must be built on EIN, NOT SSN
- Business address must be separate from personal address
- Business bank account must be separate
- Business phone must be separate and listable in 411
- These rules are enforced as a checklist before the user can proceed

### Business Credit Building Roadmap

**Tier 0 — Foundation (Pre-Requisites)**
- EIN obtained (IRS.gov)
- Business entity formed (LLC or Corp — not sole prop)
- Business checking account opened
- Business address established (virtual office acceptable)
- Business phone (not personal cell)
- D-U-N-S number claimed (free at dnb.com)
- Experian Business profile established
- Business listed at 411

**Tier 1 — Starter Trade Lines (Net-30 Vendors)**
Vendors that report to D&B, Experian Business, and/or Equifax Business:
- Uline (office supplies — reports to D&B)
- Quill (office supplies — reports to D&B)
- Grainger (industrial — reports to D&B, Experian Biz)
- Strategic Network Solutions (SNS)
- Crown Office Supplies
- Summa Office Supplies

**Tier 2 — Fleet / Gas Cards**
After 3–5 Tier 1 trade lines with on-time payment history:
- Shell Fleet Card
- Mastercard Fleet (WEX)
- Ramp (corporate card)
- Divvy

**Tier 3 — Business Credit Cards (No Personal Guarantee Goal)**
After 6–12 months of solid Tier 1–2 history:
- American Express Business (OPEN)
- Brex (no PG for qualified businesses)
- Ramp
- Bank of America Business Advantage

**Tier 4 — Bank Loans / LOC**
After 12–18 months of established business credit history.

### PAYDEX Score System

D&B PAYDEX: 0–100 scale based on payment timing.
- 80 = paid on time (goal minimum)
- 100 = paid 30 days early (optimal)

System tracks payment dates vs due dates and projects PAYDEX impact.

---

## AI Integration

- Analyze credit report and identify all disputable items by statute
- Recommend dispute strategy order (highest impact first)
- Explain each negative item in plain English
- Draft custom dispute language for unusual situations
- Monitor for patterns suggesting identity theft
- Business credit: recommend next vendor tier based on current profile strength

**AI rules:** AI suggests strategy and drafts language. User reviews and approves every letter before generation. AI never submits anything autonomously.

---

## Document Vault Integration

All generated letters + bureau responses stored in the encrypted Document Vault (Section 11):
- Letters tagged: `credit-dispute`, `bureau`, `certified-mail`
- Bureau responses tagged: `credit-dispute`, `response`
- Linked to specific `DisputeCase` record
- Full audit trail per case

---

## UI Components

- **Credit Repair Dashboard**: active disputes, upcoming deadlines, deletion wins counter, score trend
- **Negative Items Grid**: all negative items with status badges (active/disputed/deleted)
- **Dispute Wizard**: step-by-step guided flow for each dispute type
- **Letter Preview**: rendered letter with editable fields before approval
- **Timeline Tracker**: per-case event timeline with deadline countdown
- **Business Credit Roadmap**: tier progress tracker, vendor list with reporting status
- **PAYDEX Tracker**: payment history vs PAYDEX projection chart
- **Escalation Center**: cases needing action, CFPB complaint generator

---

## Events

`DisputeCaseCreated`, `DisputeLetterGenerated`, `DisputeLetterSent`, `DisputeResponseReceived`, `NegativeItemDeleted`, `NegativeItemUpdated`, `DisputeEscalated`, `BusinessCreditTierAdvanced`, `VendorAccountAdded`

---

## Implementation Order

1. Domain models + migrations → 2. Template engine (parameterized) → 3. Letter generation for T01–T05 → 4. Dispute workflow UI → 5. Timeline tracker → 6. Response deadline system → 7. Business credit roadmap → 8. Vendor tracking → 9. PAYDEX tracker → 10. AI integration → 11. Remaining templates (T06–T12) → 12. CFPB escalation flow

---

## Quality Gate

- [ ] All 12 letter templates generate correctly with correct statute citations
- [ ] 30/45-day deadlines calculated accurately
- [ ] SSN last 4 never stored in plaintext
- [ ] Generated letters encrypted before vault storage
- [ ] Disclaimer present on every generated document
- [ ] Business credit separation rules enforced (EIN ≠ SSN flow)
- [ ] AI suggestions clearly labeled as suggestions, not legal advice

---

# END OF MASTER SPECIFICATION (Sections 1–38)
