# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-27

### Added
-   **Backend**: Initial Axum-based REST API for Prometheus Service Discovery.
-   **Frontend**: SvelteKit 5 management UI with TailwindCSS.
-   **Docker**: Multi-stage Dockerfile for combined backend and frontend serving.
-   **Helm**: Advanced Helm chart with dynamic ConfigMap/Secret generation and optimized PostgreSQL.
-   **Orchestration**: `entrypoint.sh` for managing dual processes in a single container.
-   **Documentation**: Initial `README.md`, `CONTRIBUTING.md`, `LICENSE`, and `CHANGELOG.md`.

### Changed
-   Frontend now uses `@sveltejs/adapter-node` for production-ready serving in Docker.
-   Refactored Helm chart to use dynamic `.Values.env` and `.Values.secrets` ranges.
