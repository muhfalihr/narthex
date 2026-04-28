# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-04-28

### Added
- **Authentication System**:
  - **Backend**: Implemented HTTP Basic Auth middleware in Rust/Axum, activated via environment variables.
  - **Frontend**: Added a secure Login page with credential validation.
  - **Session Management**: Implemented cookie-based session tracking for the Web UI.
  - **API Security**: Automated injection of authentication headers for proxied API requests.
  - **Logout**: Added a logout mechanism and UI button (conditionally displayed based on auth status).
- **Project Structure**: Created symbolic link for `.env` files to ensure configuration consistency between root and UI directories.

### Changed
- **UI/UX**: Refactored the main layout to support full-page login views and conditional sidebar elements.
- **Documentation**: 
  - Substantially updated `README.md` with security instructions and correct Prometheus configuration examples.
  - Fixed Prometheus discovery URL in documentation (points to `/api/v1/targets`).
- **Configuration**: Added `APP_USERNAME` and `APP_PASSWORD` to `.env.example`.

### Fixed
- Fixed an issue where unauthorized API requests from server-side `load` functions would fail with a 401 error instead of redirecting to the login page.

## [0.1.0] - 2026-04-27

### Added
- **Backend**: Initial Axum-based REST API for Prometheus Service Discovery.
- **Frontend**: SvelteKit 5 management UI with TailwindCSS.
- **Docker**: Multi-stage Dockerfile for combined backend and frontend serving.
- **Helm**: Advanced Helm chart with dynamic ConfigMap/Secret generation and optimized PostgreSQL.
- **Orchestration**: `entrypoint.sh` for managing dual processes in a single container.
- **Documentation**: Initial `README.md`, `CONTRIBUTING.md`, `LICENSE`, and `CHANGELOG.md`.

### Changed
- Frontend now uses `@sveltejs/adapter-node` for production-ready serving in Docker.
- Refactored Helm chart to use dynamic `.Values.env` and `.Values.secrets` ranges.
