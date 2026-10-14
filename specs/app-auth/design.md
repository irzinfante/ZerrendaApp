# Design — app-auth

> Solo se toca **`app`**. `api` ya ofrece todo lo necesario y no cambia:
>
> - `POST /auth/register` y `POST /auth/login` (públicas, `api/src/api/mod.rs`)
>   reciben `{ email, password }` y devuelven `data.session_token`.
> - `POST /dummy` (protegida por `enforce_auth` en `api/src/main.rs`) recibe
>   `{ dummy_string }` y devuelve `data.dummy_message`; sin `Bearer` válido o
>   con la sesión caducada responde 401 `AuthError`.
> - CORS (`api/src/main.rs`) ya permite `POST`, `Content-Type` y
>   `Authorization`, y el valor local de `CORS_ALLOWED_ORIGINS` en `init.sh`
>   incluye `http://localhost:5173`.
>
> Variantes de `ErrorType` (`api/src/errors/mod.rs`): se reutilizan
> `ValidationError` (400), `AuthError` (401) e `InternalError` (500, también
> la forma pública de `DbError`). No se añade ninguna.

## Estructura resultante de `app/src`

```
App.tsx                       (mod)  useAuth + elige Login / Home
App.css                       (mod)  sin los estilos de login (pasan a Login.css)
theme.ts                      (mod)  hover más brillante (R23, R24)
vite-env.d.ts                 (mod)  VITE_API_URL
constants/Environment.ts      (mod)  API_URL
constants/Patterns.ts         (mod)  passwordPattern
services/apiClient.ts         (new)  cliente base: ApiError, ApiResponse<T>, ApiResult<T>, post
services/authService.ts       (new)  Register*/Login*, register, login
services/dummyService.ts      (new)  Dummy*, dummy
hooks/useAuth.ts              (new)  sesión (único dueño del session_token)
hooks/useDummy.ts             (new)  estado de "Check session"
hooks/ui/useButton.tsx        (mod)  isLoading, variant
hooks/ui/useTextInput.tsx     (mod)  type
components/ui/Button/Button.tsx        (mod)  isLoading, variant
components/ui/TextInput/TextInput.tsx  (mod)  type
components/Login/Login.tsx + Login.css (new)  pantalla de login
components/Home/Home.tsx + Home.css    (new)  pantalla de sesión
```

Fuera de `src/`: `app/.env.example` (mod, `VITE_API_URL= ~`) y el `app/.env`
local (no versionado), al que el implementer añade
`VITE_API_URL=http://localhost:3000` (`APP_PORT` local de `init.sh`). El
`Dockerfile` ya copia `.env`, así que no cambia.

## Variables de entorno

| Variable       | `vite-env.d.ts`                  | `.env.example`  | `constants/Environment.ts` |
|----------------|----------------------------------|-----------------|----------------------------|
| `VITE_API_URL` | `readonly VITE_API_URL: string`  | `VITE_API_URL= ~` | `export const API_URL = import.meta.env.VITE_API_URL` |

El valor va sin `/` final; las rutas de `services/` empiezan por `/`.
`init.sh` ya comprueba los tres sitios para cada `VITE_*` (bloque 5).

## Firmas nuevas

### `services/apiClient.ts` (cliente base, único `fetch`)

```ts
export interface ApiError {
  type: "ValidationError" | "AuthError" | "InternalError"
  message: string
}

export interface ApiResponse<T> {
  success: boolean
  error: ApiError | null
  data: T | null
}

export type ApiResult<T> =
  | { isNetworkError: false, status: number, response: ApiResponse<T> }
  | { isNetworkError: true }

export const post = async <Req, Res>(
  path: string,
  body: Req,
  sessionToken: string | null = null
): Promise<ApiResult<Res>>
```

- URL: `` `${API_URL}${path}` `` (R5). Cabeceras: `Content-Type:
  application/json` y, si hay `sessionToken`,
  `` Authorization: `Bearer ${sessionToken}` `` (R13).
