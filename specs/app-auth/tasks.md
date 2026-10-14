# Tasks — app-auth

> Todas las tasks son de **`app`**; `api` no cambia. Las comprobaciones
> automáticas van en `verifications/app-auth/auto.sh` (lo crea T1, ejecutable
> con `chmod +x`). Como `api` no se toca, el script **no** llama a
> `start_env`: solo carga `verifications/lib.sh` para usar `check` y busca en
> el código. Las comprobaciones "cita" se documentan en
> `progress/app-auth/impl.md` con `archivo:línea` y una frase que explique por
> qué hacen verdadero el `R<n>` (ver `docs/verification.md`, nivel 3).
> Después de cada task, `./init.sh` debe seguir en `[OK]`.

- [x] T1 — Añadir `readonly VITE_API_URL: string` a `ImportMetaEnv` en
  `app/src/vite-env.d.ts`, `VITE_API_URL= ~` a `app/.env.example` y
  `export const API_URL = import.meta.env.VITE_API_URL` a
  `app/src/constants/Environment.ts`. Añadir al `app/.env` local (no
  versionado) `VITE_API_URL=http://localhost:3000`.
  Comprobación: crear `verifications/app-auth/auto.sh` con
  V1 (`grep -c "readonly VITE_API_URL: string" app/src/vite-env.d.ts` es 1),
  V2 (`grep -c "^VITE_API_URL=" app/.env.example` es 1) y
  V3 (`grep -c "export const API_URL = import.meta.env.VITE_API_URL" app/src/constants/Environment.ts` es 1).
  Cubre: R1, R2, R3.

