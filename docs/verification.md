# Verificación — Cómo demostrar que el trabajo funciona

> Regla de oro: **el agente no dice "funciona", lo demuestra**.
> Toda feature termina con evidencia ejecutable, no con afirmaciones.

`api` y `app` no tienen tests automáticos. Su papel lo cumplen `./init.sh`
(compilación, formato, lint y reglas de arquitectura) y, por feature, la
carpeta `verifications/<feature-name>/`:

```
verifications/<feature-name>/
├── auto.sh        # comprobaciones automáticas: contra la API real (nivel 2) y búsquedas en el código de app (nivel 3)
└── manual.md      # pruebas manuales de app, las ejecuta el humano
```

## Niveles de verificación

### Nivel 1 — `./init.sh` (obligatorio)

Además del entorno y del estado del arnés, comprueba:

- Compilación: `npm run build` en `app` (antes, `npm install` si las
  dependencias faltan o han cambiado) y `cargo build` en `api`.
- Formato y lint: `cargo fmt --check` y `cargo clippy -- -D warnings` en
  `api`, `npm run lint` en `app` (lo mismo que el workflow `Lint`).
- Las reglas de `docs/app-architecture.md` que se detectan buscando en el
  código: `fetch` solo en `services/`, `import.meta.env` solo en
  `constants/Environment.ts`, `components/ui/` sin estado ni red, y cada
  `VITE_*` declarada también en `.env.example` y `constants/Environment.ts`.
  Si una feature añade una regla detectable de la misma forma, se añade su
  comprobación a `init.sh`.

Comando:
```bash
./init.sh           # debe terminar con [OK] Entorno listo
```

`./init.sh --quick` omite la compilación, el formato y el lint. Lo usan los
hooks del leader y del spec_author, que no tocan código.

### Nivel 2 — Comprobación de la API en ejecución (obligatorio si se toca `api`)

Cada endpoint nuevo o modificado se comprueba con peticiones HTTP reales a la
API, en `verifications/<feature-name>/auto.sh`. Nada de simular respuestas.

El script carga `verifications/lib.sh` y llama a `start_env`, que arranca una
CouchDB vacía y desechable (sin volumen, en `$DB`) y la API sobre ella (en
`$API`), en puertos distintos a los de `compose.yaml`. Al terminar el script,
pase lo que pase, para la API y elimina el contenedor con todos sus datos.
Por eso el script crea los datos que necesita y no tiene que borrarlos. El
resto de la configuración de la API (`AUTH_PRIVATE_KEY`,
`CORS_ALLOWED_ORIGINS`) sale de `api/.env`, que `./init.sh` crea a partir de
`api/.env.example` si no existe.

Cada comprobación (`V<n>`) es un `check <V> <descripción> <esperado> <obtenido>`
sobre el código HTTP, `success`, y `data` o `error.type`, y sobre los efectos
en CouchDB cuando el requirement los menciona. El script imprime `[OK]` o
`[FAIL]` por comprobación y sale con código ≠ 0 si alguna falla.

Ejemplo, `verifications/auth-logout/auto.sh` para la feature `auth-logout` de
`docs/specs.md`:

```bash
#!/usr/bin/env bash
# Checks of auth-logout (specs/auth-logout/). See docs/verification.md.
source "$(dirname "$0")/../lib.sh"
start_env

TOKEN=$(curl -s -X POST "$API/auth/register" -H "Content-Type: application/json" \
  -d '{"email":"logout@example.org","password":"password123"}' | jq -r .data.session_token)

RES=$(curl -s -w "\n%{http_code}" -X POST "$API/auth/logout" -H "Authorization: Bearer $TOKEN")
check V1 "logout with valid token → 200 success" "200 true" \
  "$(tail -n1 <<< "$RES") $(head -n1 <<< "$RES" | jq .success)"

RES=$(curl -s -w "\n%{http_code}" -X POST "$API/auth/logout")
check V2 "logout without Authorization → 401 AuthError" "401 AuthError" \
  "$(tail -n1 <<< "$RES") $(head -n1 <<< "$RES" | jq -r .error.type)"

RES=$(curl -s -w "\n%{http_code}" -X POST "$API/dummy" -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" -d '{"dummy_string":"hola"}')
check V3 "protected route with closed token → 401 AuthError" "401 AuthError" \
  "$(tail -n1 <<< "$RES") $(head -n1 <<< "$RES" | jq -r .error.type)"

check V4 "session removed from CouchDB" 0 "$(curl -s "$DB/sessions" | jq .doc_count)"
```

Comando (al crear el script, hazlo ejecutable con
`chmod +x verifications/<feature-name>/auto.sh`):
```bash
./verifications/<feature-name>/auto.sh
```

### Nivel 3 — Comprobación del código de `app` (obligatorio si se toca `app`)

