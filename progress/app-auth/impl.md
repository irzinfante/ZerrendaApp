# Implementación — app-auth

> Feature 2 (`in_progress`). Solo se toca `app`; `api` no cambia.
> Spec: `specs/app-auth/` — T1..T10 marcadas `[x]`.

## Resumen

- **Configuración:** `VITE_API_URL` en `app/src/vite-env.d.ts`,
  `app/.env.example` y `API_URL` en `app/src/constants/Environment.ts`. El
  `app/.env` local (no versionado) tiene `VITE_API_URL=http://localhost:3000`.
- **`services/`:** `apiClient.ts` (cliente base, único `fetch`: `ApiError`,
  `ApiResponse<T>`, `ApiResult<T>`, type guard `isApiResponse`, `post`),
  `authService.ts` (`Register*` / `Login*`, `register`, `login`) y
  `dummyService.ts` (`Dummy*`, `dummy`).
- **Hooks de dominio:** `hooks/useAuth.ts` (único dueño del `session_token`,
  solo en memoria; `login`, `register`, `authorize`, `clearError` y la tabla
  de mensajes) y `hooks/useDummy.ts` (estado de "Check session"; recibe
  `authorize` por parámetro).
- **`ui/`:** `Button` con `isLoading` y `variant`, `TextInput` con `type`, y
  sus hooks con esos parámetros opcionales. `theme.ts`: hover
  `brightness(115%)` en habilitado; en deshabilitado se mantiene
  `var(--ash-gray)` + `brightness(85%)`. `passwordPattern` en
  `constants/Patterns.ts`.
- **Pantallas:** `components/Login/` (email, contraseña, "Log in" / "Sign up",
  spinner y deshabilitado durante la petición, mensaje de error) y
  `components/Home/` ("Logged in as" / "Signed up as", "Check session").
  `App.tsx` elige `Home` o `Login` con `useAuth()`; ya no hay `Math.random()`
  ni textos simulados. Los estilos de login pasan de `App.css` a `Login.css`.

## Desviaciones y decisiones menores (sin cambiar requirements)

1. **Firma de `post`.** `design.md` propone `post<Req, Res>(path, body: Req, …)`.
   La regla `@typescript-eslint/no-unnecessary-type-parameters` de
   `strictTypeChecked` rechaza `Req` (se usa una sola vez) y `npm run lint`
   falla. Queda `post<Res>(path: string, body: object, sessionToken)`. Los
   cuerpos siguen tipados por las funciones de servicio
   (`register(request: RegisterRequest)`, `login(request: LoginRequest)`,
   `dummy(request: DummyRequest, …)`). No afecta a ningún `R<n>`.
2. **`TextInputType` y `ButtonVariant` exportados.** Los tipos
   `"text" | "password"` y `"primary" | "secondary"` del design se declaran
   una vez en `TextInput.tsx` / `Button.tsx` y los importan sus hooks.
3. **`useEffect` de `Login` que llama a `clearError`.** Tras un 401, `App`
   desmonta `Home` y monta `Login`. Un `useEffect` sobre
   `emailValue` / `passwordValue` también se ejecuta al montar, y borraría el
   mensaje de R37 nada más aparecer (en desarrollo, `StrictMode` lo ejecuta
   dos veces). El efecto compara con los valores anteriores (un `useRef`) y
   solo llama a `clearError` si el email o la contraseña han cambiado
   (`app/src/components/Login/Login.tsx:33-40`). `clearError` es estable
   (`useCallback`, `useAuth.ts:86-88`).
4. **`authorize` sin sesión.** El design dice "si no hay sesión no llama a
   nada", pero la firma obliga a devolver un `ApiResult<T>`. Devuelve
   `{ isNetworkError: true }` (`useAuth.ts:75-77`). No se llega a ese caso:
   `Home` (el único que llama a `authorize`) solo se monta con sesión
   (`App.tsx:15-16`).
5. **Mensaje de error de `Login` dentro del flujo del contenedor.** En el
   `App.css` original estaba en `position: absolute`. Los mensajes nuevos son
   más largos, ocuparían dos líneas y se saldrían del contenedor, así que ahora
   van dentro del flujo, en `Login.css`.
6. **V12, más estricta que en `tasks.md`.** En lugar de comprobar que cada
   campo aparece en algún DTO de `api`, compara los campos de cada
   `*Request` / `*Response` de `services/` con los del `pub struct` de `api`
   que se llama igual. Para comprobar que V11 y V12 detectan errores, se
   cambió un campo y una ruta de `dummyService.ts` (las dos fallaron) y luego
   se restauró.

## Verificación

- `./init.sh` → `[OK] Entorno listo` (compilación, lint, reglas de
  arquitectura; incluye `VITE_API_URL en app/.env.example y constants/Environment.ts`).
