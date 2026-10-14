#!/usr/bin/env bash
# Shared helpers of the verification scripts (docs/verification.md).
# Load it from verifications/<feature-name>/auto.sh with:
#   source "$(dirname "$0")/../lib.sh"
# The script exits with code != 0 if any check fails.

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
DB_PORT=5985
APP_PORT=3001
DB="http://admin:password@localhost:$DB_PORT"
API="http://localhost:$APP_PORT"
DB_CONTAINER=zerrenda-app-verify-db
FAILED=0
API_PID=""
API_LOG=""

check() {
  if [ "$3" = "$4" ]; then
    echo "[OK]   $1 — $2"
  else
    echo "[FAIL] $1 — $2 (expected: $3, got: $4)"
    FAILED=1
  fi
}

fail_env() {
  echo "[FAIL] ENV — $1"
  [ -n "$API_LOG" ] && tail -20 "$API_LOG"
  FAILED=1
  exit 1
}

# Starts an empty, throwaway CouchDB (no volume) and the api on top of it, on
# ports other than those of compose.yaml, so the local environment can keep running.
start_env() {
  [ -f "$ROOT/api/.env" ] || fail_env "api/.env not found: run ./init.sh to create it"
  docker rm -fv "$DB_CONTAINER" > /dev/null 2>&1
  docker run -d --rm --name "$DB_CONTAINER" -p "127.0.0.1:$DB_PORT:5984" \
    --tmpfs /opt/couchdb/data \
    -v "$ROOT/db/local.ini:/opt/couchdb/etc/local.d/single-node.ini:ro" \
    -e COUCHDB_USER=admin -e COUCHDB_PASSWORD=password \
    couchdb:3.5.0 > /dev/null || fail_env "CouchDB container did not start"
  curl -sf --retry 30 --retry-delay 1 --retry-all-errors "$DB/_all_dbs" > /dev/null \
    || fail_env "CouchDB does not respond on port $DB_PORT"

  (cd "$ROOT/api" && cargo build --quiet) || fail_env "api does not build"
  API_LOG=$(mktemp)
  (
    cd "$ROOT/api" && exec env APP_HOST=127.0.0.1 APP_PORT="$APP_PORT" \
      DB_HOST="http://localhost:$DB_PORT/" DB_USER=admin DB_PASSWORD=password \
      ./target/debug/zerrenda-app-api
  ) > "$API_LOG" 2>&1 &
  API_PID=$!
  curl -sf --retry 30 --retry-delay 1 --retry-all-errors "$API/healthcheck" > /dev/null \
    || fail_env "api does not respond on port $APP_PORT"
}

# Runs when the script exits, even if a check fails: stops the api and removes
# the CouchDB container, and with it all the test data.
finish() {
  if [ -n "$API_PID" ]; then
    kill "$API_PID" 2> /dev/null
    wait "$API_PID" 2> /dev/null
  fi
  docker rm -fv "$DB_CONTAINER" > /dev/null 2>&1
  [ -n "$API_LOG" ] && rm -f "$API_LOG"
  exit $FAILED
}

trap finish EXIT
