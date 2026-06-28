-- Phoenix LifeOS — Migration 0002: Core Financial Schema

-- Accounts
CREATE TABLE IF NOT EXISTS accounts (
    id              TEXT    PRIMARY KEY,
    name            TEXT    NOT NULL,
    account_type    TEXT    NOT NULL CHECK (account_type IN ('checking','savings','credit','loan','cash','investment','business')),
    institution     TEXT,
    balance_cents   INTEGER NOT NULL DEFAULT 0,
    currency        TEXT    NOT NULL DEFAULT 'USD',
    is_active       INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Transaction categories
CREATE TABLE IF NOT EXISTS categories (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    type        TEXT NOT NULL CHECK (type IN ('income','expense')),
    parent_id   TEXT REFERENCES categories(id) ON DELETE SET NULL,
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Transactions (soft-delete via deleted_at)
CREATE TABLE IF NOT EXISTS transactions (
    id                  TEXT    PRIMARY KEY,
    account_id          TEXT    NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    amount_cents        INTEGER NOT NULL,
    currency            TEXT    NOT NULL DEFAULT 'USD',
    transaction_type    TEXT    NOT NULL CHECK (transaction_type IN ('income','expense','transfer')),
    category_id         TEXT    REFERENCES categories(id) ON DELETE SET NULL,
    merchant            TEXT,
    description         TEXT,
    date                TEXT    NOT NULL,
    tags                TEXT    NOT NULL DEFAULT '[]',  -- JSON array
    is_recurring        INTEGER NOT NULL DEFAULT 0,
    created_at          TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at          TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    deleted_at          TEXT
);

CREATE INDEX IF NOT EXISTS idx_transactions_account_id  ON transactions (account_id);
CREATE INDEX IF NOT EXISTS idx_transactions_date        ON transactions (date);
CREATE INDEX IF NOT EXISTS idx_transactions_category_id ON transactions (category_id);
CREATE INDEX IF NOT EXISTS idx_transactions_deleted     ON transactions (deleted_at) WHERE deleted_at IS NULL;

-- Full-text search on transactions
CREATE VIRTUAL TABLE IF NOT EXISTS transactions_fts USING fts5(
    description, merchant,
    content=transactions,
    content_rowid=rowid
);

-- Budgets
CREATE TABLE IF NOT EXISTS budgets (
    id              TEXT    PRIMARY KEY,
    name            TEXT    NOT NULL,
    period_type     TEXT    NOT NULL CHECK (period_type IN ('weekly','monthly','yearly')),
    total_limit_cents INTEGER NOT NULL,
    currency        TEXT    NOT NULL DEFAULT 'USD',
    start_date      TEXT    NOT NULL,
    end_date        TEXT    NOT NULL,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS budget_items (
    id              TEXT    PRIMARY KEY,
    budget_id       TEXT    NOT NULL REFERENCES budgets(id) ON DELETE CASCADE,
    category_id     TEXT    REFERENCES categories(id) ON DELETE SET NULL,
    limit_cents     INTEGER NOT NULL,
    spent_cents     INTEGER NOT NULL DEFAULT 0
);

-- Goals
CREATE TABLE IF NOT EXISTS goals (
    id                          TEXT    PRIMARY KEY,
    name                        TEXT    NOT NULL,
    goal_type                   TEXT    NOT NULL,
    target_amount_cents         INTEGER NOT NULL,
    current_amount_cents        INTEGER NOT NULL DEFAULT 0,
    monthly_contribution_cents  INTEGER NOT NULL DEFAULT 0,
    currency                    TEXT    NOT NULL DEFAULT 'USD',
    deadline                    TEXT,
    linked_account_id           TEXT    REFERENCES accounts(id) ON DELETE SET NULL,
    created_at                  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at                  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Debts
CREATE TABLE IF NOT EXISTS debts (
    id                      TEXT    PRIMARY KEY,
    name                    TEXT    NOT NULL,
    debt_type               TEXT    NOT NULL,
    principal_cents         INTEGER NOT NULL,
    remaining_balance_cents INTEGER NOT NULL,
    interest_rate_pct       TEXT    NOT NULL, -- stored as decimal string
    minimum_payment_cents   INTEGER NOT NULL,
    currency                TEXT    NOT NULL DEFAULT 'USD',
    due_date                TEXT,
    creditor                TEXT,
    created_at              TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at              TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Credit reports
CREATE TABLE IF NOT EXISTS credit_reports (
    id                  TEXT    PRIMARY KEY,
    score               INTEGER NOT NULL,
    score_model         TEXT    NOT NULL DEFAULT 'FICO_8',
    bureau              TEXT    NOT NULL,
    utilization_rate    TEXT    NOT NULL DEFAULT '0', -- decimal string
    on_time_rate        TEXT    NOT NULL DEFAULT '100',
    derogatory_marks    INTEGER NOT NULL DEFAULT 0,
    total_accounts      INTEGER NOT NULL DEFAULT 0,
    open_accounts       INTEGER NOT NULL DEFAULT 0,
    hard_inquiries_12m  INTEGER NOT NULL DEFAULT 0,
    report_date         TEXT    NOT NULL,
    created_at          TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
