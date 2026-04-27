#!/bin/bash
set -e

echo "Starting Narthex Backend on port ${APP_PORT:-3000}..."
./narthex &
BACKEND_PID=$!

echo "Starting SvelteKit Frontend on port ${PORT:-8080}..."
cd ui
node build/index.js &
FRONTEND_PID=$!

# Wait for any process to exit
wait -n

# Exit with the status of the first process that exited
exit $?
