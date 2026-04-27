# Narthex

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg)

**Narthex** is a robust, modern manager for Prometheus HTTP Service Discovery (HTTP SD). It provides a sleek Web UI and a powerful REST API, allowing you to dynamically manage your monitoring targets, labels, and groups without manually editing static files.

---

## 📑 Table of Contents
- [Features](#-features)
- [Architecture](#-architecture)
- [Getting Started](#️-getting-started)
  - [Prerequisites](#prerequisites)
  - [Local Development](#local-development)
  - [Docker Deployment](#docker-deployment)
  - [Kubernetes / Helm](#kubernetes--helm)
- [Configuration](#⚙️-configuration)
- [Usage with Prometheus](#📡-usage-with-prometheus)
- [API Documentation](#-api-documentation)
- [License](#⚖️-license)

---

## 🚀 Features

- **Dual-Process Container**: Backend (Rust/Axum) and Frontend (SvelteKit) are efficiently bundled in a single Docker image.
- **RESTful API**: A full-featured API for programmatically managing Prometheus target groups.
- **Modern Web UI**: Built with SvelteKit 5 and TailwindCSS for a fast, responsive, and intuitive management experience.
- **Dynamic Kubernetes Deployment**: Flexible Helm chart supporting dynamic environment variables and an optimized PostgreSQL/PostGIS setup.
- **Reliable Storage**: Persistent and scalable data storage using PostgreSQL and `sqlx`.

## 🏗️ Architecture

| Component | Technology | Description |
| :--- | :--- | :--- |
| **Backend** | [Rust](https://www.rust-lang.org/) & [Axum](https://github.com/tokio-rs/axum) | High-performance, safe, and highly concurrent REST API. |
| **Frontend** | [SvelteKit 5](https://svelte.dev/) | SSR-enabled modern UI using the Node.js adapter. |
| **Database** | PostgreSQL | Relational database (optionally with PostGIS via Helm). |
| **Container**| Multi-stage Docker | Minimal image size using `debian:bookworm-slim`. |

---

## 🛠️ Getting Started

### Prerequisites

Ensure you have the following installed if developing locally:
- Rust (1.85+)
- Node.js (22+)
- PostgreSQL

### Local Development

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/narthex.git
   cd narthex
   ```

2. **Configure Environment**:
   Copy the example environment file and fill in your database credentials.
   ```bash
   cp .env.example .env
   ```

3. **Run the Backend**:
   ```bash
   cargo run
   ```

4. **Run the Frontend**:
   ```bash
   cd ui
   npm install
   npm run dev
   ```

### Docker Deployment

Build and run the combined application locally. Both the frontend and backend are served from the same container.

1. **Build the image**:
   ```bash
   docker build -t docker.io/muhfalihr/narthex:latest .
   ```

2. **Run the container**:
   ```bash
   docker run -d \
     -p 8080:8080 -p 3000:3000 \
     -e DB_HOST=host.docker.internal \
     -e DB_PORT=5432 \
     -e DB_USER=your_user \
     -e DB_PASSWORD=your_pass \
     -e DB_NAME=your_db \
     docker.io/muhfalihr/narthex:latest
   ```
   *Frontend will be available at `http://localhost:8080` and the API at `http://localhost:3000/api/v1`.*

### Kubernetes / Helm

Deploy Narthex and a bundled, optimized PostgreSQL instance using the included Helm chart.

```bash
helm install my-release ./charts/narthex
```

---

## ⚙️ Configuration

The application requires the following environment variables to connect to the database:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `DB_HOST` | PostgreSQL server hostname | `localhost` |
| `DB_PORT` | PostgreSQL server port | `5432` |
| `DB_USER` | Database user | - |
| `DB_PASSWORD` | Database password | - |
| `DB_NAME` | Database name | - |
| `APP_HOST` | Backend bind address | `0.0.0.0` |
| `APP_PORT` | Backend port | `3000` |
| `PORT` | Frontend Node.js port | `8080` |

---

## 📡 Usage with Prometheus

To instruct Prometheus to discover targets managed by Narthex, add the following to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'narthex_discovery'
    http_sd_configs:
      - url: 'http://<narthex-backend-host>:3000/api/v1/discovery'
        refresh_interval: 30s
```

---

## 📄 API Documentation

Narthex includes a built-in Swagger UI for exploring and testing the REST API.

Once the backend is running, access it at:
👉 `http://localhost:3000/swagger-ui`

---

## ⚖️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
