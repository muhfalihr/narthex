# Narthex UI

The SvelteKit 5 frontend for [Narthex](../README.md) — a Prometheus HTTP Service Discovery manager.

## Stack

- [SvelteKit 5](https://svelte.dev/) with `@sveltejs/adapter-node`
- [TailwindCSS](https://tailwindcss.com/)
- TypeScript

## Development

```bash
# Install dependencies
npm install

# Start dev server (hot reload)
npm run dev
```

The dev server runs on `http://localhost:5173` and proxies API calls to the backend at `http://localhost:3000`.

Requires the Rust backend to be running. See the [root README](../README.md#local-development) for setup steps.

## Available Scripts

| Script            | Description                        |
|:----------------- |:---------------------------------- |
| `npm run dev`     | Start development server           |
| `npm run build`   | Production build (output: `build/`) |
| `npm run preview` | Preview the production build        |
| `npm run check`   | Svelte type checking                |
| `npm run lint`    | ESLint                              |
| `npm run format`  | Prettier formatting                 |

## Production Build

```bash
npm run build
# Artifacts are in ./build/
# Served with: node build/index.js
```

In Docker, the build output is served by the Node.js adapter on `PORT` (default `8080`). The container's `entrypoint.sh` starts both this and the Rust backend.

## Authentication

When `APP_USERNAME` and `APP_PASSWORD` are set on the backend, the UI enforces a login page. The session is managed via a secure cookie; logout is available from the sidebar.
