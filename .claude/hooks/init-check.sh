#!/usr/bin/env bash
# Stop / SubagentStop: runs ./init.sh (with the given arguments, e.g. --quick)
# before the agent finishes. If it fails, exits 2 so the agent keeps working
# on it (only once per stop, to avoid loops).

stop_hook_active=$(jq -r '.stop_hook_active // false')
cd "${CLAUDE_PROJECT_DIR:-$(pwd)}" || exit 0
cmd="./init.sh${*:+ $*}"

if output=$(./init.sh "$@" 2>&1); then
  echo "{\"systemMessage\": \"[harness] $cmd OK\"}"
  exit 0
fi

if [ "$stop_hook_active" = "true" ]; then
  echo "{\"systemMessage\": \"[harness] $cmd FALLÓ — no marques nada como done\"}"
  exit 0
fi

echo "[harness] $cmd FALLÓ — corrígelo antes de terminar. Si no está en tu mano, pon la feature en curso en blocked, con el motivo en su informe de progress/<feature>/ referenciado desde progress/current.md:" >&2
echo "$output" | grep -A30 -F '[FAIL]' >&2
exit 2
