# CHECKPOINTS — Evaluación del estado final

> En sistemas multi-agente no se evalúa el camino, se evalúa el destino.
> Estos son los checkpoints objetivos que un juez (humano o IA) puede usar
> para decidir si el proyecto está sano.

## C1 — El arnés está completo

- [ ] Existen los archivos: `AGENTS.md`, `init.sh`, `feature_list.json`,
      `progress/current.md`.
- [ ] Existen los docs: `docs/api-architecture.md`, `docs/api-conventions.md`,
      `docs/app-architecture.md`, `docs/app-conventions.md`, `docs/specs.md`,
      `docs/verification.md`.
- [ ] `./init.sh` termina con exit code 0.

## C2 — El estado es coherente

- [ ] Como mucho una feature en `in_progress` en `feature_list.json`.
- [ ] Toda feature `done` con `"sdd": true` tiene su mapa de trazabilidad en
      `progress/<feature>/impl.md`; si toca `api`, su `verifications/<feature>/auto.sh`;
      y si toca `app`, sus pruebas manuales en `verifications/<feature>/manual.md`.
- [ ] `progress/current.md` está vacío o describe la sesión activa
      (no contiene basura de sesiones anteriores).

## C3 — El código respeta la arquitectura

- [ ] `api/` y `app/` siguen las capas y la estructura de
      `docs/api-architecture.md` y `docs/app-architecture.md`.
- [ ] No hay dependencias nuevas en `api/Cargo.toml` ni en `app/package.json`
      sin haberse discutido antes (estado `blocked`).
- [ ] No hay `println!` / `dbg!` / `console.log` sueltos para debug, ni TODOs
      sin contexto.

## C4 — La verificación es real

- [ ] `./init.sh` pasa compilación, formato, lint y reglas de arquitectura
      de `app`.
- [ ] Si existe, `verifications/<feature>/auto.sh` termina con exit code 0: todas las
      comprobaciones en `[OK]`.
- [ ] Las comprobaciones de `api` usan la API real sobre la CouchDB
      desechable de `start_env` (`verifications/lib.sh`), sin respuestas
      simuladas.

## C5 — La sesión se cerró bien

> Lo comprueba quien cierra la feature o la sesión (ver `AGENTS.md` §5),
> no el reviewer. La entrada del historial se escribe la última.

- [ ] No hay archivos sin trackear sospechosos (`*.tmp`, `*.log`,
      `api/target/` o `app/dist/` fuera del `.gitignore`).
- [ ] Si se ha terminado una feature, su resumen es la última entrada de
      `progress/history.md`, y es la única de esa feature.
- [ ] La última feature trabajada está reflejada en su estado correcto.

## C6 — Spec Driven Development

- [ ] Toda feature con `"sdd": true` en estado `spec_ready`, `in_progress`
      o `done` tiene su carpeta `specs/<feature>/` con los 3 archivos:
      `requirements.md`, `design.md`, `tasks.md`.
- [ ] `requirements.md` usa EARS estricto (ver `docs/specs.md`).
- [ ] Toda feature `done` con `"sdd": true` tiene todas sus tasks marcadas
      `[x]` en `tasks.md`.
- [ ] Cada `R<n>` de `requirements.md` está cubierto por al menos una
      comprobación `V<n>` concreta (ver `docs/verification.md`).

---

**Cómo usar este archivo:** el reviewer (`.claude/agents/reviewer.md`)
recorre C1-C4 y C6, marca `[x]` o `[ ]`, y rechaza la feature si quedan
boxes vacíos. C5 lo recorre quien cierra la feature o la sesión, al final
del cierre, y no termina hasta que está completo.
