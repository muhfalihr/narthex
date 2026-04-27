# Contributing to Narthex

First off, thank you for considering contributing to Narthex! It's people like you who make it such a great tool.

## 🛠️ Development Environment

### Backend (Rust)
- Use `cargo fmt` for formatting.
- Ensure all tests pass: `cargo test`.
- Run `cargo clippy` to check for common mistakes and idiomatic improvements.

### Frontend (SvelteKit)
- Use `npm run format` (Prettier) before committing.
- Check for lint errors: `npm run lint`.
- Type checking: `npm run check`.

## 🌿 Branching Strategy

- `main` is the stable branch.
- Create feature branches from `main`: `feature/my-cool-feature`.
- Fixes should go into `fix/my-bug-fix`.

## 📝 Pull Request Process

1.  Fork the repository and create your branch from `main`.
2.  If you've added code that should be tested, add tests.
3.  Ensure the build passes (Rust and Node).
4.  Update the `CHANGELOG.md` with your changes under the `[Unreleased]` section.
5.  Submit a Pull Request with a clear description of what the change does and why it's needed.

## 💬 Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat: ...` for new features.
- `fix: ...` for bug fixes.
- `docs: ...` for documentation changes.
- `style: ...` for formatting, missing semi-colons, etc.
- `refactor: ...` for code changes that neither fix a bug nor add a feature.

Thank you for your contributions!
