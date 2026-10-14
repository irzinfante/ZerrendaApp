#!/usr/bin/env bash
# PostToolUse (Edit|Write): type-checks the project that contains the edited file.
# On failure, exits 2 so the errors are fed back to Claude.

file=$(jq -r '.tool_input.file_path // .tool_response.filePath // empty')
root="${CLAUDE_PROJECT_DIR:-$(pwd)}"

case "$file" in
  "$root"/api/src/*.rs | "$root"/api/Cargo.toml)
    project="api"
    output=$(cd "$root/api" && cargo check --quiet 2>&1)
    status=$?
    ;;
  "$root"/app/src/*.ts | "$root"/app/src/*.tsx | "$root"/app/tsconfig*.json | "$root"/app/package.json)
    project="app"
    output=$(cd "$root/app" && npx tsc -b 2>&1)
    status=$?
    ;;
  *)
    exit 0
    ;;
esac

if [ "$status" -ne 0 ]; then
  echo "[harness] $project no compila tras editar $file:" >&2
  echo "$output" | tail -30 >&2
  exit 2
fi

exit 0
