# Requirements — app-auth

> Feature 2 de `feature_list.json`: registro y login de usuario en `app`.
> Afecta solo a **`app`**. `api` no cambia: se usan `POST /auth/register`,
> `POST /auth/login` (públicas) y `POST /dummy` (protegida, `enforce_auth`)
> tal como están.
>
> Los textos entre comillas son los que ve el usuario (en inglés, ver
> `docs/app-conventions.md`). "Pantalla de login" es el formulario de email y
> contraseña; "pantalla de sesión" es lo que se muestra con una sesión abierta
> (ver `design.md`).

## Configuración

### R1
El sistema DEBE declarar `readonly VITE_API_URL: string` en `ImportMetaEnv`
de `app/src/vite-env.d.ts`.

### R2
El sistema DEBE documentar la variable `VITE_API_URL` en `app/.env.example`.

### R3
El sistema DEBE exportar desde `app/src/constants/Environment.ts` la constante
`API_URL` con el valor de `import.meta.env.VITE_API_URL`.

## Cliente base y contrato con `api`

### R4
El sistema DEBE llamar a `fetch` únicamente desde el cliente base
`app/src/services/apiClient.ts`.

### R5
El cliente base DEBE construir la URL de cada petición como `API_URL` seguida
de la ruta del endpoint.

### R6
El cliente base DEBE declarar el tipo `ApiResponse<T>` con los campos
`success: boolean`, `error: ApiError | null` y `data: T | null`, donde
`ApiError` tiene `type` (`"ValidationError" | "AuthError" | "InternalError"`)
y `message: string`.

### R7
`app/src/services/authService.ts` DEBE declarar `RegisterRequest` con los
campos `email: string` y `password: string`, y `RegisterResponse` con el campo
`session_token: string`.

### R8
`app/src/services/authService.ts` DEBE declarar `LoginRequest` con los campos
`email: string` y `password: string`, y `LoginResponse` con el campo
`session_token: string`.

### R9
`app/src/services/authService.ts` DEBE tipar la respuesta de sus funciones
`register` y `login` como `ApiResponse<RegisterResponse>` y
`ApiResponse<LoginResponse>` respectivamente.

### R10
Cada ruta que usa `app/src/services/` DEBE estar registrada con el método
`POST` en `api/src/api/mod.rs`.

### R11
Los campos de cada tipo `*Request` / `*Response` de `app/src/services/` DEBEN
coincidir en nombre con los del DTO homónimo de `api/src/api/`.

## Sesión

### R12
`app/src/hooks/useAuth.ts` DEBE ser el único módulo de `app` que guarda el
`session_token` (ni otro hook, ni componente, ni `localStorage`,
`sessionStorage` o cookies).

### R13
CUANDO el cliente base envía una petición con un `session_token`, el sistema
DEBE incluir la cabecera `Authorization: Bearer <session_token>`.

## Login

### R14
MIENTRAS no hay sesión, el sistema DEBE mostrar la pantalla de login con un
campo "Email", un campo "Password" y los botones "Log in" y "Sign up".

### R15
El campo "Password" DEBE ocultar los caracteres introducidos (`type="password"`).

### R16
CUANDO el usuario pulsa "Log in", el sistema DEBE enviar `POST /auth/login`
con el email y la contraseña introducidos.

### R17
CUANDO `POST /auth/login` responde 200 con `success: true`, el sistema DEBE
guardar `data.session_token` en `useAuth`.

### R18
MIENTRAS `useAuth` guarda un `session_token`, el sistema DEBE mostrar la
pantalla de sesión en lugar de la pantalla de login.

### R19
CUANDO la sesión se ha abierto con "Log in", la pantalla de sesión DEBE
mostrar el texto "Logged in as <email>".

### R20
El sistema NO DEBE simular respuestas de `api` (sin `Math.random()` ni
resultados inventados en `app/src/`).

## Validación del formulario

### R21
MIENTRAS el campo "Email" está vacío o no cumple `emailPattern`, el sistema
DEBE deshabilitar los botones "Log in" y "Sign up".

### R22
MIENTRAS el campo "Password" está vacío o no cumple `passwordPattern` (de 8 a
72 caracteres), el sistema DEBE deshabilitar los botones "Log in" y "Sign up".

## Hover de los botones

### R23
CUANDO el puntero pasa por encima de un botón habilitado, el sistema DEBE
mostrarlo más brillante (`filter: brightness(115%)`).

### R24
CUANDO el puntero pasa por encima de un botón deshabilitado, el sistema DEBE
mantener su aspecto de deshabilitado (fondo `var(--ash-gray)` y
`filter: brightness(85%)`).

## Registro

### R25
CUANDO el usuario pulsa "Sign up", el sistema DEBE enviar
`POST /auth/register` con el email y la contraseña introducidos.

