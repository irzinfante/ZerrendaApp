# Review — feature app-auth

**Veredicto:** APPROVED

> Ronda 1 de 3. `./init.sh` → `[OK] Entorno listo` (exit 0: build, cargo
> build/fmt/clippy, `npm run lint` y las reglas de arquitectura de `app`,
> incluida `VITE_API_URL`). `./verifications/app-auth/auto.sh` → 22
> comprobaciones en `[OK]`, exit 0 (sin `start_env`, porque no se toca `api`).
> He releído todas las citas `archivo:línea` de `progress/app-auth/impl.md`
> contra el código actual y todas apuntan a las líneas correctas.

## Trazabilidad requirements ↔ comprobaciones
- R1: [x] V1 (`verifications/app-auth/auto.sh:9-10`)
- R2: [x] V2 (`auto.sh:11-12`)
- R3: [x] V3 (`auto.sh:13-14`)
- R4: [x] V4 (`auto.sh:18-19`); también init.sh bloque 5
- R5: [x] V5 (`auto.sh:20-22`); `app/src/services/apiClient.ts:38`
- R6: [x] V6 (`auto.sh:23-24`); `apiClient.ts:3-12`
- R7: [x] V9 (`auto.sh:34-36`); `authService.ts:3-10`
- R8: [x] V9 (`auto.sh:34-36`); `authService.ts:12-19`
- R9: [x] V10 (`auto.sh:37-38`); `authService.ts:21-25`, `apiClient.ts:14-16`
- R10: [x] V11 (`auto.sh:39-45`), comprobado contra `api/src/api/mod.rs:12-13,18`
- R11: [x] V12 (`auto.sh:46-54`), compara con los `pub struct` de `api/src/api/auth_api.rs:8-36` y `dummy_api.rs:7-13`
- R12: [x] V17 (`auto.sh:69-71`) + V20 `app/src/hooks/useAuth.ts:50` (único estado con el token, en memoria); `apiClient.ts:27-34` y `dummyService.ts:11-12` solo lo reciben como parámetro
- R13: [x] V7 (`auto.sh:25-26`) + `apiClient.ts:32-34` (cabecera solo si `sessionToken !== null`)
- R14: [x] V35 (`auto.sh:91-92`) + V36 `components/Login/Login.tsx:18-19,30-31,45-50` + V53 `App.tsx:15-23`
- R15: [x] V34 (`auto.sh:89-90`); `Login.tsx:19`, `components/ui/TextInput/TextInput.tsx:19`
- R16: [x] V37 `Login.tsx:23-25,30` → `useAuth.ts:70,58-59` → `authService.ts:24-25`
- R17: [x] V21 `useAuth.ts:60-61`
- R18: [x] V52 `App.tsx:15-16` (con `useAuth.ts:91`)
- R19: [x] V45 `components/Home/Home.tsx:25`; V44 (`auto.sh:96-97`)
- R20: [x] V51 (`auto.sh:100-101`)
- R21: [x] V39 `Login.tsx:21,30-31`
- R22: [x] V40 `Login.tsx:19,21`; V15 (`auto.sh:64-65`)
- R23: [x] V13 (`auto.sh:58-59`); `app/src/theme.ts:17`
- R24: [x] V14 (`auto.sh:60-63`); `theme.ts:31,35` (necesario porque `_hover` base y `_disabled` tienen la misma especificidad; el `_hover._disabled` de cada variante lo resuelve)
- R25: [x] V38 `Login.tsx:26-28,31` → `useAuth.ts:71,58-59` → `authService.ts:21-22`
- R26: [x] V22 `useAuth.ts:60-61` (misma rama para `register`)
- R27: [x] V46 `Home.tsx:25`, email guardado en `useAuth.ts:61`; V44
- R28: [x] V23 `useAuth.ts:40-41`; V43 `Login.tsx:52`; V19 (`auto.sh:74-80`)
- R29: [x] V24 `useAuth.ts:43-44`; V43; V19
- R30: [x] V8 `apiClient.ts:18-25,37-46`; V25 `useAuth.ts:36-37`; V43
- R31: [x] V26 `useAuth.ts:46`; V43
- R32: [x] V18 (`auto.sh:72-73`); mensajes constantes en `useAuth.ts:29-33` y `useDummy.ts:12-14`; `catch` sin exponer la excepción `apiClient.ts:44-45`
- R33: [x] V41 `Login.tsx:21`; `useAuth.ts:56,65-67`
- R34: [x] V42 `Login.tsx:30-31`; V16 `components/ui/Button/Button.tsx:16`
- R35: [x] V27 `useAuth.ts:79-80`
- R36: [x] V53 `App.tsx:15-23`
- R37: [x] V28 `useAuth.ts:81`; `Login.tsx:33-40` no lo borra al montar; `Login.tsx:52`; V19
- R38: [x] V47 `Home.tsx:20,26`; V44
- R39: [x] V30 `hooks/useDummy.ts:26` + `useAuth.ts:78`; V29 (`auto.sh:84-85`)
- R40: [x] V31 `useDummy.ts:30-31`; V50 `Home.tsx:27`
- R41: [x] V48 `Home.tsx:20`; `useDummy.ts:24,36-37`
- R42: [x] V49 `Home.tsx:20`; V16 `Button.tsx:16`
- R43: [x] V32 `useDummy.ts:27-29` (+ V8); V50
- R44: [x] V33 `useDummy.ts:32-34`; V50

