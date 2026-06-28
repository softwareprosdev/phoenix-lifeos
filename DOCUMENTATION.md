# Phoenix LifeOS — Technical Documentation

> Full technical reference: architecture, modules, API, security model, and coding standards.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Architecture](#2-architecture)
3. [Module Reference](#3-module-reference)
4. [Database Schema](#4-database-schema)
5. [IPC Command Reference](#5-ipc-command-reference)
6. [Security Architecture](#6-security-architecture)
7. [AI Integration](#7-ai-integration)
8. [Event System](#8-event-system)
9. [Plugin System](#9-plugin-system)
10. [Coding Standards](#10-coding-standards)
11. [Testing](#11-testing)
12. [Contributing](#12-contributing)

---

## 1. Project Overview

**Phoenix LifeOS** is a desktop application for comprehensive personal finance and life management. It is built to commercial production standards — not a hobby project, not a prototype.

### Design Principles

- **Offline-first.** Full functionality with zero internet connection. Sync is optional and asynchronous.
- **Privacy by default.** No telemetry, no analytics, no cloud dependencies by default.
- **User owns their data.** All data stored locally. Full export always available.
- **Encrypted at rest.** All sensitive fields encrypted with AES-256-GCM before touching disk.
- **AI is advisory only.** AI never mutates financial state directly. It advises; the user decides.

### Technology Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Desktop framework | Tauri 2 | Small binary, native performance, Rust backend |
| Backend language | Rust | Memory safety, performance, strong type system |
| Frontend | React 18 + TypeScript | Mature ecosystem, strict typing, developer tooling |
| Database | SQLite (WAL) | Zero-config, embedded, proven at scale |
| Encryption | AES-256-GCM | Industry standard, authenticated encryption |
| Key derivation | Argon2id | Best-in-class password hashing, memory-hard |
| Local AI | Ollama | Local LLM inference, no data leaves the machine |

---

## 2. Architecture

### Layer Model

```
┌─────────────────────────────────────────────────────────────┐
│                     React UI Layer                          │
│        Components · Pages · Hooks · Feature Stores         │
└──────────────────────────┬──────────────────────────────────┘
                           │  Tauri IPC (typed commands)
┌──────────────────────────▼──────────────────────────────────┐
│              Application Service Layer (Rust)               │
│   Orchestrates workflows · Validates inputs · Emits events  │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                    Domain Layer (Rust)                      │
│       Entities · Value Objects · Business Rules             │
│       No DB · No async · No UI · Pure logic only            │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                 Infrastructure Layer (Rust)                 │
│        SQLite Repos · AI Providers · FS · Sync              │
└─────────────────────────────────────────────────────────────┘
```

**Layer rules (enforced by Rust module visibility):**
- UI never calls the database directly
- Domain layer has no `async`, no DB imports, no UI imports
- Services call repositories through traits — never concrete implementations
- No circular crate dependencies

### Rust Workspace

```
phoenix-lifeos/
├── src-tauri/          → Tauri app + IPC command handlers
└── crates/
    ├── core/           → Domain: entities, value objects, errors
    ├── shared/         → Pagination, common result types
    ├── db/             → SQLite connection, migrations, repository traits
    ├── security/       → AES-256-GCM, Argon2id, key derivation
    ├── settings/       → App configuration model
    ├── finance/        → Accounts, transactions, repositories
    ├── budgeting/      → Budget engine, overspend detection
    ├── debt/           → Debt payoff strategies, amortization
    ├── credit/         → Credit score tracking, utilization analysis
    ├── investments/    → Portfolio tracking, performance calculations
    ├── ai/             → AI provider abstraction, context engine
    ├── vault/          → Encrypted password/secret storage
    ├── crm/            → Contact management
    ├── sync/           → Change event log, device sync
    ├── reports/        → Report generation, data analytics
    └── automation/     → Background job system, workflow engine
```

### Frontend Structure

```
src/
├── app/                → App bootstrap, root providers
├── features/           → Feature modules (self-contained)
│   ├── budget/         → components/ pages/ hooks/ services/ store/ types/
│   ├── finance/
│   ├── debt/
│   ├── credit/
│   ├── investments/
│   ├── goals/
│   ├── vault/
│   ├── crm/
│   ├── maintenance/
│   ├── business/
│   ├── reports/
│   └── settings/
├── components/         → Shared UI components (Button, Card, Modal…)
├── hooks/              → Shared React hooks
├── services/           → IPC service layer (typed wrappers around invoke())
├── stores/             → Global state (Zustand slices)
├── lib/                → Utilities (formatters, cn(), date helpers)
├── types/              → Shared TypeScript types and interfaces
└── styles/             → Global CSS, design tokens
```

---

## 3. Module Reference

### Core Domain (`crates/core`)

The domain layer — contains no I/O, no async, no external dependencies beyond serde and decimal arithmetic.

**Value Objects:**
- `Money` — currency-safe monetary amount (stored as integer cents). Use this everywhere; never use `f64` for money.
- `Currency` — ISO 4217 currency code (validated, 3 uppercase letters).
- `Percentage` — validated 0–100 decimal value with `as_rate()` helper.

**Entities:**
- `Account` — financial account with balance, type, and currency.
- `Transaction` — financial transaction. Always soft-deleted (`deleted_at`), never hard-deleted.
- `Budget` + `BudgetItem` — budget period with per-category spending limits.
- `Debt` — debt with remaining balance, APR, and minimum payment. Includes `monthly_interest_cents()`.
- `Goal` — savings or payoff goal with `progress_percentage()` and `is_achieved()` helpers.

**Business Rules:**
- All money arithmetic goes through `Money::add()` / `Money::subtract()` — enforces currency matching.
- Transactions are soft-deleted only (`deleted_at` timestamp, never `DELETE FROM`).
- Debt balance cannot go below zero — enforced at the service layer.
- Goal progress percentage is clamped to 0–100.

### Security (`crates/security`)

**Key Management:**

```rust
// Derive the master key from the user's password (Argon2id)
let master_key = MasterKey::from_bytes(argon2_hash);

// Derive domain-specific subkeys via HKDF-SHA256
let vault_key = master_key.vault_key();     // "phoenix:vault:v1"
let doc_key   = master_key.document_key(); // "phoenix:documents:v1"
let sync_key  = master_key.sync_key();     // "phoenix:sync:v1"
```

**Encryption:**

```rust
use phoenix_security::{encrypt, decrypt};

// Encrypt vault item content
let ciphertext = encrypt(&vault_key, plaintext.as_bytes())?;
// → base64(nonce[12] || ciphertext + auth_tag[16])

// Decrypt
let plaintext = decrypt(&vault_key, &ciphertext)?;
```

`MasterKey` implements `Zeroize + ZeroizeOnDrop` — key material is cleared from memory when the struct is dropped.

### Database (`crates/db`)

**Connection:**

```rust
use phoenix_db::{connection, DbPool};

// Open or create the SQLite database
let pool: DbPool = connection::open("/path/to/phoenix.db").await?;

// Run pending migrations
connection::run_migrations(&pool).await?;
```

**Repository pattern:** All data access goes through repository traits defined in feature crates. No raw SQL in service code. Services receive `Arc<dyn SomeRepository>` — testable with in-memory implementations.

---

## 4. Database Schema

### Design Rules

- Every table has: `id TEXT PRIMARY KEY` (UUID), `created_at`, `updated_at`
- Transactions are soft-deleted via `deleted_at` — never `DELETE`
- Amounts stored as `INTEGER` (cents) — never `REAL`/`FLOAT`
- Rates and decimals stored as `TEXT` (decimal string) — never `REAL`
- Foreign keys always enabled
- WAL mode always active

### Tables

| Table | Description | Migration |
|---|---|---|
| `events_log` | System-wide audit trail | 0001 |
| `app_settings` | Key-value configuration | 0001 |
| `accounts` | Financial accounts | 0002 |
| `categories` | Transaction categories | 0002 |
| `transactions` | Financial transactions (soft-delete) | 0002 |
| `transactions_fts` | FTS5 full-text search index | 0002 |
| `budgets` | Budget periods | 0002 |
| `budget_items` | Per-category budget allocations | 0002 |
| `goals` | Savings and payoff goals | 0002 |
| `debts` | Debt records | 0002 |
| `credit_reports` | Credit score snapshots | 0002 |
| `vault_items` | Encrypted vault entries | 0003 |
| `vault_access_log` | Vault access audit log | 0003 |
| `documents` | Encrypted document metadata | 0003 |

### Adding a Migration

1. Create `migrations/000N_description.sql` (increment `N`)
2. Write idempotent SQL using `CREATE TABLE IF NOT EXISTS`
3. Restart the app — migrations run automatically at startup
4. **Never modify a migration after it has been applied in production**

---

## 5. IPC Command Reference

All frontend → backend communication goes through Tauri's typed IPC layer. The frontend never touches the database directly.

### Conventions

- Command names: `snake_case`
- Payloads: strongly typed Rust structs with `#[derive(Serialize, Deserialize)]`
- Errors: returned as `Err(String)` from Tauri commands; mapped to typed errors on the frontend
- All inputs validated in Rust before processing

### Invoking from Frontend

```typescript
import { invoke } from "@tauri-apps/api/core";

// Example: fetch all active accounts
const accounts = await invoke<Account[]>("list_accounts");

// Example: create a transaction
const tx = await invoke<Transaction>("create_transaction", {
  payload: {
    accountId: "...",
    amountCents: 4200,
    currency: "USD",
    transactionType: "expense",
    description: "Coffee",
    date: new Date().toISOString(),
  }
});
```

### Command Listing (grows as modules are built)

| Command | Description |
|---|---|
| `greet` | Health check (placeholder, remove after first real command) |

> Commands are added per-module during the build phase. This table is updated as each module is implemented.

---

## 6. Security Architecture

### Threat Model

The application assumes:
- The database file on disk may be accessed by an attacker
- Memory may be inspected (e.g., via crash dumps)
- Plugins may be malicious
- Sync endpoints may be compromised

### Key Architecture

```
User Password
    ↓ Argon2id (memory-hard KDF)
Master Key (32 bytes, in-memory only)
    ↓ HKDF-SHA256 per domain
    ├── vault_key       → vault items
    ├── document_key    → documents & OCR text
    ├── sync_key        → sync payloads
    └── backup_key      → encrypted backups
```

**Rules:**
- Master key never persisted to disk
- Derived keys never persisted to disk
- Keys exist in memory only during active use
- `MasterKey` is `ZeroizeOnDrop` — cleared when dropped
- Session locks clear all keys from memory

### Vault Security

Each vault item is independently encrypted:

```
vault_items.encrypted_data = base64(nonce[12] || AES-256-GCM(plaintext, vault_key))
```

The vault key is derived from the master key. A compromised database file without the master key cannot decrypt vault items.

### IPC Security Rules

1. All IPC inputs validated in Rust using domain types
2. Frontend DTOs never contain raw encryption keys
3. Vault operations require re-authentication (future: session re-confirm)
4. Destructive operations (export all, delete all) require explicit confirmation

### What Plugins Cannot Access

- `MasterKey` or any derived key
- Raw vault item contents (only metadata via declared `read_vault_metadata` permission)
- Other plugins' storage namespace
- Direct database access

---

## 7. AI Integration

### Provider Architecture

```rust
// All AI calls go through this trait — never directly to a provider
#[async_trait]
trait AiProvider: Send + Sync {
    async fn chat(&self, messages: Vec<Message>, options: ChatOptions) -> Result<String>;
    async fn stream(&self, messages: Vec<Message>, options: ChatOptions) -> impl Stream<Item = String>;
}
```

Supported providers: `OllamaProvider`, `OpenAiProvider`, `AnthropicProvider`, `GeminiProvider`.

The app never depends on a specific provider — all feature code calls `AiService::chat()`.

### Context Sanitization Pipeline

Before any data is sent to an AI provider:

1. Remove all vault contents and encryption keys
2. Remove exact account numbers and SSNs
3. Summarize transaction details (amounts → ranges, merchants → categories)
4. Limit to requested scope (e.g., last 90 days of transactions)
5. Enforce maximum token budget

**AI is never given:** raw passwords, vault contents, encryption keys, or unsanitized PII.

### Tool-Using System

AI operates through a tool system — it never directly reads the database:

```
AI request → AIService → sanitize context → call AI with tools
AI response → parse tool calls → route to domain services → return results to AI
AI final response → display to user
```

---

## 8. Event System

### Domain Events

Events are published by services and consumed by subscribers. No direct cross-service calls.

| Event | Publisher | Subscribers |
|---|---|---|
| `TransactionCreated` | FinanceService | BudgetService, GoalService, ReportService |
| `BudgetExceeded` | BudgetService | NotificationService, AIService |
| `DebtPaymentMade` | DebtService | GoalService, ReportService |
| `CreditScoreUpdated` | CreditService | AIService, ReportService |
| `GoalAchieved` | GoalService | NotificationService |
| `DocumentAdded` | VaultService | OCRService, AIService |
| `VaultAccessed` | VaultService | AuditLogService |

### Publishing Events

```rust
// In a service:
event_bus.publish(DomainEvent::TransactionCreated {
    transaction_id: tx.id,
    account_id: tx.account_id,
    amount_cents: tx.amount_cents,
    timestamp: Utc::now(),
}).await;
```

---

## 9. Plugin System

### Plugin Structure

```
my-plugin.plx (zip)
├── manifest.json       → ID, name, version, permissions
├── plugin.wasm         → WASM module (sandboxed via wasmtime)
├── ui/                 → Optional React components
└── assets/             → Icons, resources
```

### Available Permissions

| Permission | Grants |
|---|---|
| `read_finance` | Read accounts, transactions, balances |
| `create_transaction` | Create transactions (user confirmation required) |
| `read_goals` | Read goal progress |
| `add_widget` | Register a Life Hub dashboard widget |
| `schedule_job` | Register background jobs |
| `write_settings` | Read/write plugin-specific settings namespace |

Vault, encryption keys, documents, and raw passwords are inaccessible to plugins under any permission.

---

## 10. Coding Standards

### Rust

- Idiomatic Rust — no unnecessary clones, proper ownership
- All errors via `thiserror` — no `unwrap()` in production code paths
- No raw SQL in service code — all queries through repository traits
- `Money` for all financial values — never `f64`
- `async fn` everywhere I/O is involved — no blocking calls in async context
- `tracing::instrument` on all public service methods
- Tests in `#[cfg(test)]` inline modules for unit tests, `tests/` for integration

### TypeScript / React

- Strict TypeScript — no `any`, no type assertions unless unavoidable
- Functional components only — no class components
- Custom hooks for all stateful logic — no business logic in components
- `zod` schemas at all IPC boundaries — validate before using
- `TanStack Query` for all server state — no manual `useEffect` for fetching
- Feature-first organization — all code for a feature lives in `src/features/<name>/`
- Accessibility: `aria-label` on all icon-only buttons, keyboard-navigable

### Git

- `feat:`, `fix:`, `chore:`, `docs:` commit prefixes
- One logical change per commit
- No commits that break the build
- PR description includes: what changed, why, how to test

---

## 11. Testing

### Rust Tests

**Unit tests** (in-module `#[cfg(test)]` blocks):
- All value object arithmetic and validation
- All business rule calculations
- Pure functions — no I/O

**Integration tests** (`crates/*/tests/`):
- Repository operations against real in-memory SQLite
- Service workflows end-to-end
- Use `DbPool::connect(":memory:")` — no test file cleanup needed

### Frontend Tests (Vitest)

**Unit tests:**
- Utility functions, formatters
- Custom hooks via `renderHook`

**Component tests:**
- Render + user interaction
- Mock the IPC service layer — never call real Rust from frontend tests
- Use `userEvent` for interactions (not `fireEvent`)

### Running Tests

```bash
make test              # All tests
make test-rust         # Rust only
make test-frontend     # Frontend only
cargo test -p phoenix-core     # Single crate
```

---

## 12. Contributing

### Branch Workflow

```
main          → stable, protected
develop       → integration branch
feature/*     → new features
fix/*         → bug fixes
chore/*       → tooling, deps, refactors
```

### Pull Request Checklist

- [ ] `make check` passes (lint + typecheck + test)
- [ ] New features have tests
- [ ] Security-sensitive changes reviewed against the threat model
- [ ] No secrets or PII added to the codebase
- [ ] DOCUMENTATION.md updated if public API changed

### Module Development Workflow

Each new feature module follows this order:

1. Domain entities + value objects in `crates/core`
2. Repository trait definition
3. SQLite migration
4. Repository implementation in `crates/db`
5. Service implementation
6. Tauri IPC command(s) in `src-tauri/src/commands/`
7. TypeScript IPC service wrapper in `src/services/`
8. Feature UI in `src/features/<name>/`
9. Tests at each layer
10. Documentation update

---

*For setup and installation, see [SETUP.md](./SETUP.md).*
*For the full 37-section engineering specification, see [docs/superpowers/specs/](./docs/superpowers/specs/).*