### R26
CUANDO `POST /auth/register` responde 200 con `success: true`, el sistema DEBE
guardar `data.session_token` en `useAuth`.

### R27
CUANDO la sesión se ha abierto con "Sign up", la pantalla de sesión DEBE
mostrar el texto "Signed up as <email>", con el email registrado.

## Mensajes de error

### R28
SI `POST /auth/login` responde 401 con `error.type` = `AuthError` ENTONCES el
sistema DEBE mostrar en la pantalla de login el mensaje
"Incorrect email or password".

### R29
SI `POST /auth/register` responde 400 con `error.type` = `ValidationError`
ENTONCES el sistema DEBE mostrar en la pantalla de login el mensaje
"Could not sign up with this email and password".

### R30
SI una petición de login o registro no obtiene respuesta de `api` (`fetch`
rechaza) o la respuesta no tiene el formato `ApiResponse` ENTONCES el sistema
DEBE mostrar en la pantalla de login el mensaje
"Could not connect to the server".

### R31
SI una petición de login o registro recibe una respuesta de error no cubierta
por R28 ni R29 ENTONCES el sistema DEBE mostrar en la pantalla de login el
mensaje "Something went wrong, try again later".

### R32
El sistema NO DEBE mostrar al usuario el `error.message` de `api` ni el texto
de una excepción.

## Petición en curso

### R33
MIENTRAS una petición de login o registro está en curso, el sistema DEBE
deshabilitar los botones "Log in" y "Sign up".

### R34
MIENTRAS una petición de login o registro está en curso, el sistema DEBE
mostrar un spinner en el botón que la ha lanzado.

## 401 en una ruta protegida

### R35
CUANDO una petición a una ruta protegida responde 401, el sistema DEBE
eliminar el `session_token` de `useAuth`.

### R36
CUANDO la sesión se elimina por un 401, el sistema DEBE mostrar la pantalla de
login.

### R37
CUANDO la sesión se elimina por un 401, la pantalla de login DEBE mostrar el
mensaje "Your session has expired, log in again".

## Comprobación de sesión (ruta protegida `POST /dummy`)

> Soporte de R35-R37: es la única ruta protegida de `api` y, sin una pantalla
> que la llame, el 401 no se puede provocar ni probar (ver `design.md`).

### R38
MIENTRAS hay sesión, la pantalla de sesión DEBE mostrar un botón
"Check session".

### R39
CUANDO el usuario pulsa "Check session", el sistema DEBE enviar `POST /dummy`
con el `session_token` de `useAuth`.

### R40
CUANDO `POST /dummy` responde 200 con `success: true`, el sistema DEBE mostrar
en la pantalla de sesión el texto "Session is active".

### R41
MIENTRAS `POST /dummy` está en curso, el sistema DEBE deshabilitar el botón
"Check session".

### R42
MIENTRAS `POST /dummy` está en curso, el sistema DEBE mostrar un spinner en el
botón "Check session".

### R43
SI `POST /dummy` no obtiene respuesta de `api` o la respuesta no tiene el
formato `ApiResponse` ENTONCES el sistema DEBE mostrar en la pantalla de
sesión el mensaje "Could not connect to the server".

### R44
SI `POST /dummy` responde con un error distinto de 401 ENTONCES el sistema
DEBE mostrar en la pantalla de sesión el mensaje
"Something went wrong, try again later".

## Cobertura de los `acceptance` de `feature_list.json`

| # | Acceptance                                                                 | Requirements          |
|---|----------------------------------------------------------------------------|-----------------------|
| 1 | URL de api en `VITE_*`, `vite-env.d.ts`, `.env.example`, `Environment.ts`  | R1, R2, R3, R5        |
| 2 | Cliente base con `fetch` y `authService.ts` con sus tipos                  | R4, R6, R7, R8, R9, R10, R11 |
| 3 | `useAuth.ts` único sitio del token; Bearer en el cliente base              | R12, R13              |
| 4 | Email y contraseña; login real contra api, sin simulaciones                | R14, R15, R16, R17, R18, R19, R20 |
| 5 | Botón deshabilitado con email o contraseña no válidos                      | R21, R22              |
| 6 | Botón más brillante al hacer hover cuando está habilitado                 | R23, R24              |
| 7 | Registro de usuario nuevo contra api                                       | R25, R26              |
| 8 | Mensajes de error sin detalles técnicos                                    | R28, R29, R30, R31, R32 |
| 9 | Botón deshabilitado y elemento gráfico mientras hay petición en curso      | R33, R34, R41, R42    |
| 10 | Registro exitoso muestra el usuario registrado                            | R27                   |
| 11 | `401` en ruta protegida cierra la sesión y vuelve al login                | R35, R36, R37, R38, R39, R40, R43, R44 |
