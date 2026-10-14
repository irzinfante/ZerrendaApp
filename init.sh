#!/usr/bin/env bash
# init.sh — Verificación e inicialización del entorno
#
# Este script lo ejecuta el agente al COMENZAR una sesión y antes de
# declarar cualquier tarea como `done`. Si falla, la sesión no debe avanzar.
# Si no existe api/.env, lo crea a partir de api/.env.example.
#
# Salida esperada: códigos de salida claros y bloques marcados con [OK]/[FAIL].
#
# Con --quick omite la compilación, el formato y el lint (bloque 4): lo usan
# los hooks de los agentes que no tocan código.

set -u
cd "$(dirname "$0")" || exit 1

QUICK=0
if [ "${1:-}" = "--quick" ]; then QUICK=1; fi

if [ -t 1 ]; then
  RED='\033[0;31m'
  GREEN='\033[0;32m'
  NC='\033[0m'
else
  RED=''
  GREEN=''
  NC=''
fi

ok()    { printf "${GREEN}[OK]${NC}    %s\n" "$1"; }
fail()  { printf "${RED}[FAIL]${NC}  %s\n" "$1"; }

EXIT_CODE=0

# Ejecuta un comando; si falla, muestra el final de su salida.
run() {
  local label=$1 output
  shift
  if output=$("$@" 2>&1); then
    ok "$label"
  else
    fail "$label"
    echo "$output" | tail -30
    EXIT_CODE=1
  fi
}

# Compila app, instalando antes las dependencias si faltan o han cambiado
# (puede actualizar package-lock.json, p. ej. al añadir una dependencia).
build_app() {
  (
    cd app || exit 1
    if [ ! -d node_modules ] || [ package.json -nt node_modules ] \
      || { [ -f package-lock.json ] && [ package-lock.json -nt node_modules ]; }; then
      npm install && touch node_modules || exit 1
    fi
    npm run build
  )
}

# Regla de arquitectura: OK si la búsqueda no encuentra nada.
arch() {
  if [ -z "$2" ]; then
    ok "$1"
  else
    fail "$1:"
    echo "$2" | sed 's/^/        /'
    EXIT_CODE=1
  fi
}

echo "── 1. Verificando entorno ─────────────────────────────"

for cmd in node npm cargo docker jq python3; do
  if command -v "$cmd" >/dev/null 2>&1; then
    ok "$cmd -> $("$cmd" --version 2>&1 | head -1)"
  else
    fail "$cmd no está instalado"
    EXIT_CODE=1
  fi
done
if [ $EXIT_CODE -ne 0 ]; then exit 1; fi

# Valor local de cada variable de api/.env.example, el mismo entorno que
# compose.yaml. Una variable nueva en api/.env.example se añade aquí.
api_env_value() {
  case "$1" in
    APP_HOST) echo "0.0.0.0" ;;
    APP_PORT) echo "3000" ;;
    DB_HOST) echo "http://localhost:5984/" ;;
    DB_USER) echo "admin" ;;
    DB_PASSWORD) echo "password" ;;
    AUTH_PRIVATE_KEY) python3 -c "import secrets; print(secrets.token_hex(32))" ;;
    CORS_ALLOWED_ORIGINS) echo "http://localhost:5173,http://localhost" ;;
    *) return 1 ;;
  esac
}

API_ENV_KEYS=$(grep -oE '^[A-Z_]+=' api/.env.example | tr -d '=')
if [ -f api/.env ]; then
  missing=$(for key in $API_ENV_KEYS; do grep -q "^$key=" api/.env || printf " %s" "$key"; done)
  if [ -z "$missing" ]; then
    ok "api/.env tiene todas las variables de api/.env.example"
  else
    fail "A api/.env le faltan variables de api/.env.example:$missing"
    EXIT_CODE=1
  fi
else
  missing=$(for key in $API_ENV_KEYS; do api_env_value "$key" > /dev/null || printf " %s" "$key"; done)
  if [ -z "$missing" ]; then
    while IFS= read -r line || [ -n "$line" ]; do
      key=${line%%=*}
      if [[ $line =~ ^[A-Z_]+= ]]; then
        echo "$key=$(api_env_value "$key")"
      else
        echo "$line"
      fi
    done < api/.env.example > api/.env
    ok "Creado api/.env a partir de api/.env.example con los valores locales"
  else
    fail "No se puede crear api/.env: sin valor local en init.sh para$missing"
    EXIT_CODE=1
  fi