- Lee siempre el cuerpo antes de decidir (los errores llegan con código ≠ 200
  y el mismo formato). `const json: unknown = await response.json()` y un type
  guard `isApiResponse(json): json is ApiResponse<Res>` que comprueba la forma
  (`success` booleano, `error` objeto o `null`, clave `data`), sin `as`.
- `fetch` rechazado, cuerpo que no es JSON o que no pasa el type guard →
  `{ isNetworkError: true }` (R30, R43). El `catch` devuelve ese resultado:
  no se traga el error.
- El token es opaco: el cliente no lo decodifica ni lo guarda (R12).

### `services/authService.ts`

```ts
export interface RegisterRequest { email: string, password: string }
export interface RegisterResponse { session_token: string }
export interface LoginRequest { email: string, password: string }
export interface LoginResponse { session_token: string }

export const register = (request: RegisterRequest): Promise<ApiResult<RegisterResponse>>
export const login = (request: LoginRequest): Promise<ApiResult<LoginResponse>>
```

Rutas `"/auth/register"` y `"/auth/login"`, definidas solo aquí. `ApiResult<T>`
lleva la respuesta como `ApiResponse<T>` (R9).

### `services/dummyService.ts`

```ts
export interface DummyRequest { dummy_string: string }
export interface DummyResponse { dummy_message: string }

export const dummy = (request: DummyRequest, sessionToken: string): Promise<ApiResult<DummyResponse>>
```

Ruta `"/dummy"`. Los campos coinciden con `DummyRequest` / `DummyResponse` de
`api/src/api/dummy_api.rs` (R11).

### `hooks/useAuth.ts` (único dueño del `session_token`)

```ts
export type AuthAction = "login" | "register"

export type Authorize = <T>(
  request: (sessionToken: string) => Promise<ApiResult<T>>
) => Promise<ApiResult<T>>

type UseAuth = () => {
  isLoggedIn: boolean
  email: string | null
  sessionAction: AuthAction | null     // con qué se abrió la sesión (R19, R27)
  pendingAction: AuthAction | null     // petición en curso (R33, R34)
  errorMessage: string | null          // mensaje para la pantalla de login
  login: (email: string, password: string) => Promise<void>
  register: (email: string, password: string) => Promise<void>
  authorize: Authorize
  clearError: () => void
}
```

- Estado: `useState<Session | null>(null)` con
  `interface Session { sessionToken: string, email: string, action: AuthAction }`,
  más `pendingAction` y `errorMessage`. Ningún otro módulo guarda el token ni
  lo recibe salvo dentro de `authorize` (R12). Se guarda solo en memoria.
- `login` / `register`: limpian `errorMessage`, ponen `pendingAction`, llaman
  al servicio y al terminar (`finally`) vuelven `pendingAction` a `null`. Con
  `status === 200 && response.success && response.data` guardan la sesión con
  el email introducido (R17, R26). Si no, `errorMessage` según:

  | Resultado                                         | Mensaje                                              | R   |
  |---------------------------------------------------|------------------------------------------------------|-----|
  | `isNetworkError`                                  | "Could not connect to the server"                    | R30 |
  | `login`, 401 y `error.type === "AuthError"`       | "Incorrect email or password"                        | R28 |
  | `register`, 400 y `error.type === "ValidationError"` | "Could not sign up with this email and password"  | R29 |
  | cualquier otro error                              | "Something went wrong, try again later"              | R31 |

  Los mensajes son constantes del hook; nunca se usa `error.message` (R32).
- `authorize(request)`: si no hay sesión no llama a nada; si la hay, ejecuta
  `request(session.sessionToken)`. Si el resultado es `status === 401`, pone la
  sesión a `null` (R35) y `errorMessage` a
  "Your session has expired, log in again" (R37). Devuelve el resultado al
  llamante. Toda petición a una ruta protegida pasa por `authorize`, así el
  tratamiento del 401 está en un solo sitio.

### `hooks/useDummy.ts`

