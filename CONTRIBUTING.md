# Contributing to Narthex

Thank you for your interest in contributing! This guide covers everything you need to get started.

## Development Setup

### Backend (Rust)

```bash
# Run the server
cargo run

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Run tests
cargo test
```

### Frontend (SvelteKit)

```bash
cd ui

# Install dependencies
npm install

# Start dev server
npm run dev

# Format
npm run format

# Lint + type check
npm run lint && npm run check

# Production build
npm run build
```

## Project Structure

```
narthex/
├── src/                  # Rust backend
│   ├── main.rs           # Entry point, router setup
│   ├── config.rs         # Env-based configuration
│   ├── auth.rs           # HTTP Basic Auth middleware
│   ├── discovery/        # GET /api/v1/targets (Prometheus SD)
│   ├── groups/           # CRUD for target groups
│   ├── targets/          # CRUD for targets within groups
│   └── labels/           # CRUD for labels on groups
├── ui/                   # SvelteKit frontend
├── migrations/           # sqlx SQL migrations
├── charts/               # Helm chart
└── Dockerfile
```

## Branching Strategy

| Branch pattern          | Purpose              |
|:----------------------- |:-------------------- |
| `main`                  | Stable, always deployable |
| `feature/<name>`        | New features          |
| `fix/<name>`            | Bug fixes             |
| `docs/<name>`           | Documentation changes |

Always branch from `main`.

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

| Prefix       | When to use                                      |
|:------------ |:------------------------------------------------ |
| `feat:`      | New feature                                      |
| `fix:`       | Bug fix                                          |
| `docs:`      | Documentation only                               |
| `refactor:`  | Code change, no behavior change                  |
| `style:`     | Formatting, whitespace                           |
| `test:`      | Adding or fixing tests                           |
| `chore:`     | Build, CI, dependencies                          |

## Pull Request Process

1. Fork the repo and create your branch from `main`.
2. Add tests for any new logic.
3. Ensure `cargo test`, `cargo clippy`, and the frontend checks all pass.
4. Update `CHANGELOG.md` under `[Unreleased]`.
5. Open a PR with a clear description of what and why.

## Reporting Issues

Open a [GitHub Issue](https://github.com/muhfalihr/narthex/issues) and include:

- What you expected vs. what happened
- Steps to reproduce
- Narthex version, OS, and relevant config (redact credentials)

## Code Style

- **Rust**: follow `rustfmt` defaults; use `clippy` suggestions; no `unwrap()` in production paths.
- **Svelte/TS**: follow Prettier defaults; no `any` types without a comment explaining why.
- **SQL**: snake_case column names; explicit column lists in `SELECT`.