- `./verifications/app-auth/auto.sh` → 22 comprobaciones en `[OK]`, exit 0.
  No usa `start_env` (no se toca `api`).
- `verifications/app-auth/manual.md` → P1..P12, que cubren R1..R44 (no ejecutadas: las hace el humano).

## Trazabilidad

Las V automáticas están en `verifications/app-auth/auto.sh`; las que son citas
llevan `archivo:línea` (rutas relativas a `app/src/`).

- R1 → V1: `readonly VITE_API_URL: string` en `ImportMetaEnv` (`auto.sh`).
- R2 → V2: `VITE_API_URL=` en `app/.env.example` (`auto.sh`).
- R3 → V3: `export const API_URL = import.meta.env.VITE_API_URL` en `constants/Environment.ts` (`auto.sh`).
- R4 → V4: `fetch(` solo aparece en `services/apiClient.ts` (`auto.sh`).
- R5 → V5: `API_URL` (palabra completa) solo en `Environment.ts` y `apiClient.ts`, que construye `` `${API_URL}${path}` `` (`auto.sh`); `services/apiClient.ts:38`.
- R6 → V6: `ApiResponse<T>` con `success: boolean`, `error: ApiError | null`, `data: T | null`, y `ApiError` con los tres `type` y `message: string` (`auto.sh`).
- R7 → V9: `RegisterRequest` = `email`, `password` (`string`); `RegisterResponse` = `session_token: string` (`auto.sh`).
- R8 → V9: `LoginRequest` = `email`, `password`; `LoginResponse` = `session_token: string` (`auto.sh`).
- R9 → V10: `register` / `login` devuelven `Promise<ApiResult<RegisterResponse>>` / `Promise<ApiResult<LoginResponse>>`, y `ApiResult<T>` lleva `response: ApiResponse<T>` (`auto.sh`); `services/authService.ts:21-25`.
- R10 → V11: las rutas de `services/` (`/auth/login`, `/auth/register`, `/dummy`) están todas como `.route("<ruta>", post(` en `api/src/api/mod.rs` (`auto.sh`).
- R11 → V12: los campos de cada `*Request` / `*Response` de `services/` son iguales a los del `pub struct` de `api/src/api/*_api.rs` que se llama igual (`auto.sh`).
- R12 → V17: `session_token` solo aparece en `services/authService.ts` (tipo) y `hooks/useAuth.ts`, y no hay `localStorage` / `sessionStorage` / `document.cookie` (`auto.sh`); V20: `hooks/useAuth.ts:50`, el `useState<Session | null>` es el único estado que guarda el token (solo en memoria). `apiClient.ts:27-34` y `dummyService.ts:11-12` lo reciben como parámetro y no lo guardan.
- R13 → V7: `` Authorization: `Bearer ${sessionToken}` `` en `apiClient.ts` (`auto.sh`); `services/apiClient.ts:32-34`: la cabecera se añade solo cuando `sessionToken` no es `null`.
- R14 → V35: `Login.tsx` crea "Email", "Password", "Log in" y "Sign up" (`auto.sh`); V36: `components/Login/Login.tsx:18-19,30-31,46-50` los renderiza; V53: `App.tsx:15-23`, sin sesión se renderiza `Login`.
- R15 → V34: `useTextInput("Password", "", passwordPattern, "password")` y `TextInput.tsx` pasa `type={type}` a `Input` (`auto.sh`); `components/ui/TextInput/TextInput.tsx:19`.
- R16 → V37: `components/Login/Login.tsx:23-25,30`: "Log in" llama a `handleLogin`, que llama a `login(emailValue, passwordValue)`; `hooks/useAuth.ts:70,58-59` lo envía con `loginRequest` (`POST /auth/login`, `services/authService.ts:24-25`).
- R17 → V21: `hooks/useAuth.ts:60-61`: con `status === 200`, `success` y `data` no nulo, `login` (`useAuth.ts:70`) guarda `data.session_token` en la sesión.
- R18 → V52: `App.tsx:15-16`: con `isLoggedIn` (sesión con token, `useAuth.ts:91`) se renderiza `Home` y no `Login`.
- R19 → V45: `components/Home/Home.tsx:25`: con `sessionAction === "login"` muestra `` `Logged in as ${email}` ``; `sessionAction` es la `action` guardada en `useAuth.ts:61`; V44 (`auto.sh`).
- R20 → V51: no hay `Math.random` ni los textos simulados ("Login link sent by email", "Login attempt failed") en `app/src` (`auto.sh`).
- R21 → V39: `components/Login/Login.tsx:21`: `isDisabled` incluye `emailValue === "" || isEmailInvalid` y se pasa a los dos botones (`Login.tsx:30-31`).
- R22 → V40: `components/Login/Login.tsx:19,21`: `passwordPattern` valida "Password" e `isDisabled` incluye `passwordValue === "" || isPasswordInvalid`; V15: `passwordPattern = /^.{8,72}$/u` (`auto.sh`).
- R23 → V13: `brightness(115%)` en `_hover` de `baseStyle` (`auto.sh`); `theme.ts:17`.
- R24 → V14: las dos variantes de `Button` tienen `_hover: { _disabled: { bg: "var(--ash-gray)", filter: "brightness(85%)" } }` (`auto.sh`); `theme.ts:31,35` (y `_disabled` base en `theme.ts:18-23`).
- R25 → V38: `components/Login/Login.tsx:26-28,31`: "Sign up" llama a `register(emailValue, passwordValue)`; `hooks/useAuth.ts:71,58-59` lo envía con `registerRequest` (`POST /auth/register`, `services/authService.ts:21-22`).
- R26 → V22: `hooks/useAuth.ts:60-61`: la misma condición guarda el `data.session_token` de `register` (`useAuth.ts:71`).
- R27 → V46: `components/Home/Home.tsx:25`: con `sessionAction === "register"` muestra `` `Signed up as ${email}` ``, con el email introducido al registrarse (`useAuth.ts:61`); V44 (`auto.sh`).
- R28 → V23: `hooks/useAuth.ts:40-41`: `login` + 401 + `AuthError` → "Incorrect email or password"; V43: `Login.tsx:52` lo muestra; V19 (`auto.sh`).
- R29 → V24: `hooks/useAuth.ts:43-44`: `register` + 400 + `ValidationError` → "Could not sign up with this email and password"; V43: `Login.tsx:52`; V19 (`auto.sh`).
- R30 → V8: `services/apiClient.ts:37-46` (+ type guard `apiClient.ts:18-25`): si `fetch` rechaza, el cuerpo no es JSON o no es un `ApiResponse`, el resultado es `{ isNetworkError: true }`; V25: `hooks/useAuth.ts:36-37` → "Could not connect to the server"; V43: `Login.tsx:52`.
- R31 → V26: `hooks/useAuth.ts:46`: cualquier otro error → "Something went wrong, try again later"; V43: `Login.tsx:52`.
- R32 → V18: `.message` no aparece en `App.tsx`, `components/` ni `hooks/` (`auto.sh`); los mensajes son constantes de `useAuth.ts:29-33` y `useDummy.ts:12-14`, y el `catch` de `apiClient.ts:44-45` no expone la excepción.
- R33 → V41: `components/Login/Login.tsx:21`: `isDisabled` incluye `pendingAction !== null`; `pendingAction` vale la acción mientras dura la petición (`useAuth.ts:56,65-66`).
- R34 → V42: `components/Login/Login.tsx:30-31`: `isLoading` es `pendingAction === "login"` / `"register"` en cada botón; V16: `components/ui/Button/Button.tsx:16` pasa `isLoading` a `ChakraUiButton`, que muestra el spinner.
- R35 → V27: `hooks/useAuth.ts:79-80`: en `authorize`, `status === 401` → `setSession(null)`.
- R36 → V53: `App.tsx:15-23`: sin sesión (también tras el `setSession(null)` de `authorize`) se renderiza `Login`.
- R37 → V28: `hooks/useAuth.ts:81`: fija "Your session has expired, log in again"; `Login.tsx:33-40` no lo borra al montar y `Login.tsx:52` lo muestra; V19 (`auto.sh`).
- R38 → V47: `components/Home/Home.tsx:20,26`: botón "Check session" en la pantalla de sesión; V44 (`auto.sh`).
- R39 → V30: `hooks/useDummy.ts:26`: `authorize((sessionToken) => dummy({ dummy_string: "ping" }, sessionToken))`, y `authorize` le pasa el token de la sesión (`useAuth.ts:78`); V29 (`auto.sh`).
- R40 → V31: `hooks/useDummy.ts:30-31`: 200 + `success` → "Session is active"; V50: `Home.tsx:27` lo muestra.
- R41 → V48: `components/Home/Home.tsx:20`: `isDisabled = isChecking`; `isChecking` es `true` durante la petición (`useDummy.ts:24,36-37`).
- R42 → V49: `components/Home/Home.tsx:20`: `isLoading = isChecking`; V16: `Button.tsx:16`.
- R43 → V32: `hooks/useDummy.ts:27-29`: `isNetworkError` (V8, `apiClient.ts:37-46`) → "Could not connect to the server"; V50: `Home.tsx:27`.
- R44 → V33: `hooks/useDummy.ts:32-34`: cualquier error distinto de 401 → "Something went wrong, try again later"; V50: `Home.tsx:27`.

Pruebas manuales (`verifications/app-auth/manual.md`): P1..P12, cuyos `Cubre:`
incluyen R1..R44.
