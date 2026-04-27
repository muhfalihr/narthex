# Stage 1: Build Rust Backend
FROM rust:1.88-slim AS backend-builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests and build dependencies for caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source and build binary
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Build SvelteKit Frontend
FROM node:22-alpine AS frontend-builder
WORKDIR /app/ui

# Install dependencies
COPY ui/package.json ui/package-lock.json ./
RUN npm ci

# Build the application
COPY ui ./
RUN npm run build

# Remove development dependencies
RUN npm prune --omit=dev

# Stage 3: Runtime Environment
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y nodejs npm ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/narthex ./narthex

# Copy frontend build and node_modules
COPY --from=frontend-builder /app/ui/build ./ui/build
COPY --from=frontend-builder /app/ui/package.json ./ui/package.json
COPY --from=frontend-builder /app/ui/node_modules ./ui/node_modules

# Copy and setup entrypoint
COPY entrypoint.sh ./
RUN chmod +x entrypoint.sh

# Environment defaults
# Backend expected on 3000 by API_BASE
ENV APP_PORT=3000
ENV APP_HOST=0.0.0.0
# Frontend expected on 8080
ENV PORT=8080

EXPOSE 3000 8080
ENTRYPOINT ["./entrypoint.sh"]