```ts
type UseDummy = (authorize: Authorize) => {
  isChecking: boolean
  checkMessage: string | null
  isCheckError: boolean
  checkSession: () => Promise<void>
}
```

`checkSession` llama a `authorize((token) => dummy({ dummy_string: "ping" }, token))`
(R39). 200 + `success` → "Session is active" (R40); `isNetworkError` →
"Could not connect to the server" (R43); 401 → nada (la sesión ya se cerró y
`Home` se desmonta); otro error → "Something went wrong, try again later"
(R44). `dummy_message` no se muestra (contiene ids internos).

### Componentes `ui/` y sus hooks

```ts
type ButtonVariant = "primary" | "secondary"   // variantes ya definidas en theme.ts

interface ButtonProps {
  text: string
  isDisabled: boolean
  isLoading: boolean
  variant: ButtonVariant
  handleClick: React.MouseEventHandler<HTMLButtonElement>
}

type UseButton = (
  text: string,
  isDisabled: boolean,
  handleClick: React.MouseEventHandler,
  isLoading?: boolean,          // por defecto false
  variant?: ButtonVariant       // por defecto "primary"
) => { Button: React.JSX.Element }

interface TextInputProps { ...; type: "text" | "password" }

type UseTextInput = (
  label: string,
  placeholder: string,
  validationRegex: RegExp,
  type?: "text" | "password"    // por defecto "text"
) => { TextInput: React.JSX.Element, value: string, isInvalid: boolean }
```

`Button` pasa `isLoading` a `ChakraUiButton`, que muestra el spinner (R34,
R42). Siguen sin estado ni red (regla de `init.sh`).

### `theme.ts`

```ts
baseStyle: {
  ...
  _hover: { filter: "brightness(115%)" },                  // R23
  _disabled: { bg: "var(--ash-gray)", filter: "brightness(85%)", opacity: 1, cursor: "not-allowed" },
},
variants: {
  primary:   { bg: "var(--striped-blue)", _hover: { _disabled: { bg: "var(--ash-gray)", filter: "brightness(85%)" } } },  // R24
  secondary: { bg: "var(--sage-green)",   _hover: { _disabled: { bg: "var(--ash-gray)", filter: "brightness(85%)" } } },  // R24
}
```

Hoy `_hover` oscurece (`brightness(85%)`); pasa a aclarar solo si el botón
está habilitado.

### `constants/Patterns.ts`

```ts
export const passwordPattern = /^.{8,72}$/u
```

Refleja `is_valid_password` de `api/src/helpers/validation.rs` (mínimo 8
caracteres). El máximo de `api` es 72 **bytes**; el patrón cuenta caracteres,
así que con caracteres multibyte la app puede dejar pasar una contraseña que
`api` rechaza con 400, y se muestra el mensaje de R29. La API es la autoridad.

### Pantallas

```ts
interface LoginProps {
  pendingAction: AuthAction | null
  errorMessage: string | null
  login: (email: string, password: string) => Promise<void>
  register: (email: string, password: string) => Promise<void>
  clearError: () => void
}

interface HomeProps {
  email: string
  sessionAction: AuthAction
  authorize: Authorize
}
```

- `components/Login/Login.tsx`: título "Log in to continue", `EmailInput`
  (`useTextInput("Email", "name@example.org", emailPattern)`),
  `PasswordInput` (`useTextInput("Password", "", passwordPattern, "password")`),
  botón "Log in" (`primary`) y "Sign up" (`secondary`). Ambos con
  `isDisabled = emailValue === "" || isEmailInvalid || passwordValue === "" || isPasswordInvalid || pendingAction !== null`
  (R21, R22, R33) e `isLoading = pendingAction === "<acción>"` (R34).
  Manejadores `handleLogin = () => { void login(emailValue, passwordValue) }`
  (y `handleRegister`), porque `no-misused-promises` impide pasar una función
  async como `onClick`. `errorMessage` se muestra con la clase
  `login-error`. Un `useEffect` sobre `emailValue` / `passwordValue` llama a
  `clearError` (como hoy con el email). Estilos de `#login-container` movidos
  de `App.css` a `Login.css`.
