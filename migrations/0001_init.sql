-- Phoenix LifeOS — Migration 0001: Foundation
-- Enables WAL mode and foreign keys (applied via connection options, not SQL).
-- This migration creates the schema versioning and core system tables.

CREATE TABLE IF NOT EXISTS _migrations (
    version     INTEGER PRIMARY KEY,
    name        TEXT    NOT NULL,
    applied_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- System-wide event log (audit trail)
CREATE TABLE IF NOT EXISTS events_log (
    id              TEXT    PRIMARY KEY,
    event_type      TEXT    NOT NULL,
    entity_type     TEXT,
    entity_id       TEXT,
    metadata        TEXT,   -- JSON
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_events_log_type       ON events_log (event_type);
CREATE INDEX IF NOT EXISTS idx_events_log_entity      ON events_log (entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_events_log_created_at  ON events_log (created_at);

-- Application settings (key-value store)
CREATE TABLE IF NOT EXISTS app_settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
