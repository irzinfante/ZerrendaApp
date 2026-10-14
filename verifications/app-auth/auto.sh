#!/usr/bin/env bash
# Checks of app-auth (specs/app-auth/). See docs/verification.md.
# Only app changes, so there is no start_env: every check searches the code.
source "$(dirname "$0")/../lib.sh"

SRC="$ROOT/app/src"

# T1 — VITE_API_URL
check V1 "VITE_API_URL declared in ImportMetaEnv" 1 \
  "$(grep -c "readonly VITE_API_URL: string" "$SRC/vite-env.d.ts")"
check V2 "VITE_API_URL documented in app/.env.example" 1 \
  "$(grep -c "^VITE_API_URL=" "$ROOT/app/.env.example")"
check V3 "API_URL exported from constants/Environment.ts" 1 \
  "$(grep -c "export const API_URL = import.meta.env.VITE_API_URL" "$SRC/constants/Environment.ts")"

# T2 — base client
CLIENT="$SRC/services/apiClient.ts"
check V4 "fetch( only in services/apiClient.ts" "services/apiClient.ts" \
  "$(grep -rl "fetch(" "$SRC" | sed "s|^$SRC/||" | sort | tr '\n' ' ' | sed 's/ $//')"
check V5 "API_URL only in Environment.ts and apiClient.ts, URL built as API_URL + path" \
  "constants/Environment.ts services/apiClient.ts 1" \
  "$(grep -rlw "API_URL" "$SRC" | sed "s|^$SRC/||" | sort | tr '\n' ' ')$(grep -c '`${API_URL}${path}`' "$CLIENT")"
check V6 "ApiResponse<T> and ApiError declared with the api fields" "1 1 1 1 1 1" \
  "$(grep -c "export interface ApiResponse<T> {" "$CLIENT") $(grep -c "^  success: boolean$" "$CLIENT") $(grep -c "^  error: ApiError | null$" "$CLIENT") $(grep -c "^  data: T | null$" "$CLIENT") $(grep -c '^  type: "ValidationError" | "AuthError" | "InternalError"$' "$CLIENT") $(grep -c "^  message: string$" "$CLIENT")"
check V7 "Authorization: Bearer <session_token> header" 1 \
  "$(grep -c 'Authorization: `Bearer ${sessionToken}`' "$CLIENT")"

# T3 — services and contract with api
AUTH="$SRC/services/authService.ts"
DUMMY="$SRC/services/dummyService.ts"
fields() { # prints the fields of interface $2 in file $1, one line, sorted
  sed -n "/^export interface $2 {/,/^}/p" "$1" | grep -E '^  [a-z_]+: ' | sed 's/^  //' | sort | tr '\n' ';'
}
check V9 "Register*/Login* declared with their fields" \
  "email: string;password: string;|session_token: string;|email: string;password: string;|session_token: string;" \
  "$(fields "$AUTH" RegisterRequest)|$(fields "$AUTH" RegisterResponse)|$(fields "$AUTH" LoginRequest)|$(fields "$AUTH" LoginResponse)"
check V10 "register/login typed with ApiResponse<RegisterResponse>/<LoginResponse> via ApiResult" "1 1 1" \
  "$(grep -c "Promise<ApiResult<RegisterResponse>>" "$AUTH") $(grep -c "Promise<ApiResult<LoginResponse>>" "$AUTH") $(grep -c "response: ApiResponse<T>" "$CLIENT")"
ROUTES=$(grep -rhoE 'post<[A-Za-z]+>\("[^"]+"' "$SRC/services" | sed -E 's/.*\("//; s/"$//' | sort -u)
MISSING=""
for ROUTE in $ROUTES; do
  grep -qF ".route(\"$ROUTE\", post(" "$ROOT/api/src/api/mod.rs" || MISSING="$MISSING $ROUTE"
done
check V11 "every route of services/ is a POST route of api/src/api/mod.rs" \
  "/auth/login /auth/register /dummy|" "$(tr '\n' ' ' <<< "$ROUTES" | sed 's/ $//')|$MISSING"
DTO_DIFF=""
for FILE in "$AUTH" "$DUMMY"; do
  for TYPE in $(grep -oE '^export interface [A-Za-z]+(Request|Response) ' "$FILE" | awk '{print $3}'); do
    TS=$(sed -n "/^export interface $TYPE {/,/^}/p" "$FILE" | grep -oE '^  [a-z_]+' | tr -d ' ' | sort | tr '\n' ' ')
    RS=$(sed -n "/^pub struct $TYPE {/,/^}/p" "$ROOT"/api/src/api/*_api.rs | grep -oE '^    [a-z_]+:' | tr -d ' :' | sort | tr '\n' ' ')
    [ -n "$RS" ] && [ "$TS" = "$RS" ] || DTO_DIFF="$DTO_DIFF $TYPE"
  done
done
check V12 "fields of each services/ *Request/*Response match the api DTO with the same name" "" "$DTO_DIFF"

# T4 — ui components and theme
THEME="$SRC/theme.ts"
check V13 "enabled buttons get brighter on hover (baseStyle._hover)" "1 1" \
  "$(grep -c "brightness(115%)" "$THEME") $(sed -n '/baseStyle: {/,/^      },/p' "$THEME" | grep -c '^        _hover: { filter: "brightness(115%)" },$')"
BUTTON_VARIANTS=$(sed -n '/^    Button: {/,/^    },/p' "$THEME" | sed -n '/^      variants: {/,/^      },/p')
VARIANTS=$(grep -cE '^        [a-z]+: \{$' <<< "$BUTTON_VARIANTS")
HOVER_DISABLED=$(grep -c '_hover: { _disabled: { bg: "var(--ash-gray)", filter: "brightness(85%)" } }' <<< "$BUTTON_VARIANTS")
check V14 "every button variant keeps the disabled look on hover" "2 2" "$VARIANTS $HOVER_DISABLED"
check V15 "passwordPattern of 8 to 72 characters" 1 \
  "$(grep -c 'export const passwordPattern = /^.{8,72}$/u' "$SRC/constants/Patterns.ts")"

# T5 — useAuth
USE_AUTH="$SRC/hooks/useAuth.ts"
check V17 "session_token only in authService.ts and useAuth.ts, no browser storage" \
  "hooks/useAuth.ts services/authService.ts |" \
  "$(grep -rl "session_token" "$SRC" | sed "s|^$SRC/||" | sort | tr '\n' ' ')|$(grep -rlE "localStorage|sessionStorage|document.cookie" "$SRC")"
check V18 "error.message of api is never read outside services/" "" \
  "$(grep -rn "\.message" "$SRC/App.tsx" "$SRC/components" "$SRC/hooks")"
MESSAGES=0
for MESSAGE in "Incorrect email or password" "Could not sign up with this email and password" \
  "Could not connect to the server" "Something went wrong, try again later" \
  "Your session has expired, log in again"; do
  grep -qF "\"$MESSAGE\"" "$USE_AUTH" && MESSAGES=$((MESSAGES + 1))
done
check V19 "useAuth.ts defines the five user messages" 5 "$MESSAGES"

# T6 — useDummy
USE_DUMMY="$SRC/hooks/useDummy.ts"
check V29 "useDummy calls dummy( through authorize( and does not use the useAuth hook" "1 1 0" \
  "$(grep -c "authorize((sessionToken) => dummy(" "$USE_DUMMY") $(grep -c "dummy({ dummy_string: " "$USE_DUMMY") $(grep -cE "import useAuth|useAuth\(" "$USE_DUMMY")"

# T7 — Login screen
LOGIN="$SRC/components/Login/Login.tsx"
check V34 "Password field is created with type password and TextInput passes type to Input" "1 1" \
  "$(grep -c 'useTextInput("Password", "", passwordPattern, "password")' "$LOGIN") $(grep -c '^      type={type}$' "$SRC/components/ui/TextInput/TextInput.tsx")"
check V35 "Login screen has the Email and Password fields and the Log in and Sign up buttons" "1 1 1 1" \
  "$(grep -c 'useTextInput("Email", ' "$LOGIN") $(grep -c 'useTextInput("Password", ' "$LOGIN") $(grep -c 'useButton("Log in", ' "$LOGIN") $(grep -c 'useButton("Sign up", ' "$LOGIN")"

# T8 — Home screen
HOME_TSX="$SRC/components/Home/Home.tsx"
check V44 "Home screen shows Logged in as / Signed up as and the Check session button" "1 1 1" \
  "$(grep -c 'Logged in as ${email}' "$HOME_TSX") $(grep -c 'Signed up as ${email}' "$HOME_TSX") $(grep -c 'useButton("Check session", ' "$HOME_TSX")"

# T9 — App.tsx without simulated responses
check V51 "no Math.random nor simulated texts in app/src" "|" \
  "$(grep -rn "Math.random" "$SRC")|$(grep -rnE "Login link sent by email|Login attempt failed" "$SRC")"
