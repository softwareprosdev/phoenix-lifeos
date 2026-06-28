<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=24,12,29&height=260&section=header&text=Phoenix%20LifeOS&fontSize=72&fontColor=ffffff&animation=fadeIn&fontAlignY=42&desc=The%20Complete%20Personal%20Financial%20%26%20Life%20Operating%20System&descColor=cdd6f4&descSize=18&descAlignY=63" width="100%" />

<br/>

[![Typing SVG](https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=600&size=20&duration=3000&pause=1200&color=89B4FA&center=true&vCenter=true&multiline=false&repeat=true&width=750&lines=Built+with+Rust+%2B+Tauri+%2B+React+%E2%9A%A1;Offline-First+%C2%B7+Encrypted+%C2%B7+AI-Powered+%F0%9F%A4%96;Your+Data.+Your+Control.+Forever.+%F0%9F%94%92;37+modules.+One+operating+system+for+your+life.)](https://github.com/softwareprosdev/phoenix-lifeos)

<br/>

<p>
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript" />
  <img src="https://img.shields.io/badge/React_18-20232A?style=for-the-badge&logo=react&logoColor=61DAFB" alt="React" />
  <img src="https://img.shields.io/badge/Tauri_2-24C8D8?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri" />
  <img src="https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white" alt="SQLite" />
  <img src="https://img.shields.io/badge/License-MIT-f5a97f?style=for-the-badge" alt="MIT License" />
</p>

<p>
  <img src="https://img.shields.io/github/actions/workflow/status/softwareprosdev/phoenix-lifeos/ci.yml?branch=main&label=CI%20Build&logo=github&style=flat-square&color=a6e3a1" />
  <img src="https://img.shields.io/github/actions/workflow/status/softwareprosdev/phoenix-lifeos/security.yml?branch=main&label=Security%20Scan&logo=shieldsdotio&style=flat-square&color=89b4fa" />
  <img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-cba6f7?style=flat-square" />
  <img src="https://img.shields.io/badge/status-active%20development-f5c2e7?style=flat-square" />
</p>

</div>

---

## What is Phoenix LifeOS?

**Phoenix LifeOS** is a production-quality, offline-first desktop application for managing your entire financial and personal life. Built with a Rust backend, React frontend, and Tauri for native desktop integration — everything runs locally on your machine, encrypted at rest, with no cloud dependencies.

> *"Build software that another senior engineer would enjoy maintaining."*

---

## ✨ Features

<br/>

<table>
<tr>
<td align="center" valign="top" width="33%">
<br/>
<img src="https://img.shields.io/badge/💰-Finance%20Engine-e94560?style=for-the-badge" />
<br/><br/>
<b>Complete Finance Suite</b>
<br/>
<sub>Budget tracking · Debt payoff strategies · Credit monitoring · Investment portfolio · Net worth calculation · Cash flow analysis</sub>
<br/><br/>
</td>
<td align="center" valign="top" width="33%">
<br/>
<img src="https://img.shields.io/badge/🔐-Zero%20Knowledge%20Vault-1e1e2e?style=for-the-badge" />
<br/><br/>
<b>Privacy-First Architecture</b>
<br/>
<sub>AES-256-GCM encryption · Master key derivation via Argon2id · OS keychain integration · Vault contents never touch a network</sub>
<br/><br/>
</td>
<td align="center" valign="top" width="33%">
<br/>
<img src="https://img.shields.io/badge/🤖-AI%20Advisor-89b4fa?style=for-the-badge" />
<br/><br/>
<b>Local AI Financial Advisor</b>
<br/>
<sub>Ollama (fully local) · OpenAI · Anthropic · Gemini · AI sees only sanitized context — never your keys or raw vault data</sub>
<br/><br/>
</td>
</tr>
<tr>
<td align="center" valign="top" width="33%">
<br/>
<img src="https://img.shields.io/badge/📊-Life%20Hub-a6e3a1?style=for-the-badge" />
<br/><br/>
<b>Life Hub Dashboard</b>
<br/>
<sub>Draggable widget grid · Real-time event stream · Life Score engine · Aggregated view across all modules</sub>
<br/><br/>
</td>
<td align="center" valign="top" width="33%">
<br/>
<img src="https://img.shields.io/badge/🔄-Offline%20First-fab387?style=for-the-badge" />
<br/><br/>
<b>Offline-First Sync</b>
<br/>
<sub>Full functionality with zero internet · E2E encrypted device sync · Conflict resolution · Automatic local backups</sub>
<br/><br/>
</td>
<td align="center" valign="top" width="33%">
<br/>
<img src="https://img.shields.io/badge/🧩-Plugin%20SDK-cba6f7?style=for-the-badge" />
<br/><br/>
<b>Extensible Plugin System</b>
<br/>
<sub>WASM sandbox · Declared permissions · Stable API surface · Vault and keys never accessible to plugins</sub>
<br/><br/>
</td>
</tr>
</table>

<br/>

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                React + TypeScript (Frontend)             │
│          Components · Pages · Hooks · Stores            │
└──────────────────────────┬──────────────────────────────┘
                           │  Tauri IPC (Typed Commands)
┌──────────────────────────▼──────────────────────────────┐
│             Application Service Layer (Rust)             │
│         Business Workflows · Use Cases · Events          │
└──────────────────────────┬──────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────┐
│                  Domain Layer (Rust)                     │
│      Entities · Value Objects · Business Rules           │
└──────────────────────────┬──────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────┐
│               Infrastructure Layer (Rust)                │
│        SQLite · AI Providers · File Storage · Sync       │
└─────────────────────────────────────────────────────────┘
```

---

## 🛠️ Tech Stack

<br/>

<p align="center">
  <img src="https://skillicons.dev/icons?i=rust,ts,react,vite,tailwind,sqlite&theme=dark" />
</p>

<br/>

| Layer | Technology |
|---|---|
| **Desktop Shell** | [Tauri 2](https://tauri.app) — Rust backend + WebView frontend |
| **Backend Language** | [Rust](https://www.rust-lang.org) — async via Tokio, SQLx, Serde, tracing |
| **Frontend** | [React 18](https://react.dev) + [TypeScript 5](https://www.typescriptlang.org) + [Vite 5](https://vitejs.dev) |
| **Styling** | [Tailwind CSS 3](https://tailwindcss.com) + [shadcn/ui](https://ui.shadcn.com) |
| **Animation** | [Framer Motion](https://www.framer.com/motion/) |
| **State** | [TanStack Query v5](https://tanstack.com/query) + [Zustand](https://zustand-demo.pmnd.rs) |
| **Forms** | [React Hook Form](https://react-hook-form.com) + [Zod](https://zod.dev) |
| **Charts** | [Recharts](https://recharts.org) |
| **Database** | SQLite (WAL mode, encrypted at rest) via [SQLx](https://github.com/launchbadge/sqlx) |
| **Encryption** | AES-256-GCM · XChaCha20-Poly1305 · Argon2id |
| **AI** | [Ollama](https://ollama.ai) (local) · OpenAI · Anthropic · Gemini |

---

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/softwareprosdev/phoenix-lifeos.git
cd phoenix-lifeos

# Install JavaScript dependencies
pnpm install

# Run database migrations
make db-migrate

# Start the development server
make dev
```

> See [SETUP.md](./SETUP.md) for full environment setup and prerequisites.

---

## 📁 Project Structure

```
phoenix-lifeos/
├── src/                     # React + TypeScript frontend
│   ├── app/                 # App bootstrap & routing
│   ├── features/            # Feature modules (budget, credit, debt…)
│   ├── components/          # Shared UI components
│   ├── hooks/               # Shared React hooks
│   ├── services/            # IPC service layer
│   └── stores/              # State management
├── src-tauri/               # Tauri backend bridge
│   └── src/                 # Rust entry point + IPC commands
├── crates/                  # Rust workspace crates
│   ├── core/                # Domain entities, value objects, business rules
│   ├── finance/             # Finance service (transactions, accounts)
│   ├── security/            # Encryption, key management, vault
│   ├── db/                  # Database layer, migrations, repositories
│   ├── ai/                  # AI provider abstraction
│   ├── sync/                # Sync engine
│   └── ...                  # 14 total crates
├── migrations/              # SQLite migration files
├── docs/                    # Architecture docs, ADRs, specs
├── .github/workflows/       # CI/CD pipelines
└── Makefile                 # Development scripts
```

---

## 📋 Module Roadmap

<table>
<tr>
<td valign="top" width="50%">

**Core Modules**
- [x] Project scaffold & architecture
- [ ] Database foundation + migrations
- [ ] Authentication & key management
- [ ] Life Hub dashboard
- [ ] Finance Engine (accounts, transactions)
- [ ] Budget Engine
- [ ] Debt Payoff Engine

</td>
<td valign="top" width="50%">

**Advanced Modules**
- [ ] Credit Engine
- [ ] Investment Portfolio
- [ ] Retirement Planner
- [ ] Tax Estimator
- [ ] Document Vault + OCR
- [ ] AI Financial Advisor
- [ ] Plugin SDK

</td>
</tr>
</table>

---

## 🔒 Security

Phoenix LifeOS is designed from the ground up for privacy:

- **No telemetry.** No analytics. No phoning home.
- **No cloud required.** Everything runs locally by default.
- **Encrypted at rest.** AES-256-GCM for all sensitive data.
- **Zero-knowledge keys.** Master key derived from your password — never stored.
- **Plugin sandboxing.** WASM execution, declared permissions, no vault access.

See [DOCUMENTATION.md](./DOCUMENTATION.md#security-architecture) for the full security model.

---

## 📖 Documentation

| Document | Description |
|---|---|
| [DOCUMENTATION.md](./DOCUMENTATION.md) | Full technical documentation, architecture, API reference |
| [SETUP.md](./SETUP.md) | Environment setup, build instructions, development workflow |
| [docs/specs/](./docs/superpowers/specs/) | Complete 37-section engineering specification |

---

## 🤝 Contributing

This project is in active development. Contributions welcome after v1.0 stabilizes.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Follow the [coding standards](./DOCUMENTATION.md#coding-standards) from the spec
4. Ensure all tests pass (`make test`)
5. Submit a pull request

---

## 📄 License

MIT License — see [LICENSE](./LICENSE) for details.

---

<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=24,12,29&height=120&section=footer" width="100%" />

<sub>Built with ❤️ using Rust + Tauri + React</sub>

</div>
