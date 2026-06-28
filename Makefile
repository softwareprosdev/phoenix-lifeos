.PHONY: dev build test lint fmt check clean db-migrate db-reset db-seed install

# ─── Development ─────────────────────────────────────────────────────────────

dev: ## Start development server (frontend + Tauri)
	pnpm tauri dev

build: ## Build production release for all platforms
	pnpm tauri build

install: ## Install all dependencies
	pnpm install

# ─── Testing ─────────────────────────────────────────────────────────────────

test: ## Run all tests (Rust + frontend)
	cargo test --workspace && pnpm test

test-rust: ## Run Rust tests only
	cargo test --workspace

test-frontend: ## Run frontend tests only
	pnpm test

test-e2e: ## Run end-to-end Playwright tests
	pnpm playwright test

# ─── Code Quality ────────────────────────────────────────────────────────────

lint: ## Run all linters
	cargo clippy --workspace -- -D warnings && pnpm lint

fmt: ## Format all code
	cargo fmt --all && pnpm prettier --write "src/**/*.{ts,tsx}"

fmt-check: ## Check formatting without modifying
	cargo fmt --all --check && pnpm prettier --check "src/**/*.{ts,tsx}"

check: ## Run full quality gate (lint + type-check + test)
	cargo fmt --all --check
	cargo clippy --workspace -- -D warnings
	pnpm type-check
	pnpm lint
	cargo test --workspace
	pnpm test

# ─── Database ────────────────────────────────────────────────────────────────

db-migrate: ## Run all pending database migrations
	cargo run --package phoenix-db --bin migrate

db-reset: ## Reset database to empty state (DESTRUCTIVE)
	@echo "WARNING: This will delete all data. Press Ctrl+C to cancel."
	@sleep 3
	cargo run --package phoenix-db --bin reset

db-seed: ## Seed database with test data
	cargo run --package phoenix-db --bin seed

# ─── Cleaning ────────────────────────────────────────────────────────────────

clean: ## Remove build artifacts
	cargo clean
	rm -rf dist/ node_modules/.vite

# ─── Help ────────────────────────────────────────────────────────────────────

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
