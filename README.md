<div align="center">
  <img src="logo.svg" width="90" height="90" alt="Narthex Logo">
  <h1>Narthex</h1>
  <p><strong>Modern Prometheus HTTP Service Discovery Manager</strong></p>

  <a href="https://github.com/muhfalihr/narthex/releases"><img src="https://img.shields.io/badge/version-0.1.1-f97316?style=flat-square" alt="Version"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License"></a>
  <img src="https://img.shields.io/badge/rust-1.85+-orange?style=flat-square&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/SvelteKit-5-FF3E00?style=flat-square&logo=svelte&logoColor=white" alt="SvelteKit">
  <img src="https://img.shields.io/badge/PostgreSQL-supported-4169E1?style=flat-square&logo=postgresql&logoColor=white" alt="PostgreSQL">
  <img src="https://img.shields.io/badge/docker-ready-2496ED?style=flat-square&logo=docker&logoColor=white" alt="Docker">
</div>

---

**Narthex** is a dynamic manager for [Prometheus HTTP Service Discovery](https://prometheus.io/docs/prometheus/latest/http_sd/). It provides a Web UI and REST API to manage your monitoring targets, labels, and groups — no static file editing required.

> A *narthex* is the entrance hall of a church: the gateway before the main space. Narthex is your gateway into Prometheus monitoring.

---

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Local Development](#local-development)
  - [Docker Deployment](#docker-deployment)
  - [Kubernetes / Helm](#kubernetes--helm)
- [API Reference](#api-reference)
- [Security & Authentication](#security--authentication)
- [Configuration](#configuration)
- [Usage with Prometheus](#usage-with-prometheus)
- [License](#license)

---

## Features

- **Full REST API** — Programmatically manage groups, targets, and labels. Swagger UI included.
- **Modern Web UI** — SvelteKit 5 + TailwindCSS dashboard, fast and responsive.
- **Optional Authentication** — HTTP Basic Auth on the API; session-cookie login for the UI.
- **Dual-Process Container** — Backend (Rust/Axum) and Frontend (SvelteKit) served from a single Docker image.
- **Helm Chart** — Kubernetes-ready with dynamic env/secret injection and bundled PostgreSQL.
- **PostgreSQL Storage** — Reliable, persistent data with `sqlx` and automated migrations.

---

## Architecture

| Component     | Technology                                  | Role                                                         |
|:------------- |:------------------------------------------- |:------------------------------------------------------------ |
| **Backend**   | [Rust](https://www.rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum) | High-performance REST API, auth middleware, DB access |
| **Frontend**  | [SvelteKit 5](https://svelte.dev/) + TailwindCSS | SSR-enabled management UI                             |
| **Database**  | PostgreSQL + sqlx                           | Persistent storage with compile-time-checked queries         |
| **Container** | Multi-stage Docker                          | Minimal `debian:bookworm-slim` image with both processes     |

```
┌─────────────────────────────────────────┐
│              Docker Container            │
│                                         │
│  ┌─────────────┐    ┌─────────────────┐ │
│  │  SvelteKit  │───▶│   Axum Backend  │ │
│  │  :8080      │    │   :3000         │ │
│  └─────────────┘    └────────┬────────┘ │
│                              │          │
│                    ┌─────────▼────────┐ │
│                    │    PostgreSQL     │ │
│                    └──────────────────┘ │
└─────────────────────────────────────────┘
         ▲                    ▲
   Browser / UI         Prometheus
```

---

## Getting Started

### Prerequisites

For local development, install:

- [Rust](https://rustup.rs/) 1.85+
- [Node.js](https://nodejs.org/) 22+
- PostgreSQL

### Local Development

```bash
# 1. Clone
git clone https://github.com/muhfalihr/narthex.git
cd narthex

# 2. Configure environment
cp .env.example .env
# Edit .env with your database credentials

# 3. Run the backend
cargo run

# 4. Run the frontend (separate terminal)
cd ui && npm install && npm run dev
```

Backend is available at `http://localhost:3000`, frontend at `http://localhost:5173`.

### Docker Deployment

The Docker image bundles both processes. A single `docker run` is all you need.

```bash
# Build
docker build -t narthex:latest .

# Run
docker run -d \
  -p 8080:8080 \
  -p 3000:3000 \
  -e DB_HOST=host.docker.internal \
  -e DB_PORT=5432 \
  -e DB_USER=postgres \
  -e DB_PASSWORD=yourpassword \
  -e DB_NAME=narthex \
  -e APP_USERNAME=admin \
  -e APP_PASSWORD=securepassword \
  narthex:latest
```

| Service  | URL                               |
|:-------- |:--------------------------------- |
| Web UI   | `http://localhost:8080`           |
| REST API | `http://localhost:3000/api/v1`    |
| Swagger  | `http://localhost:3000/swagger-ui`|

### Kubernetes / Helm

```bash
helm install narthex ./charts/narthex \
  --set secrets.DB_USER=postgres \
  --set secrets.DB_PASSWORD=yourpassword \
  --set secrets.DB_NAME=narthex \
  --set env.APP_USERNAME=admin \
  --set secrets.APP_PASSWORD=securepassword
```

See [`charts/narthex/values.yaml`](charts/narthex/values.yaml) for all available values.

---

## API Reference

All endpoints are prefixed with `/api/v1`. Authentication uses HTTP Basic Auth when enabled.

| Method   | Endpoint                                           | Description                          |
|:-------- |:-------------------------------------------------- |:------------------------------------ |
| `GET`    | `/targets`                                         | Prometheus SD endpoint               |
| `GET`    | `/groups`                                          | List all target groups               |
| `POST`   | `/groups`                                          | Create a target group                |
| `PUT`    | `/groups/:id`                                      | Update a target group                |
| `DELETE` | `/groups/:id`                                      | Delete a target group                |
| `GET`    | `/groups/:id/targets`                              | List targets in a group              |
| `POST`   | `/groups/:id/targets`                              | Add a target to a group              |
| `PUT`    | `/groups/:id/targets/:tid`                         | Update a target                      |
| `DELETE` | `/groups/:id/targets/:tid`                         | Remove a target                      |
| `GET`    | `/groups/:id/labels`                               | List labels on a group               |
| `POST`   | `/groups/:id/labels`                               | Add a label to a group               |
| `DELETE` | `/groups/:id/labels/:lid`                          | Remove a label                       |

Interactive API explorer: `http://localhost:3000/swagger-ui`

---

## Security & Authentication

Authentication is **optional**. Set both `APP_USERNAME` and `APP_PASSWORD` to enable it; omit either to run in open access mode.

| Client          | Auth Method                                        |
|:--------------- |:-------------------------------------------------- |
| **Web UI**      | Login page → session cookie                        |
| **API / curl**  | HTTP Basic Auth (`-u username:password`)           |
| **Prometheus**  | `basic_auth` block in `http_sd_configs`            |

---

## Configuration

| Variable       | Description                      | Default     |
|:-------------- |:-------------------------------- |:----------- |
| `DB_HOST`      | PostgreSQL hostname              | `localhost` |
| `DB_PORT`      | PostgreSQL port                  | `5432`      |
| `DB_USER`      | Database user                    | —           |
| `DB_PASSWORD`  | Database password                | —           |
| `DB_NAME`      | Database name                    | —           |
| `APP_HOST`     | Backend bind address             | `0.0.0.0`   |
| `APP_PORT`     | Backend port                     | `3000`      |
| `APP_USERNAME` | Admin username (enables auth)    | —           |
| `APP_PASSWORD` | Admin password (enables auth)    | —           |
| `PORT`         | Frontend Node.js port            | `8080`      |

---

## Usage with Prometheus

Add Narthex as an HTTP SD source in your `prometheus.yml`:

```yaml
# Without authentication
scrape_configs:
  - job_name: 'my_services'
    http_sd_configs:
      - url: 'http://<narthex-host>:3000/api/v1/targets'
        refresh_interval: 30s
```

```yaml
# With authentication
scrape_configs:
  - job_name: 'my_services'
    http_sd_configs:
      - url: 'http://<narthex-host>:3000/api/v1/targets'
        refresh_interval: 30s
        basic_auth:
          username: 'your_username'
          password: 'your_password'
```

Prometheus will poll `/api/v1/targets` on the configured interval and automatically update its scrape targets.

---

## License

MIT — see [LICENSE](LICENSE) for details.