Los agentes no ejecutan `app` para verificar: no hay navegador, capturas ni
comprobaciones manuales (esas las hace el humano al final, ver "Pruebas
manuales de `app`"). Además del nivel 1 (con `tsc` en modo `strict` y las
reglas `strictTypeChecked` de `typescript-eslint`), cada `R<n>` de `app` se
demuestra con una `V<n>`:

- Si se puede comprobar buscando en el código, como un `check` en
  `verifications/<feature-name>/auto.sh` (sin `start_env`, que solo hace falta
  para la API).
- Si no, citando el código que lo hace verdadero: `archivo:línea` y una frase
  que explique por qué. Por ejemplo, en la pantalla de login actual:
  - V1 — Con un email que no cumple `emailPattern`, el botón "Log in" está
    deshabilitado → `app/src/App.tsx:23-24`: `useTextInput` valida con
    `emailPattern` y devuelve `isInvalid`, y `useButton` recibe
    `emailValue === "" || isEmailInvalid` como estado deshabilitado.

Si la feature habla con `api`, una `V<n>` comprueba también el contrato: cada
ruta que usa `services/` existe en `api/src/api/` y los campos de los tipos
`*Request` / `*Response` coinciden con los DTO de `api`.

### Nivel 4 — Trazabilidad de requirements (obligatorio para features con `"sdd": true`)

Cada `R<n>` de `specs/<feature>/requirements.md` debe poder mapearse a al menos
una `V<n>` de los niveles 2 o 3. El reviewer rechaza si falta cobertura.

El implementer documenta el mapa en `progress/<feature>/impl.md`:

```markdown
## Trazabilidad
- R1 → V4: tras el logout, `doc_count` de `sessions` es 0 (`verifications/auth-logout/auto.sh`)
- R2 → V1: la respuesta es 200 con `success: true` (`verifications/auth-logout/auto.sh`)
- R3 → V2: `POST /auth/logout` sin `Authorization` → 401 `AuthError` (`verifications/auth-logout/auto.sh`)
- R4 → V3: `POST /dummy` con el token cerrado → 401 `AuthError` (`verifications/auth-logout/auto.sh`)
```

El reviewer vuelve a ejecutar `./init.sh` y, si existe, `verifications/<feature>/auto.sh`, y
relee el código citado en las `V<n>` de `app`.

## Pruebas manuales de `app` (documentadas, obligatorias si se toca `app`)

`app` no tiene tests automáticos, así que cada feature que toca `app` deja
documentado cómo probarla a mano en `verifications/<feature-name>/manual.md`. Las
escribe el implementer y las revisa el reviewer, pero **ninguno las
ejecuta**: las ejecuta el humano cuando todo está terminado, y no bloquean
el `done`.

- Cada `R<n>` de `app` aparece en el `Cubre:` de al menos un caso `P<n>`.
- Cada paso es una acción concreta en la pantalla (qué se escribe, qué se
  pulsa) y el resultado esperado es algo que se ve, sin leer código.
- Incluye los caminos de error (credenciales incorrectas, `api` caída, ...)
  y cómo provocarlos.

Formato (ejemplo con la pantalla de login actual):

```markdown
# Pruebas manuales — <feature-name>

> Para ejecutar a mano cuando todo esté terminado. Rellena `Resultado` con
> `OK` o con lo que falla.

## Preparación

1. `./init.sh` (crea `api/.env` si no existe), `docker compose up -d db` y
   `cd api && cargo run`.
2. `cd app && npm run dev` y abre `http://localhost:5173`.
3. <datos previos que hagan falta y cómo crearlos>.

## P1 — Email inválido deshabilita el botón

Cubre: R1

1. Escribe `no-es-un-email` en "Email".
2. Mira el botón "Log in".

**Esperado:** el campo se marca como inválido y "Log in" está deshabilitado.
**Resultado:** _pendiente_
```

## Anti-patrones (no hacer)

- ❌ "He añadido el endpoint, debería funcionar." → falta la petición real
  con su respuesta.
- ❌ Comprobar solo que la respuesta es 200. → hay que comprobar el cuerpo
  (`success`, `data` / `error.type`) y los efectos en la BD.
- ❌ Simular el backend en `app` o probar contra respuestas inventadas.
- ❌ Verificar contra la CouchDB de `compose.yaml` (la de desarrollo) en
  lugar de la de `start_env`.
- ❌ Una `V<n>` de `app` sin `archivo:línea`, o que cita código que no
  demuestra el `R<n>`.
- ❌ Marcar la feature como `done` sin pasar `./init.sh`.

## Verificación final antes de cerrar

```bash
./init.sh                             # debe terminar con [OK] Entorno listo
./verifications/<feature-name>/auto.sh     # si existe: todas las comprobaciones en [OK]
```

Si algo de lo anterior no es OK, **no** marques nada como `done`. Anota el
motivo del bloqueo en el informe de la feature (`progress/<feature-name>/`),
referéncialo desde `progress/current.md` y pon estado `blocked` en
`feature_list.json`.
