-- Phoenix LifeOS — Migration 0003: Vault & Documents

-- Vault items (passwords, secrets, notes)
-- NEVER stores plaintext — encrypted_data is always AES-256-GCM ciphertext
CREATE TABLE IF NOT EXISTS vault_items (
    id              TEXT PRIMARY KEY,
    item_type       TEXT NOT NULL CHECK (item_type IN ('password','note','key','secret')),
    title           TEXT NOT NULL,
    encrypted_data  TEXT NOT NULL,  -- base64(nonce || ciphertext)
    key_version     INTEGER NOT NULL DEFAULT 1,
    metadata        TEXT NOT NULL DEFAULT '{}',  -- JSON, non-sensitive fields only
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Vault access audit log
CREATE TABLE IF NOT EXISTS vault_access_log (
    id          TEXT    PRIMARY KEY,
    item_id     TEXT    NOT NULL REFERENCES vault_items(id) ON DELETE CASCADE,
    action      TEXT    NOT NULL CHECK (action IN ('view','create','update','delete')),
    created_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Documents
CREATE TABLE IF NOT EXISTS documents (
    id                  TEXT PRIMARY KEY,
    name                TEXT NOT NULL,
    file_type           TEXT NOT NULL,
    mime_type           TEXT,
    encrypted_path      TEXT NOT NULL,  -- path is itself encrypted
    ocr_text_encrypted  TEXT,           -- OCR result, encrypted
    extracted_metadata  TEXT NOT NULL DEFAULT '{}',  -- JSON
    tags                TEXT NOT NULL DEFAULT '[]',  -- JSON array
    source              TEXT NOT NULL DEFAULT 'upload' CHECK (source IN ('upload','email','scan')),
    created_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