- [x] T2 — Crear `app/src/services/apiClient.ts` con `ApiError`,
  `ApiResponse<T>`, `ApiResult<T>`, el type guard `isApiResponse` y `post`
  según `design.md` (URL `` `${API_URL}${path}` ``, `Content-Type`,
  `Authorization: Bearer` si hay token, `{ isNetworkError: true }` si `fetch`
  rechaza o el cuerpo no es un `ApiResponse`).
  Comprobación: añadir a `auto.sh`
  V4 (`grep -rl "fetch(" app/src` devuelve solo `app/src/services/apiClient.ts`),
  V5 (`grep -rl "API_URL" app/src` devuelve solo `constants/Environment.ts` y
  `services/apiClient.ts`, y `apiClient.ts` contiene `` `${API_URL}${path}` ``),
  V6 (`apiClient.ts` contiene `interface ApiResponse<T>`, `success: boolean`,
  `error: ApiError | null` y `data: T | null`, y `ApiError` con los tres
  `type` y `message: string`),
  V7 (`apiClient.ts` contiene `` Authorization: `Bearer ${ ``).
  Citas: V7 también con `archivo:línea` de la cabecera condicionada a
  `sessionToken`; V8 — el `catch` y el type guard que devuelven
  `{ isNetworkError: true }`.
  Cubre: R4, R5, R6, R13 (y la base de R30 y R43).

- [x] T3 — Crear `app/src/services/authService.ts` (`RegisterRequest`,
  `RegisterResponse`, `LoginRequest`, `LoginResponse`, `register`, `login`) y
  `app/src/services/dummyService.ts` (`DummyRequest`, `DummyResponse`,
  `dummy`) según `design.md`.
  Comprobación: añadir a `auto.sh`
  V9 (`authService.ts` declara los cuatro tipos con `email: string`,
  `password: string` y `session_token: string`),
  V10 (`authService.ts` contiene `Promise<ApiResult<RegisterResponse>>` y
  `Promise<ApiResult<LoginResponse>>`, y `ApiResult<T>` en `apiClient.ts` lleva
  `response: ApiResponse<T>`),
  V11 (para cada ruta `"/auth/register"`, `"/auth/login"` y `"/dummy"` que
  aparece en `app/src/services/`, `api/src/api/mod.rs` contiene
  `.route("<ruta>", post(`),
  V12 (cada campo de los tipos de `services/` — `email`, `password`,
  `session_token`, `dummy_string`, `dummy_message` — aparece como
  `<campo>: String` en `api/src/api/auth_api.rs` o `api/src/api/dummy_api.rs`).
  Cubre: R7, R8, R9, R10, R11.

- [x] T4 — Componentes `ui/` y tema: añadir `isLoading` y `variant` a
  `components/ui/Button/Button.tsx` y a `hooks/ui/useButton.tsx`; añadir
  `type` a `components/ui/TextInput/TextInput.tsx` y a
  `hooks/ui/useTextInput.tsx`; cambiar el hover en `app/src/theme.ts`; añadir
  `passwordPattern` a `constants/Patterns.ts`. Todo según `design.md`.
  Comprobación: añadir a `auto.sh`
  V13 (`grep -c "brightness(115%)" app/src/theme.ts` es 1 y está bajo
  `_hover` de `baseStyle`),
  V14 (cada variante de `theme.ts` tiene `_hover: { _disabled: {` con
  `var(--ash-gray)` y `brightness(85%)`),
  V15 (`grep -c "export const passwordPattern = /^.{8,72}\$/u" app/src/constants/Patterns.ts` es 1).
  Citas: V13 y V14 con `theme.ts:línea`; V16 — `Button.tsx:línea` pasa
  `isLoading` a `ChakraUiButton`.
  Cubre: R23, R24 (y la base de R15, R22, R34, R42).

- [x] T5 — Crear `app/src/hooks/useAuth.ts` con `Session`, `AuthAction`,
  `Authorize`, `login`, `register`, `authorize` y `clearError`, y la tabla de
  mensajes de `design.md`.
  Comprobación: añadir a `auto.sh`
  V17 (`grep -rl "session_token" app/src` devuelve solo
  `services/authService.ts` y `hooks/useAuth.ts`, y
  `grep -rlE "localStorage|sessionStorage|document.cookie" app/src` no
  devuelve nada),
  V18 (`grep -rn "\.message" app/src/App.tsx app/src/components app/src/hooks`
  no devuelve nada),
  V19 (`useAuth.ts` contiene literalmente los cinco mensajes:
  "Incorrect email or password", "Could not sign up with this email and
  password", "Could not connect to the server", "Something went wrong, try
  again later" y "Your session has expired, log in again").
  Citas en `useAuth.ts`: V20 — el `useState<Session | null>` que guarda el
  token (R12); V21 — `login` guarda `data.session_token` con 200 y
  `success` (R17); V22 — `register` ídem (R26); V23 — 401 + `AuthError` en
  login → mensaje de R28; V24 — 400 + `ValidationError` en registro →
  mensaje de R29; V25 — `isNetworkError` → mensaje de R30; V26 — resto →
  mensaje de R31; V27 — `authorize` pone la sesión a `null` con 401 (R35);
  V28 — y fija el mensaje de R37.
  Cubre: R12, R17, R26, R28, R29, R30, R31, R32, R35, R37.

- [x] T6 — Crear `app/src/hooks/useDummy.ts` según `design.md`.
  Comprobación: añadir a `auto.sh` V29 (`useDummy.ts` contiene `authorize(`
  y `dummy(`, y no importa `useAuth`).
  Citas en `useDummy.ts`: V30 — `checkSession` envía `POST /dummy` a través
  de `authorize` con el token (R39); V31 — 200 + `success` → "Session is
  active" (R40); V32 — `isNetworkError` → "Could not connect to the server"
  (R43); V33 — error distinto de 401 → "Something went wrong, try again
  later" (R44).
  Cubre: R39, R40, R43, R44.

- [x] T7 — Crear `app/src/components/Login/Login.tsx` y `Login.css` (moviendo
  los estilos de `#login-container` desde `App.css`) según `design.md`.
  Comprobación: añadir a `auto.sh`
  V34 (`Login.tsx` contiene `passwordPattern, "password")` y `TextInput.tsx`
  pasa `type={type}` a `Input`),
  V35 (`Login.tsx` contiene `"Log in"` y `"Sign up"`).
  Citas en `Login.tsx`: V36 — campos "Email" / "Password" y los dos botones
  (R14); V37 — `handleLogin` llama a `login` con email y contraseña (R16);
  V38 — `handleRegister` llama a `register` (R25); V39 — `isDisabled` incluye
  `emailValue === "" || isEmailInvalid` (R21); V40 — e
  `passwordValue === "" || isPasswordInvalid` (R22); V41 — y
  `pendingAction !== null` (R33); V42 — `isLoading` es
  `pendingAction === "login"` / `"register"` en cada botón (R34); V43 — se
  renderiza `errorMessage` (soporte de R28-R31, R37).
  Cubre: R14, R15, R16, R21, R22, R25, R33, R34.

- [x] T8 — Crear `app/src/components/Home/Home.tsx` y `Home.css` según
  `design.md`.
  Comprobación: añadir a `auto.sh` V44 (`Home.tsx` contiene
  `Logged in as`, `Signed up as` y `"Check session"`).
  Citas en `Home.tsx`: V45 — texto según `sessionAction === "login"` (R19);
  V46 — y `"register"` con el email (R27); V47 — botón "Check session"
  (R38); V48 — `isDisabled = isChecking` (R41); V49 — `isLoading = isChecking`
  (R42); V50 — se renderiza `checkMessage` (soporte de R40, R43, R44).
  Cubre: R19, R27, R38, R41, R42.

- [x] T9 — Reescribir `app/src/App.tsx`: `useAuth()` y `Home` o `Login` según
  `design.md`, sin `Math.random()` ni textos simulados; quitar de `App.css`
  los estilos movidos a `Login.css`.
  Comprobación: añadir a `auto.sh` V51 (`grep -rn "Math.random" app/src` no
  devuelve nada, ni `"Login link sent by email"`).
  Citas en `App.tsx`: V52 — con `isLoggedIn` se renderiza `Home` y no
  `Login` (R18); V53 — sin sesión (también tras el 401 de `authorize`) se
  renderiza `Login` (R14, R36).
  Comprobación final: `./init.sh` en `[OK]` y
  `./verifications/app-auth/auto.sh` con todo en `[OK]`.
  Cubre: R18, R20, R36.

- [x] T10 — Escribir `verifications/app-auth/manual.md` (formato de
  `docs/verification.md`). Cubre todos los `R<n>` (todos son de `app`).
  Preparación: `./init.sh`, `docker compose up -d db`, `cd api && cargo run`,
  `VITE_API_URL=http://localhost:3000` en `app/.env`, `cd app && npm run dev`
  y `http://localhost:5173`. Casos mínimos:
  - Email inválido / contraseña de menos de 8 caracteres deshabilitan los dos
    botones (R21, R22); la contraseña se ve como puntos (R15).
  - Hover sobre botón habilitado (más claro) y deshabilitado (sin cambio)
    (R23, R24).
  - Registro con email nuevo → spinner en "Sign up", ambos botones
    deshabilitados (R33, R34) y después "Signed up as <email>" (R7, R9-R11,
    R25-R27, R18).
  - Registro con el mismo email → "Could not sign up with this email and
    password" (R29).
  - Login con las credenciales correctas → "Logged in as <email>" (R8, R16,
    R17, R19); contraseña incorrecta → "Incorrect email or password" (R28).
  - Con `api` parada → "Could not connect to the server" (R1-R6, R30); con
    CouchDB parada y `api` en marcha → "Something went wrong, try again
    later" (R31). En ningún caso aparecen textos de `api` ni de excepciones
    (R32).
  - "Check session" con sesión válida → spinner y botón deshabilitado, luego
    "Session is active" (R13, R38-R42); con `api` parada → "Could not connect
    to the server" (R43); con CouchDB parada → "Something went wrong, try
    again later" (R44).
  - 401: borrar el documento de la sesión en Fauxton
    (`http://localhost:5984/_utils`, `admin` / `password`, base `sessions`) y
    pulsar "Check session" → vuelve la pantalla de login con "Your session has
    expired, log in again" (R12, R35-R37).
  - Recorrer la app y confirmar que ninguna respuesta es aleatoria: el mismo
    login repetido da siempre el mismo resultado (R20).
  Comprobación: el reviewer comprueba que cada `R1`-`R44` aparece en el
  `Cubre:` de al menos un `P<n>`.
  Cubre: R1-R44.
