# Spec Driven Development (SDD)

> Este proyecto sigue un flujo Kiro-style: requirements → design → tasks → code.
> El código no se escribe hasta que el spec está aprobado por un humano.

## Estructura

Cada feature nueva (`"sdd": true` en `feature_list.json`) tiene una carpeta
dedicada en cuanto deja `pending`:

```
specs/<feature-name>/
├── requirements.md   # QUÉ se necesita (EARS notation)
├── design.md         # CÓMO se construirá (decisiones técnicas)
└── tasks.md          # PASOS concretos a implementar
```

El `feature-name` coincide con el campo `name` de `feature_list.json`.

Una feature puede tocar `api`, `app` o ambos proyectos. El spec indica
siempre a cuál afecta cada requirement y cada task.

## Estados de una feature

| Estado         | Significado                                                    |
|----------------|----------------------------------------------------------------|
| `pending`      | Sin spec. El `spec_author` es el primero en actuar.            |
| `spec_ready`   | Spec drafted. Esperando aprobación humana. NO se toca código.  |
| `in_progress`  | Spec aprobado. `implementer` trabajando.                       |
| `done`         | `./init.sh` y comprobaciones correctas, `reviewer` aprobó.     |
| `blocked`      | Atascado. Razón en el informe de la feature en `progress/<feature-name>/`, referenciado desde `progress/current.md`. |

## La puerta de aprobación humana

El flujo automático se detiene **una vez**: cuando el `spec_author` termina
sus tres archivos, marca la feature como `spec_ready` y para. El humano
lee `specs/<feature>/` y dice "aprobado" (o pide cambios).

Solo entonces el `leader` transiciona `spec_ready → in_progress` y lanza
el `implementer`.

```
pending → [spec_author] → spec_ready → ⏸ HUMANO → in_progress → [implementer → reviewer] → done
```

## requirements.md — EARS estricto

Las requirements se redactan en **EARS** (Easy Approach to Requirements
Syntax). Cada requirement es un párrafo numerado con uno de estos cinco
patrones:

| Patrón         | Plantilla                                                   |
|----------------|-------------------------------------------------------------|
| **Ubicuo**     | `El sistema DEBE <acción>.`                                 |
| **Evento**     | `CUANDO <disparador>, el sistema DEBE <acción>.`            |
| **Estado**     | `MIENTRAS <estado>, el sistema DEBE <acción>.`              |
| **Opcional**   | `DONDE <feature opcional>, el sistema DEBE <acción>.`       |
| **No deseado** | `SI <evento no deseado> ENTONCES el sistema DEBE <acción>.` |

Reglas duras:

- Cada requirement tiene un id estable: `R1`, `R2`, ...
- Cada requirement DEBE ser verificable con al menos una comprobación
  concreta y reproducible (ver `docs/verification.md`): una petición HTTP
  contra `api` con su respuesta esperada, o, en `app`, una búsqueda en el
  código o la cita del código que lo hace verdadero (`archivo:línea`). No
  hay tests automáticos en `api` ni en `app`.
- No mezcles varios `DEBE` en un mismo requirement. Si hay más de uno, parte.
- No uses verbos blandos ("podría", "puede", "soporta"). Solo `DEBE` / `NO DEBE`.
- Los códigos HTTP y los `error.type` citados deben existir en `ErrorType`
  (`api/src/errors/mod.rs`) o el spec debe añadirlos explícitamente.

Ejemplo (feature `auth-logout`):

```markdown
## R1
CUANDO un usuario autenticado envía `POST /auth/logout`, el sistema DEBE
eliminar de la base `sessions` la sesión asociada a su `session_token`.

## R2
CUANDO la sesión se ha eliminado, el sistema DEBE responder con código 200
y `success: true`.

## R3
SI `POST /auth/logout` llega sin cabecera `Authorization: Bearer <token>`
válida ENTONCES el sistema DEBE responder con código 401 y
`error.type` = `AuthError`.

## R4
CUANDO una petición a una ruta protegida usa un `session_token` ya cerrado,
el sistema DEBE responder con código 401 y `error.type` = `AuthError`.
```

## design.md — decisiones técnicas

Captura **antes** de tocar código:

- Qué archivos se crean / modifican, en cada proyecto (`api/`, `app/`).
- Qué firmas nuevas aparecen (handlers, DTOs `*Request` / `*Response`,
  métodos de servicio y de colección, hooks, componentes, servicios de `app`).