fi

echo ""
echo "── 2. Verificando archivos base del arnés ──────────────"

for f in AGENTS.md CHECKPOINTS.md feature_list.json progress/current.md \
  progress/history.md docs/api-architecture.md docs/api-conventions.md \
  docs/app-architecture.md docs/app-conventions.md docs/specs.md docs/verification.md \
  verifications/lib.sh; do
  if [ ! -f "$f" ]; then
    fail "Falta archivo base: $f"
    EXIT_CODE=1
  else
    ok "Existe $f"
  fi
done

echo ""
echo "── 3. Validando feature_list.json y specs ─────────────"

python3 - <<'PY'
import json, os, sys
try:
    data = json.load(open("feature_list.json"))
    valid = {"pending", "spec_ready", "in_progress", "done", "blocked"}
    in_progress = [f for f in data["features"] if f["status"] == "in_progress"]
    if len(in_progress) > 1:
        print(f"[FAIL]  Hay {len(in_progress)} features en in_progress (máximo 1)")
        sys.exit(1)
    requires_spec = {"spec_ready", "in_progress", "done"}
    spec_errors = []
    for f in data["features"]:
        if f["status"] not in valid:
            print(f"[FAIL]  Estado inválido en feature {f['id']}: {f['status']}")
            sys.exit(1)
        if f.get("sdd") and f["status"] in requires_spec:
            spec_dir = os.path.join("specs", f["name"])
            for fname in ("requirements.md", "design.md", "tasks.md"):
                if not os.path.isfile(os.path.join(spec_dir, fname)):
                    spec_errors.append(
                        f"feature {f['id']} ({f['name']}) en {f['status']} "
                        f"sin {spec_dir}/{fname}"
                    )
    if spec_errors:
        for e in spec_errors:
            print(f"[FAIL]  {e}")
        sys.exit(1)
    print(f"[OK]    feature_list.json válido ({len(data['features'])} features)")
    print(f"[OK]    Specs presentes para features sdd con estado no-pending")
except SystemExit:
    raise
except Exception as e:
    print(f"[FAIL]  feature_list.json o specs inválidos: {e}")
    sys.exit(1)
PY

if [ $? -ne 0 ]; then EXIT_CODE=1; fi

echo ""
echo "── 4. Compilando y pasando formato y lint ──────────────"

if [ $QUICK -eq 1 ]; then
  echo "Omitido (--quick)"
else
  run "npm run build (app compila)" build_app
  run "cargo build (api compila)" cargo build --manifest-path api/Cargo.toml
  run "cargo fmt --check (api)" cargo fmt --manifest-path api/Cargo.toml --check
  run "cargo clippy -- -D warnings (api)" \
    cargo clippy --manifest-path api/Cargo.toml --quiet -- -D warnings
  run "npm run lint (app)" npm run lint --prefix app
fi

echo ""
echo "── 5. Comprobando la arquitectura de app ───────────────"

# Reglas de docs/app-architecture.md que se detectan buscando en el código.
SRC=app/src
arch "fetch solo en services/" "$(grep -rl 'fetch(' "$SRC" --exclude-dir=services)"
arch "import.meta.env solo en constants/Environment.ts" \
  "$(grep -rl 'import.meta.env' "$SRC" | grep -vx "$SRC/constants/Environment.ts")"
arch "components/ui sin estado ni red" \
  "$(grep -rlE 'useState|useEffect|fetch\(|services/' "$SRC/components/ui")"
for VAR in $(grep -o 'VITE_[A-Z_]*' "$SRC/vite-env.d.ts"); do
  arch "$VAR en app/.env.example y constants/Environment.ts" \
    "$(grep -L "$VAR" app/.env.example "$SRC/constants/Environment.ts")"
done

echo ""
echo "── 6. Resumen ──────────────────────────────────────────"

if [ $EXIT_CODE -eq 0 ]; then
  ok "Entorno listo. Puedes empezar a trabajar."
else
  fail "Entorno NO está listo. Resuelve los errores antes de avanzar."
fi

exit $EXIT_CODE
