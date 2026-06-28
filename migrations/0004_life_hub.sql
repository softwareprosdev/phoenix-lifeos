-- Phoenix LifeOS — Migration 0004: Life Hub Extensions

-- Investments
CREATE TABLE IF NOT EXISTS investments (
    id              TEXT PRIMARY KEY,
    account_id      TEXT REFERENCES accounts(id) ON DELETE CASCADE,
    asset_type      TEXT NOT NULL,
    symbol          TEXT,
    quantity        TEXT NOT NULL DEFAULT '0',  -- decimal string
    purchase_price_cents INTEGER NOT NULL DEFAULT 0,
    current_price_cents  INTEGER NOT NULL DEFAULT 0,
    currency        TEXT NOT NULL DEFAULT 'USD',
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Vehicles
CREATE TABLE IF NOT EXISTS vehicles (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    make            TEXT,
    model           TEXT,
    year            INTEGER,
    vin             TEXT,
    license_plate   TEXT,
    purchase_price_cents INTEGER NOT NULL DEFAULT 0,
    currency        TEXT NOT NULL DEFAULT 'USD',
    mileage         INTEGER NOT NULL DEFAULT 0,
    status          TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','sold','scrapped')),
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Vehicle service records
CREATE TABLE IF NOT EXISTS vehicle_service_records (
    id              TEXT PRIMARY KEY,
    vehicle_id      TEXT NOT NULL REFERENCES vehicles(id) ON DELETE CASCADE,
    service_type    TEXT NOT NULL,
    description     TEXT,
    cost_cents      INTEGER NOT NULL DEFAULT 0,
    currency        TEXT NOT NULL DEFAULT 'USD',
    mileage_at_service INTEGER NOT NULL DEFAULT 0,
    service_date    TEXT NOT NULL,
    vendor          TEXT,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Properties
CREATE TABLE IF NOT EXISTS properties (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    address         TEXT,
    property_type   TEXT NOT NULL DEFAULT 'home' CHECK (property_type IN ('home','condo','rental','land','commercial')),
    purchase_price_cents INTEGER NOT NULL DEFAULT 0,
    current_value_cents  INTEGER NOT NULL DEFAULT 0,
    mortgage_balance_cents INTEGER NOT NULL DEFAULT 0,
    currency        TEXT NOT NULL DEFAULT 'USD',
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Home assets / maintenance items
CREATE TABLE IF NOT EXISTS home_assets (
    id              TEXT PRIMARY KEY,
    property_id     TEXT REFERENCES properties(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    category        TEXT NOT NULL,
    purchase_date   TEXT,
    purchase_cost_cents INTEGER NOT NULL DEFAULT 0,
    warranty_end    TEXT,
    lifespan_years  INTEGER,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Contacts (CRM)
CREATE TABLE IF NOT EXISTS contacts (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    role            TEXT,
    organization    TEXT,
    email           TEXT,
    phone           TEXT,
    notes           TEXT,
    tags            TEXT NOT NULL DEFAULT '[]',
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Contact interactions
CREATE TABLE IF NOT EXISTS contact_interactions (
    id              TEXT PRIMARY KEY,
    contact_id      TEXT NOT NULL REFERENCES contacts(id) ON DELETE CASCADE,
    interaction_type TEXT NOT NULL,
    notes           TEXT,
    interaction_date TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Businesses
CREATE TABLE IF NOT EXISTS businesses (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    business_type   TEXT NOT NULL,
    tax_id_encrypted TEXT,
    income_cents    INTEGER NOT NULL DEFAULT 0,
    expense_cents   INTEGER NOT NULL DEFAULT 0,
    currency        TEXT NOT NULL DEFAULT 'USD',
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Business transactions
CREATE TABLE IF NOT EXISTS business_transactions (
    id              TEXT PRIMARY KEY,
    business_id     TEXT NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    amount_cents    INTEGER NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'USD',
    category        TEXT NOT NULL,
    description     TEXT,
    transaction_date TEXT NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- AI conversation logs (no raw prompts stored)
CREATE TABLE IF NOT EXISTS ai_logs (
    id              TEXT PRIMARY KEY,
    provider        TEXT NOT NULL,
    model           TEXT,
    prompt_hash     TEXT NOT NULL,
    tokens_used     INTEGER NOT NULL DEFAULT 0,
    duration_ms     INTEGER NOT NULL DEFAULT 0,
    response_summary TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Maintenance tasks
CREATE TABLE IF NOT EXISTS tasks (
    id              TEXT PRIMARY KEY,
    title           TEXT NOT NULL,
    description     TEXT,
    due_date        TEXT,
    priority        TEXT NOT NULL DEFAULT 'medium' CHECK (priority IN ('low','medium','high','urgent')),
    status          TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','in_progress','completed','cancelled')),
    category        TEXT,
    related_entity_type TEXT,
    related_entity_id   TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_investments_account_id ON investments(account_id);
CREATE INDEX IF NOT EXISTS idx_vehicle_service_vehicle_id ON vehicle_service_records(vehicle_id);
CREATE INDEX IF NOT EXISTS idx_contacts_name ON contacts(name);
CREATE INDEX IF NOT EXISTS idx_contact_interactions_contact_id ON contact_interactions(contact_id);
CREATE INDEX IF NOT EXISTS idx_business_transactions_business_id ON business_transactions(business_id);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date);