## Pruebas manuales (si toca `app`)
`verifications/app-auth/manual.md` existe, con Preparación y P1..P12. Los
`Cubre:` incluyen R1..R44 sin huecos (comprobado programáticamente). Pasos y
resultados esperados son concretos y visibles (textos exactos, colores,
spinner, cursor), e incluyen los caminos de error y cómo provocarlos (api
parada, `docker compose stop db`, borrado de sesiones en Fauxton con las
credenciales reales de `compose.yaml:12-13`).
- R1, R2, R3, R5: [x] P4, P7
- R4, R6, R7, R9, R10, R11: [x] P4
- R8: [x] P6
- R12: [x] P11
- R13: [x] P9
- R14, R15: [x] P1
- R16, R17, R19, R28: [x] P6
- R18: [x] P4, P6
- R20: [x] P12
- R21, R22: [x] P2
- R23, R24: [x] P3
- R25, R26, R27: [x] P4
- R29: [x] P5
- R30: [x] P7
- R31: [x] P8
- R32: [x] P5, P6, P7, P8
- R33, R34: [x] P4, P6
- R35, R36, R37: [x] P11
- R38, R39, R40, R41, R42: [x] P9
- R43, R44: [x] P10

## Tasks completas
- T1: [x]
- T2: [x]
- T3: [x]
- T4: [x]
- T5: [x]
- T6: [x]
- T7: [x]
- T8: [x]
- T9: [x]
- T10: [x]

## Desviaciones de `design.md` (impl.md §"Desviaciones")
1. `post<Res>(path, body: object, …)` en lugar de `post<Req, Res>`: aceptable.
   `no-unnecessary-type-parameters` de `strictTypeChecked` lo impone y los
   cuerpos siguen tipados en `authService.ts:21,24` y `dummyService.ts:11`. No
   afecta a R5, R6, R9 ni R13.
2. `ButtonVariant` / `TextInputType` exportados desde los componentes `ui/` y
   reutilizados en sus hooks: aceptable. Evita duplicar el tipo y respeta la
   dirección `hooks/ui → components/ui`.
3. `useRef` en `Login.tsx:33-40` para no llamar a `clearError` al montar:
   aceptable y necesario. Sin él, el mensaje de R37 se borraría al remontar
   `Login` tras el 401. `clearError` es estable (`useAuth.ts:86-88`), así que
   el efecto no se dispara en bucle. El comentario explica un porqué no obvio.
4. `authorize` sin sesión devuelve `{ isNetworkError: true }`
   (`useAuth.ts:75-77`): aceptable. La firma `Authorize` obliga a devolver un
   `ApiResult<T>` y la rama no se alcanza (`Home` solo se monta con sesión,
   `App.tsx:15-16`).
5. `.login-error` dentro del flujo del contenedor (`Login.css:27-31`):
   aceptable. Es un cambio de presentación, usa `var(--terracotta-red)` y no
   cambia ningún R.
6. V12 más estricta que en `tasks.md`: aceptable y mejor. Compara el conjunto
   de campos de cada tipo con el `pub struct` del mismo nombre.

## Arquitectura y convenciones (`docs/app-architecture.md`, `docs/app-conventions.md`)
- Capas: `services/apiClient.ts` (único `fetch`), `authService.ts` /
  `dummyService.ts` (rutas definidas una sola vez), hooks de dominio
  `useAuth.ts` / `useDummy.ts`, pantallas `components/Login/` y
  `components/Home/` sin imports de Chakra, `ui/` sin estado ni red. Las
  dependencias van hacia abajo. `useDummy` solo importa el tipo `Authorize`,
  no el hook (decisión de `design.md`).
- Contrato tipado: `ApiResponse<T>`, `*Request` / `*Response` en
  `snake_case`. El cliente base lee el cuerpo antes de decidir
  (`apiClient.ts:39-43`), y la UI decide por `status` / `error.type`
  (`useAuth.ts:39-46`), nunca por `message`.
- Sesión: token solo en memoria en `useAuth`, Bearer en el cliente base y
  401 centralizado en `authorize`.
- Configuración: `VITE_API_URL` en los tres sitios; `app/.env` local con
  `http://localhost:3000` e ignorado por git (`.gitignore:16`).
- Convenciones: comillas dobles, sin punto y coma, `useState` con el genérico
  explícito, booleanos `is*`, manejadores `handle*`, `export default` al
  final, textos en inglés y sin `console.log` ni TODOs. No hay dependencias
  nuevas en `app/package.json` ni en `api/Cargo.toml`.

## Checkpoints (sin C5)
- C1: [x]
- C2: [x]
- C3: [x]
- C4: [x]
- C6: [x]

## Observaciones (no bloqueantes)
- `apiClient.ts:36`: el comentario describe sobre todo el *qué*. Se acepta
  porque justifica una decisión no obvia (un cuerpo que no es `ApiResponse`
  se trata como fallo de red, R30/R43).
- `specs/app-auth/requirements.md`: varios R usan como sujeto una parte del
  sistema ("El cliente base DEBE", "`authService.ts` DEBE", "la pantalla de
  sesión DEBE") en lugar de "el sistema DEBE". Mantienen los patrones EARS
  Ubicuo/Evento/Estado/No deseado, con un solo `DEBE` por R, y el spec tiene
  la aprobación humana. Para futuros specs, conviene usar el sujeto literal
  de `docs/specs.md`.

## Cambios requeridos (si aplica)
Ninguno.