- Qué endpoints se añaden y si son públicos o protegidos (`enforce_auth`).
- Qué variantes de `ErrorType` se reutilizan o se añaden.
- Qué variables de entorno nuevas aparecen (`config.rs` / `.env.example` /
  valor local en `init.sh`, o `vite-env.d.ts` / `.env.example` /
  `constants/Environment.ts`).
- Qué alternativa se descartó y por qué (mínimo una).

NO es ingeniería desde primeros principios — apóyate en
`docs/api-architecture.md`, `docs/api-conventions.md`,
`docs/app-architecture.md` y `docs/app-conventions.md`. El `design.md`
documenta los puntos donde tu feature roza la frontera de esas reglas.

Ejemplo de decisión para `auth-logout`: `logout` vive en `auth_api.rs`, pero
`auth_api()` monta rutas públicas; la ruta se registra en una función
`auth_protected_api()` que `main.rs` añade a `protected_endpoints`, para que
la sesión llegue como `Extension<Session>` vía `enforce_auth`. Descartado:
validar el token a mano en el handler (lo prohíbe `docs/api-architecture.md`).

## tasks.md — checklist ejecutable

Pasos discretos en orden, cada uno con checkbox. Cada task es un incremento
comprobable: el cambio de código **y** la comprobación (`V<n>`, ver
`docs/verification.md`) que demuestra los `R<n>` que cubre. Una task no está
completa hasta que su comprobación pasa. Si un requirement ya lo cumple el
código existente, la task es solo su comprobación.

Ejemplo:

```markdown
- [ ] T1 — Añadir `delete_session` en `api/src/db/sessions_collection.rs`,
  `AuthService::logout(session)` en `api/src/services/auth_service.rs`,
  `LogoutResponse` y el handler `logout` en `api/src/api/auth_api.rs`, y
  registrar `POST /auth/logout` en `auth_protected_api()` y en
  `protected_endpoints` de `main.rs`.
  Comprobación: crear `verifications/auth-logout/auto.sh` con V1 (200 y
  `success: true`) y V4 (`doc_count` de `sessions` es 0). Cubre: R1, R2.
- [ ] T2 — Sin cambios de código: `enforce_auth` ya protege la ruta y
  rechaza un token cerrado.
  Comprobación: añadir a `verifications/auth-logout/auto.sh` V2 (sin
  `Authorization` → 401 `AuthError`) y V3 (`POST /dummy` con el token
  cerrado → 401 `AuthError`). Cubre: R3, R4.
```

Si la feature toca `app`, la última task es escribir sus pruebas manuales
en `verifications/<feature-name>/manual.md` (ver `docs/verification.md`), y cubre
todos los `R<n>` de `app`.

El `implementer` marca `[x]` cada task al completarla. El `reviewer`
rechaza si queda alguna `[ ]` sin justificación documentada.

## Trazabilidad (regla dura)

- Cada comprobación (`V<n>`) de `verifications/<name>/auto.sh` debe poder
  mapearse a un `R<n>` de su spec.
- Cada `R<n>` debe tener al menos una `V<n>`: un `check` en
  `verifications/<name>/auto.sh` o, en `app`, la cita `archivo:línea` del código
  que lo hace verdadero (ver `docs/verification.md`).
- El `reviewer` comprueba esta correspondencia explícitamente y rechaza si
  falta.

El `implementer` documenta el mapa en `progress/<name>/impl.md`:

```markdown
## Trazabilidad
- R1 → V4: tras el logout, `doc_count` de `sessions` es 0 (`verifications/auth-logout/auto.sh`)
- R2 → V1: la respuesta es 200 con `success: true` (`verifications/auth-logout/auto.sh`)
- R3 → V2: `POST /auth/logout` sin `Authorization` → 401 `AuthError` (`verifications/auth-logout/auto.sh`)
- R4 → V3: `POST /dummy` con el token cerrado → 401 `AuthError` (`verifications/auth-logout/auto.sh`)
```

## Cuándo NO aplica SDD

Las features con `"sdd": false` o sin el campo `sdd` (las anteriores a
adoptar SDD, como el registro y login de `api`) NO tienen spec. SDD solo se
aplica hacia adelante.
