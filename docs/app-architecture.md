# Arquitectura de `app` — Qué significa "hacer un buen trabajo"

> Este documento define el estándar de calidad del proyecto `app` (frontend en
> React + TypeScript + Vite + Chakra UI v2). Los agentes revisores evalúan
> código contra este archivo. Si no está aquí, no es un requisito.

## Principios

1. **Capas claras, una pieza por dominio.** Cada carpeta de `src/` tiene una
   única responsabilidad, y cada feature nueva añade su pieza en cada capa
   (`services/<dominio>Service.ts`, `hooks/use<Dominio>.ts`, pantalla en
   `components/<Nombre>/`):
   - `main.tsx` — punto de entrada, sin lógica.
   - `App.tsx` — composición de pantallas y estado de alto nivel (sesión).
   - `components/<Nombre>/` — pantallas y layout; componen componentes `ui/`.
   - `components/ui/<Nombre>/` — envoltorios atómicos de Chakra UI. Solo
     presentación: todo por props, sin estado ni red.
   - `hooks/ui/use<Nombre>.tsx` — estado de un componente `ui/`; devuelven
     `{ <Nombre>: JSX.Element, ...estado }` (patrón de `useTextInput`).
   - `hooks/use<Dominio>.ts` — estado de dominio (sesión, listas…); conectan
     componentes con `services/`.
   - `services/` — cliente HTTP de `api`: un cliente base común y un
     `<dominio>Service.ts` por dominio. Único lugar con `fetch`.
   - `constants/` — constantes y variables de entorno.
   Las dependencias van solo hacia abajo, por dos ramas:
   - UI: `App → components → hooks/ui → components/ui`.
   - Dominio: `App → components → hooks/<dominio> → services`.

2. **Dependencias acotadas.** Solo las de `package.json`; HTTP con `fetch`
   nativo. Si una feature requiere una dependencia, primero se discute
   (estado `blocked`). Lo que se usa en ejecución va en `dependencies`; en `devDependencies` solo herramientas.

3. **Contrato con `api` tipado.** `services/` refleja el formato de respuesta
   de `docs/api-architecture.md` con un tipo `ApiResponse<T>` y un tipo
   `*Request` / `*Response` por endpoint, con los nombres de campo del backend
   (`snake_case`). Los errores llegan con código HTTP ≠ 200 y el mismo formato,
   así que el cliente base lee el cuerpo antes de decidir. La UI decide
   por código HTTP / `error.type`, nunca por el texto de `message`. La
   validación en la app es solo ergonomía; la API es la autoridad.

4. **Sesión centralizada.** El `session_token` se guarda en un único sitio
   (hook de autenticación) y el cliente base lo envía como
   `Authorization: Bearer <token>`. Un `401` en una ruta protegida cierra la
   sesión y vuelve al login. El token es opaco: no se decodifica ni se deduce
   nada de él.

5. **Errores visibles.** Fallos de red, `success: false` o `401` se muestran al
   usuario con un mensaje; nunca se ignoran en silencio ni se muestran detalles
   técnicos. Mientras una petición está en curso, la UI lo indica e impide
   reenviarla (p. ej. botón deshabilitado), sin bloquear el resto de la app.

6. **Configuración por entorno.** Variables `VITE_*` declaradas en
   `ImportMetaEnv` (`vite-env.d.ts`), documentadas en `.env.example` y
   expuestas solo desde `constants/Environment.ts`. Una variable nueva se
   añade a los tres sitios.

## Flujo de datos

```
usuario
   │
   ▼
App.tsx / components/<Pantalla>  ←  hooks/ui/use*.tsx → components/ui/*
   │
   ▼
hooks/use<Dominio>.ts   (estado de dominio)
   │
   ▼
services/<dominio>Service.ts → cliente base (fetch + Bearer token)
   │
   ▼
api ─→ ApiResponse<T> → services (tipado) → hook (estado) → re-render
```

## Qué NO hacer

- No llamar a `fetch` fuera de `services/` ni leer `import.meta.env` fuera de
  `constants/Environment.ts`.
- No meter estado, red ni lógica de negocio en `components/ui/`.
- No importar Chakra UI en pantallas si existe (o debería existir) un
  envoltorio en `ui/`.
- No persistir contraseñas.
- No duplicar la URL del backend ni las rutas; se definen una vez en
  `services/`.
- No simular respuestas del backend.
