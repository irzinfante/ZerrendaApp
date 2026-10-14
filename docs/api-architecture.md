# Arquitectura de `api` — Qué significa "hacer un buen trabajo"

> Este documento define el estándar de calidad del proyecto `api` (backend en
> Rust: Axum + CouchDB). Los agentes revisores evalúan código contra este
> archivo. Si no está aquí, no es un requisito.

## Principios

1. **Capas claras, una pieza por dominio.** Toda funcionalidad sigue la misma estructura:
   - `api/<dominio>_api.rs` — handlers y DTOs `*Request` / `*Response`;
     `api/mod.rs` expone una función por dominio que devuelve su `Router`.
   - `services/<dominio>_service.rs` — lógica de negocio (`<Dominio>Service`),
     devuelve `Result<T, AppError>`.
   - `db/<nombre>_collection.rs` — acceso a una base de CouchDB. Solo
     persistencia.
   - `models/<nombre>_model.rs` — documentos de dominio.
   - `helpers/` — funciones puras transversales (validación, hash, tokens, ids).
   - `errors/`, `config.rs` y `main.rs` (solo composición, sin lógica).
   Las dependencias van solo hacia abajo: `api → services → db → models`.
   `errors/` y `helpers/` pueden usarse desde cualquier capa.

2. **Dependencias acotadas.** Solo las crates de `Cargo.toml`. Si una feature
   requiere una dependencia, primero se discute (estado `blocked`).

3. **Respuesta uniforme.** Todo endpoint responde con `ApiResponse<T>`,
   también en caso de error, y el código HTTP se deriva del `ErrorType`, nunca
   se fija en el handler. Los cuerpos JSON se extraen con `ApiJson<T>` para que
   los rechazos sigan el mismo formato. Una categoría de error nueva se añade
   como variante de `ErrorType`.

4. **Errores sin pistas.** Lo que ve el cliente no ayuda a un atacante:
   mensajes genéricos y fijos, sin detalles internos, de la BD ni de librerías;
   sin distinguir la causa de un fallo de autenticación o de acceso; y sin
   diferencias de tiempo de respuesta que revelen si algo existe. La ausencia
   de un dato no es un error (`Ok(None)`).

5. **Estado creado una vez al arrancar.** Los recursos con estado (cliente de
   BD, claves, servicios, colecciones) se crean en el arranque y se inyectan
   (`State<Arc<Service>>`), nunca por petición ni tras un `Mutex` que
   serialice las peticiones. El trabajo intensivo en CPU no bloquea el runtime
   async (`spawn_blocking`).

6. **Autenticación y propiedad.** Las rutas son protegidas por defecto
   (`enforce_auth`); una ruta pública requiere una razón explícita. La
   identidad del usuario sale solo de la sesión verificada contra la BD, nunca
   de datos enviados por el cliente. Cada recurso de usuario registra a su
   propietario y el servicio comprueba la propiedad antes de cualquier acceso;
   un recurso ajeno se trata como inexistente.

7. **Persistencia acorde a CouchDB.** Leer por `_id` siempre que sea posible.
   La unicidad se garantiza mediante el `_id` del documento y el 409 de
   CouchDB, no buscando antes de crear. Las actualizaciones usan `_rev` y los
   conflictos no se ignoran.

8. **Validación en el servidor.** La API es la autoridad: toda entrada se
   valida en el servicio con funciones puras de `helpers/`, y los valores con
   forma canónica (como el email) se normalizan antes de guardarlos o
   compararlos.

9. **Configuración por entorno.** Las variables de entorno se leen solo en
    `config.rs`, se documentan en `.env.example` y tienen su valor local en
    `init.sh` (con el que crea `api/.env` si no existe); los valores llegan al
    resto de módulos por parámetro desde `main.rs`.

10. **CORS mínimo.** Solo los orígenes configurados y los métodos y cabeceras
    que la API usa; quien introduce uno nuevo amplía la capa en el mismo
    cambio.

## Flujo de datos

```
cliente HTTP
   │
   ▼
main.rs (Router Axum) ── CorsLayer
   ├─ rutas públicas (excepción justificada)
   └─ rutas protegidas ── enforce_auth → Extension<Session>
          ▼
   api/<dominio>_api.rs     (ApiJson<*Request>, user_id de la sesión)
          ▼
   services/<dominio>_service.rs  (reglas, propiedad, Result<_, AppError>)
          ├─→ helpers/
          └─→ db/<nombre>_collection.rs ─→ CouchDB
          ▼
   ApiResponse<*Response> → código HTTP + JSON → cliente
```

## Qué NO hacer

- No acceder a `db/` desde handlers ni poner lógica de negocio en `api/` o `db/`.
- No devolver JSON fuera de `ApiResponse<T>` ni usar `axum::Json` como
  extractor.
- No crear errores con strings sueltos en lugar de variantes de `ErrorType`.
- No validar la sesión manualmente en un handler ni aceptar un `user_id`
  enviado por el cliente.
- No usar `_find` cuando el documento se puede leer por `_id`.
- No usar CORS permisivo (`permissive()`, `Any`) ni cabeceras
  `Access-Control-*` a mano.
