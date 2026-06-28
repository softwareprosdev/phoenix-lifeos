# Phoenix LifeOS — Setup Guide

> Complete environment setup, build instructions, and development workflow.

---

## Prerequisites

Before you begin, install the following tools:

### Required

| Tool | Version | Install |
|---|---|---|
| **Rust** | 1.80+ (stable) | [rustup.rs](https://rustup.rs) |
| **Node.js** | 20+ | [nodejs.org](https://nodejs.org) |
| **pnpm** | 9+ | `npm install -g pnpm` |
| **Tauri CLI** | 2.x | `cargo install tauri-cli` |

### Linux (Fedora / Ubuntu)

```bash
# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file \
  libappindicator-gtk3-devel librsvg2-devel

# Ubuntu / Debian
sudo apt-get install libwebkit2gtk-4.1-dev libssl-dev curl wget file \
  libappindicator3-dev librsvg2-dev patchelf
```

### macOS

```bash
# Xcode Command Line Tools (if not already installed)
xcode-select --install
```

### Windows

- Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (pre-installed on Windows 11)

---

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/softwareprosdev/phoenix-lifeos.git
cd phoenix-lifeos
```

### 2. Install JavaScript dependencies

```bash
pnpm install
```

### 3. Configure environment

```bash
cp .env.example .env.local
# Edit .env.local with your settings
```

### 4. Run database migrations

Migrations run automatically on first launch. To run them manually:

```bash
make db-migrate
```

### 5. Start the development server

```bash
make dev
# or: pnpm tauri dev
```

The Tauri window will open automatically. Hot-reload is active for both the frontend (Vite) and Rust backend (via `cargo watch`).

---

## AI Setup (Optional)

Phoenix LifeOS works fully offline without any AI configuration. To enable AI features:

### Local AI (Recommended — fully private)

```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Pull a recommended model
ollama pull llama3.1:8b        # General purpose (8B parameters)
ollama pull mistral:7b          # Fast financial reasoning
ollama pull phi3:mini           # Lightweight for older hardware
```

The app auto-detects Ollama at `http://localhost:11434`.

### Cloud AI (Optional — data leaves your machine)

Add your API key to `.env.local`:

```bash
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
GEMINI_API_KEY=AIza...
```

Cloud providers are never used by default — must be explicitly selected in Settings.

---

## Development Workflow

### Project commands

```bash
make dev          # Start dev server (Tauri + Vite + HMR)
make build        # Build production release
make test         # Run all tests (Rust + frontend)
make lint         # Run all linters
make fmt          # Auto-format all code
make check        # Full quality gate (lint + typecheck + test)
make db-migrate   # Run pending migrations
make db-reset     # Reset database (DESTRUCTIVE)
make db-seed      # Seed with test data
make clean        # Remove build artifacts
make help         # Show all available commands
```

### Frontend only

```bash
pnpm dev          # Vite dev server (no Tauri)
pnpm build        # Build frontend bundle
pnpm type-check   # TypeScript type checking
pnpm lint         # ESLint
pnpm test         # Vitest unit tests
pnpm test:watch   # Watch mode
```

### Rust only

```bash
cargo build --workspace          # Compile all crates
cargo test --workspace           # All tests
cargo clippy --workspace         # Lint
cargo fmt --all                  # Format
cargo doc --workspace --open     # Generate and open docs
```

---

## Building for Production

```bash
# Build for the current platform
pnpm tauri build

# Output locations:
# Linux:   src-tauri/target/release/bundle/deb/
#          src-tauri/target/release/bundle/rpm/
#          src-tauri/target/release/bundle/appimage/
# Windows: src-tauri/target/release/bundle/msi/
# macOS:   src-tauri/target/release/bundle/dmg/
```

---

## Database

### Location

The database file is stored at the platform-specific application data directory:

| Platform | Path |
|---|---|
| Linux | `~/.local/share/com.phoenixlifeos.app/phoenix.db` |
| Windows | `%APPDATA%\com.phoenixlifeos.app\phoenix.db` |
| macOS | `~/Library/Application Support/com.phoenixlifeos.app/phoenix.db` |

### Migrations

Migrations live in `migrations/` and are embedded into the binary at compile time via SQLx's `sqlx::migrate!()` macro. They run automatically on startup if any pending migrations exist.

To add a new migration:
```bash
# Create a new migration file (increment the number)
touch migrations/0004_my_feature.sql
# Write your SQL, then restart the app
```

---

## Troubleshooting

### `error: failed to run custom build command for tauri-build`

Missing system dependencies. Install the packages listed in the [Prerequisites](#prerequisites) section.

### `WebKitGTK not found` (Linux)

```bash
# Fedora
sudo dnf install webkit2gtk4.1-devel

# Ubuntu
sudo apt-get install libwebkit2gtk-4.1-dev
```

### `pnpm: command not found`

```bash
npm install -g pnpm
```

### App window doesn't open in dev mode

Check that port 5173 is not already in use:
```bash
lsof -i :5173
kill -9 <PID>
```

### Database migration error on startup

Try resetting the database (this deletes all local data):
```bash
make db-reset && make dev
```

### Rust compilation errors after pulling changes

```bash
# Clean build artifacts and retry
make clean && cargo build --workspace
```

---

## IDE Setup

### VS Code (Recommended)

Install the recommended extensions (prompted automatically when opening the project):

```json
// .vscode/extensions.json — already included in the repo
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tauri-apps.tauri-vscode",
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "bradlc.vscode-tailwindcss",
    "usernamehw.errorlens"
  ]
}
```

### RustRover / IntelliJ IDEA

Open the root `Cargo.toml` as the project root. Rust plugin handles the workspace automatically.

---

## Environment Variables Reference

| Variable | Default | Description |
|---|---|---|
| `PHOENIX_DEV_MODE` | `false` | Enable developer debug overlay |
| `RUST_LOG` | `phoenix_lifeos=info` | Log level filter |
| `PHOENIX_DB_PATH` | platform default | Override database file path |
| `OLLAMA_HOST` | `http://localhost:11434` | Ollama server URL |
| `OPENAI_API_KEY` | — | OpenAI API key (optional) |
| `ANTHROPIC_API_KEY` | — | Anthropic API key (optional) |
| `GEMINI_API_KEY` | — | Google Gemini API key (optional) |
| `PHOENIX_SYNC_ENABLED` | `false` | Enable device sync (future) |

---

## Contributing

See the [Contributing section in DOCUMENTATION.md](./DOCUMENTATION.md#contributing) for coding standards, branch workflow, and PR guidelines.
