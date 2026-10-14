# AGENTS.md — Mapa de navegación para agentes de IA

> Este archivo es el **punto de entrada** para cualquier agente que trabaje en este
> repositorio. NO es una biblia de reglas: es un **mapa**. Lee solo lo que
> necesites cuando lo necesites (divulgación progresiva).

---

## 1. Antes de empezar (obligatorio)

1. Ejecuta `./init.sh` y verifica que termina sin errores. Si falla, **para**
   y resuelve el entorno antes de tocar código.
2. Lee `progress/current.md` para entender en qué estado quedó la última sesión.
3. Lee `feature_list.json`. Toda feature nueva (`"sdd": true`) pasa por
   **Spec Driven Development** — ver `docs/specs.md` y §4 de este archivo.
4. Lee `docs/specs.md` antes de tocar cualquier spec o feature `sdd: true`.

## 2. Mapa del repositorio

| Archivo / carpeta            | Qué contiene                                                                | Cuándo leerlo |
|------------------------------|-----------------------------------------------------------------------------|---------------|
| `feature_list.json`          | Lista de tareas con estado (`pending` / `spec_ready` / `in_progress` / `done` / `blocked`) | Siempre, al empezar |
| `progress/current.md`        | Estado de la sesión actual                                                  | Siempre, al empezar |
| `progress/history.md`        | Bitácora append-only: una entrada por feature terminada                     | Si necesitas contexto histórico |
| `progress/<feature>/`        | Informes de la feature: `spec.md` (bloqueo del spec_author), `impl.md` (implementer), `review.md` (reviewer) | Al trabajar en esa feature |
| `specs/<feature>/`           | `requirements.md` + `design.md` + `tasks.md` (Kiro-style)                   | Antes de implementar cualquier feature con `"sdd": true` |
| `docs/api-architecture.md`, `docs/app-architecture.md` | Qué significa "hacer un buen trabajo" en `api` y en `app` | Antes de implementar |
| `docs/api-conventions.md`, `docs/app-conventions.md`   | Reglas de estilo, nombres, estructura de `api` y de `app` | Antes de escribir código |
| `docs/specs.md`              | Proceso SDD: EARS notation, los 3 archivos, puerta de aprobación humana     | Antes de redactar o leer un spec |
| `docs/verification.md`       | Cómo verificar que tu trabajo funciona (incluye trazabilidad requirements)  | Antes de declarar una tarea como `done` |
| `verifications/`             | `<feature>/auto.sh`: comprobaciones automáticas de la feature (contra la API real y sobre el código de `app`); `<feature>/manual.md`: pruebas manuales de `app` para el humano; `lib.sh`: `check` y entorno desechable (CouchDB + API) | Al verificar o revisar |
| `CHECKPOINTS.md`             | Criterios objetivos de "estado final correcto"                              | Para auto-evaluarte |
| `.claude/agents/`            | Definiciones de subagentes (`leader`, `spec_author`, `implementer`, `reviewer`). El `leader` se ejecuta como agente principal (`claude --agent leader`): un subagente no puede lanzar otros | Si orquestas trabajo |
| `.claude/settings.json`      | Hooks (comprobación de tipos al editar; al terminar, `./init.sh` en el implementer y `./init.sh --quick` en el leader y el spec_author) y permisos | Si un hook te bloquea |
| `api/`                       | Backend: Rust (Axum + CouchDB)                                              | Para implementar |
| `app/`                       | Frontend: React + TypeScript + Vite + Chakra UI v2                          | Para implementar |
| `init.sh`                    | Verificación del entorno: herramientas, `api/.env` (lo crea si falta), arnés, `feature_list.json`, compilación de `app` y `api`, formato, lint y reglas de arquitectura de `app` | Siempre, al empezar y al verificar |

`api` y `app` no tienen tests automáticos: la verificación está en
`docs/verification.md`.

## 3. Reglas duras (no negociables)

- **Una sola feature a la vez.** No mezcles cambios de varias tareas en la misma sesión.
- **No declares una tarea `done` sin verificación.** `./init.sh` correcto,
  `verifications/<feature>/auto.sh` (si existe) con todo en `[OK]` y el
  reviewer en `APPROVED` (ver `docs/verification.md`).
- **No saltes la fase de spec.** Toda feature con `"sdd": true` debe pasar
  por `spec_author` y obtener aprobación humana antes de tocar código.
- **No saltes la puerta de aprobación humana.** El leader detiene el flujo
  en `spec_ready` y espera.
- **Documenta lo que haces** en `progress/current.md` mientras trabajas, no al final.
- **Deja el repositorio limpio** antes de cerrar la sesión (ver §5).
- **Si no sabes algo, busca en `docs/`** antes de inventarlo.

## 4. Flujo de trabajo (SDD)

```
pending → [spec_author] → spec_ready → ⏸ HUMANO → in_progress → [implementer → reviewer] → done
```

1. El leader detecta la primera feature `pending` con `"sdd": true`.
2. El leader lanza `spec_author`, que crea
   `specs/<name>/{requirements,design,tasks}.md` y marca el status como
   `spec_ready`.
3. **Pausa.** El humano lee el spec en `specs/<name>/` y aprueba (o pide cambios).
4. Una vez aprobado, el leader cambia el status a `in_progress` y lanza `implementer`.
5. El implementer ejecuta `tasks.md` una a una, marcándolas `[x]`, y escribe
   sus comprobaciones en `verifications/<name>/auto.sh`.
6. El reviewer verifica trazabilidad `R<n>` ↔ comprobación `V<n>` y tasks
   completas; aprueba o rechaza en `progress/<name>/review.md`. Si rechaza,
   el leader relanza al implementer con `review.md`, hasta 3 rondas;
   después, `blocked`.
7. Si aprueba, el implementer cierra la feature (§5): la marca `done` y,
   como último paso, añade su resumen a `progress/history.md`.

## 5. Cierre de sesión (lifecycle)

Lo aplica quien cierra la feature (el implementer, cuando el reviewer
aprueba) o la sesión (el agente principal), no cada subagente al terminar
su parte.

Antes de terminar:

1. Ejecuta `./init.sh` — todo correcto.
2. Si la feature está acabada y el reviewer la aprobó: marca `status: "done"`
   en `feature_list.json` (ver §4).
3. Vacía `progress/current.md` dejando solo la plantilla, salvo si la
   feature queda `blocked`: entonces conserva la referencia a su informe.
4. No dejes archivos temporales, ni `println!` / `dbg!` / `console.log` de
   debug, ni TODOs sin contexto.
5. Recorre C5 de `CHECKPOINTS.md` (el reviewer no lo evalúa) y corrige lo
   que falte. La entrada del historial se comprueba en el paso 6.
6. **Último paso, solo si se ha terminado una feature:** añade al final de
   `progress/history.md` el resumen de lo implementado (a partir de
   `progress/<name>/impl.md`). Es la única entrada de la feature en el
   historial y lo último que se escribe: después no se toca nada más. Las
   sesiones que no terminan una feature (spec, `blocked`) no añaden nada.

## 6. Si te bloqueas

- Relee la sección relevante de `docs/`.
- Si la herramienta no hace lo que esperas, **no inventes un workaround**:
  anota el motivo en el informe de la feature (`progress/<name>/`),
  referéncialo desde `progress/current.md`, pon estado `blocked` y para la
  sesión.