- `components/Home/Home.tsx`: texto "Logged in as <email>" o
  "Signed up as <email>" según `sessionAction` (R19, R27); botón
  "Check session" (`useButton`, `isDisabled = isChecking`, `isLoading =
  isChecking`) (R38, R41, R42); `checkMessage` con clase `home-success` o
  `home-error` según `isCheckError`. Estilos en `Home.css` con `var(--…)`.
- `App.tsx`: llama a `useAuth()` y renderiza `<Home …/>` si `isLoggedIn` y
  `email` / `sessionAction` no son `null`, o `<Login …/>` en caso contrario
  (R14, R18, R36). Mantiene logo y `Footer`. Desaparecen `Math.random()` y los
  textos simulados (R20).

El email mostrado es el que escribió el usuario. `api` lo guarda normalizado
(recortado y en minúsculas, `normalize_email`), pero `RegisterResponse` solo
devuelve el token y el token es opaco, así que la app no tiene otro dato del
usuario.

## Reglas de `docs/app-architecture.md` que roza la feature

- **Hooks de dominio que dependen de otro hook.** `useDummy` no importa
  `useAuth`: recibe `authorize` por parámetro desde `Home`, que lo recibe de
  `App`. Así el token no sale de `useAuth` y `App.tsx` sigue siendo el dueño
  del estado de sesión.
- **Chakra en pantallas.** `Login` y `Home` solo usan `ui/` (`TextInput`,
  `Button` vía sus hooks); los mensajes son `<span>` con CSS propio.
- **Sin dependencias nuevas.** El spinner es el `isLoading` de Chakra
  `Button`, ya en `dependencies`.

## Alternativas descartadas

1. **Guardar el token en el cliente base (variable de módulo) o en
   `localStorage`.** Contradice el acceptance 3 (`useAuth.ts` único sitio
   del token). `localStorage` además lo expone a cualquier script de la
   página y el acceptance no pide sobrevivir a una recarga: al recargar, la
   sesión se pierde y se vuelve al login.
2. **Pasar `sessionToken` y un callback `onUnauthorized` a cada hook que use
   rutas protegidas.** Cada hook tendría que acordarse de tratar el 401 y el
   token circularía por varias capas. `authorize` centraliza ambas cosas en
   `useAuth`.
3. **Tratar el 401 en el cliente base** (por ejemplo, con un callback global
   registrado al arrancar). El cliente base no sabe si la ruta es protegida
   (`/auth/login` también responde 401, con credenciales incorrectas, y eso
   no debe "cerrar la sesión" sino mostrar R28) y necesitaría estado global.
4. **No llamar a ninguna ruta protegida en esta feature.** El tratamiento del
   401 (acceptance 11) quedaría como código muerto: no se podría provocar ni
   en las pruebas manuales. Se usa `POST /dummy`, la única ruta protegida,
   con el botón mínimo "Check session" (R38-R44).
5. **Un único botón con modo "login / sign up" alternable.** Añade un control
   y estado más; dos botones con las variantes `primary` / `secondary` que ya
   define `theme.ts` resuelven lo mismo.

## Puntos a confirmar en la aprobación

- El acceptance 2 cita `ApiResponse<T>` en `authService.ts`. Se define en el
  cliente base (lo comparten `authService.ts` y `dummyService.ts`) y
  `authService.ts` lo usa en el tipo de sus funciones a través de
  `ApiResult<T>` (R6, R9).
- La pantalla de sesión con "Check session" (R38-R44) no está en el
  `acceptance` literal; se añade para que el acceptance 11 sea alcanzable y
  verificable. Para provocar el 401 a mano se borra la sesión en CouchDB
  (ver `tasks.md`, T10).
- No hay "Log out": `api` no tiene endpoint de logout y el acceptance no lo
  pide.
