# Bitácora histórica (append-only)

> Cada feature terminada añade aquí su resumen una sola vez, como último
> paso de su cierre (ver `AGENTS.md` §5). No edites entradas anteriores.
> Solo añades al final.

---

## 2026-10-14 — Creación del arnés
- **Agente:** humano (@irzinfante)
- **Cambios:** estructura inicial del arnés.
- **Resultado:** entorno listo. `./init.sh` correcto.

## 2026-10-14 — Feature 2: app-auth
- **Agente:** implementer (revisado por reviewer: `APPROVED`, ronda 1)
- **Spec:** `specs/app-auth/` — T1..T10 completadas. Solo se toca `app`; `api` no cambia.
- **Cambios:**
  - Configuración: `VITE_API_URL` en `app/src/vite-env.d.ts`, `app/.env.example` y `API_URL` en `app/src/constants/Environment.ts`.
  - `services/`: `apiClient.ts` (único `fetch`, `ApiResponse<T>`, `ApiResult<T>`, type guard, cabecera `Authorization: Bearer` opcional, fallos de red como `isNetworkError`), `authService.ts` (`register`, `login`) y `dummyService.ts` (`dummy`).
  - Hooks de dominio: `useAuth.ts` (dueño único del `session_token`, solo en memoria; `login`, `register`, `authorize` con cierre de sesión ante `401`, mensajes de error sin detalles técnicos) y `useDummy.ts` ("Check session").
  - `ui/`: `Button` con `isLoading` y `variant`, `TextInput` con `type`; hover `brightness(115%)` en `theme.ts`; `passwordPattern` (8-72 caracteres).
  - Pantallas: `components/Login/` (email, contraseña, "Log in" / "Sign up", spinner y deshabilitado durante la petición, mensaje de error) y `components/Home/` ("Logged in as" / "Signed up as", "Check session"). `App.tsx` elige pantalla según la sesión; se eliminan `Math.random()` y los textos simulados.
- **Desviaciones menores:** `post<Res>(path, body: object, sessionToken)` por la regla de lint `no-unnecessary-type-parameters`; `authorize` sin sesión devuelve `{ isNetworkError: true }`; el mensaje de error de `Login` pasa al flujo del contenedor; `Login` solo limpia el error cuando cambian email o contraseña (para no borrar el aviso de sesión expirada al montar).
- **Verificación:** `./init.sh` correcto; `verifications/app-auth/auto.sh` con 22 comprobaciones en `[OK]`; pruebas manuales P1..P12 en `verifications/app-auth/manual.md` (cubren R1..R44). Trazabilidad R1..R44 en `progress/app-auth/impl.md`.
- **Resultado:** feature `done`.
