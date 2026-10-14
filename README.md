# ZerrendaApp

Aplicación para gestionar listas e inventarios

## Desarrollo con agentes

El repositorio incluye un arnés de Spec Driven Development (SDD) para Claude
Code. El punto de entrada para los agentes es `AGENTS.md`; el proceso SDD
está descrito en `docs/specs.md`.

### Requisitos

- `node`, `npm`, `cargo`, `docker`, `jq` y `python3` en el `PATH`.
- Docker en marcha: las comprobaciones de `verifications/` levantan una
  CouchDB y una api desechables.

### 1. Añadir la feature

Cada feature es una entrada en `feature_list.json` con un `name` único
(será el nombre de `specs/<name>/` y `progress/<name>/`), un `title`, una
`description`, la lista de criterios de `acceptance`, `"sdd": true` y
`"status": "pending"`. El arnés trabaja siempre sobre la primera feature que
no esté `done` ni `blocked`, una sola a la vez.

### 2. Arrancar la sesión

1. `./init.sh` — debe terminar todo correcto. Si no existe `api/.env`, lo
   crea a partir de `api/.env.example` con los valores locales.
2. Lanza Claude Code con el orquestador como agente principal:
   `claude --agent leader`. Tiene que ser así, y no pedirle a una sesión
   normal que lance el `leader`: un subagente no puede lanzar otros.
3. Pídele: **«implementa la siguiente feature pendiente»**.

### 3. Fase 1 — Spec

El `spec_author` escribe `specs/<feature>/{requirements.md, design.md,
tasks.md}`, pasa la feature a `spec_ready` y el leader se detiene. Revisa los
tres archivos y responde **«aprobado»**, o pide los cambios que necesites.
Sin esa aprobación no se escribe código.

### 4. Fase 2 — Código

Tras la aprobación, el leader pasa la feature a `in_progress` y:

1. El `implementer` sigue `tasks.md` una a una y escribe sus comprobaciones
   en `verifications/<feature>/auto.sh`. Su informe queda en
   `progress/<feature>/impl.md`.
2. El `reviewer` vuelve a ejecutarlas y deja su veredicto en
   `progress/<feature>/review.md`. Si pide cambios, el implementer los
   corrige y se revisa otra vez, hasta 3 rondas; después la feature queda
   `blocked`.
3. Con `APPROVED`, el implementer cierra la feature: la marca `done` y añade
   su resumen a `progress/history.md`.

### Lanzar las comprobaciones a mano

Desde la raíz del repositorio, con Docker en marcha y `api/.env` creado por
`./init.sh`:

```bash
./verifications/<feature>/auto.sh   # sale con 0 si todo está [OK] y con 1 si algo falla
```

Si la feature toca `api`, el script levanta una CouchDB vacía y desechable (contenedor
`zerrenda-app-verify-db`, puerto 5985) y la api sobre ella (puerto 3001),
ejecuta las comprobaciones y, al terminar, para la api y elimina el
contenedor con sus datos. Al usar puertos distintos de los de `compose.yaml`,
puede lanzarse con el entorno local levantado.

Cada comprobación imprime `[OK] V<n> — ...` o
`[FAIL] V<n> — ... (expected: X, got: Y)`. Un `[FAIL] ENV — ...` es un fallo
del entorno, no de la feature: Docker parado, un puerto ocupado o la api sin
compilar.

### 5. Pruebas manuales

`app` no tiene tests automáticos: cada feature que la toca deja en
`verifications/<feature>/manual.md` cómo probarla a mano. Ejecútalas cuando
todo esté terminado y rellena el `Resultado` de cada caso.

### Retomar una sesión

Lanza otra vez `claude --agent leader` y repite la petición. El leader
decide según el `status` de la feature en `feature_list.json`:

| Status        | Qué hace el leader                                             |
|---------------|----------------------------------------------------------------|
| `pending`     | Lanza al `spec_author` (fase 1).                               |
| `spec_ready`  | Espera tu «aprobado» antes de seguir.                          |
| `in_progress` | La sesión se interrumpió: te pregunta si reanudar o abortar.   |

Las features `blocked` se saltan: el motivo está en `progress/<feature>/`,
referenciado desde `progress/current.md`. Resuélvelo y vuelve a ponerla en
el status que corresponda para que el leader la retome.
