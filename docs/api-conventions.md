# Convenciones de código de `api`

> Homogeneidad extrema. La IA predice mejor cuando el repositorio se parece
> a sí mismo en todas partes.

## Estilo Rust

- **Versión:** Rust 1.85+, edición 2024.
- **Formato:** `cargo fmt` (rustfmt por defecto, líneas máximo 100
  caracteres). `cargo clippy` sin warnings; un `#[allow(...)]` solo con un
  comentario que explique por qué.
- **Imports:** un único bloque de `use` sin líneas en blanco, en el orden de
  rustfmt. Una línea por crate, agrupando con llaves
  (`use crate::{errors::..., models::...}`). Dentro de `api/`, los módulos
  hermanos se importan con `super::`.
- **Strings:** `"...".to_string()` para crear un `String`. Nada de
  `String::from` ni `.into()`.
- **`format!`** para interpolación, con la variable dentro de las llaves
  cuando es un identificador simple (`format!("email:{email}")`).
- **Textos:** mensajes de error, de `expect` y comentarios en inglés.

## Nombres

| Tipo                       | Convención                  | Ejemplo                          |
|----------------------------|-----------------------------|----------------------------------|
| Módulos `api/`             | `<dominio>_api`             | `auth_api.rs`                    |
| Módulos `services/`        | `<dominio>_service`         | `auth_service.rs`                |
| Módulos `db/`              | `<nombres>_collection`      | `users_collection.rs`            |
| Módulos `models/`          | `<nombre>_model`            | `user_model.rs`                  |
| Módulos `helpers/`         | `snake_case` temático       | `auth_helpers.rs`, `validation.rs` |
| Tipos                      | `PascalCase`                | `AuthService`, `UsersCollection` |
| DTOs                       | `<Acción>Request` / `<Acción>Response` | `LoginRequest`        |
| Funciones / variables      | `snake_case`                | `normalize_email`                |
| Router de un dominio       | `<dominio>_api()`           | `auth_api(auth_service)`         |
| Handlers                   | verbo de la acción          | `register`, `login`              |
| Métodos de colección       | `create_` / `get_` / `delete_<nombre>` | `get_session`         |
| Getters                    | `get_<campo>`               | `get_user_id`                    |
| Predicados                 | `is_<algo>`                 | `is_valid_password`              |
| Constantes                 | `UPPER_SNAKE`               | `PASSWORD_MAX_BYTES`             |
| Bases de CouchDB           | plural en minúsculas        | `"users"`                        |
| Campos JSON                | `snake_case`                | `session_token`                  |
| Variables de entorno       | `UPPER_SNAKE` con prefijo de grupo | `DB_HOST`, `AUTH_PRIVATE_KEY` |

## Estructura de archivo

Cada archivo de `api/` agrupa, por endpoint, el `*Request`, el `*Response` y
el handler, en ese orden:

```rust
use super::extractors::ApiJson;
use crate::{models::ApiResponse, services::auth_service::AuthService};
use axum::extract::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    session_token: String,
}

pub async fn login(
    State(auth_service): State<Arc<AuthService>>,
    ApiJson(req): ApiJson<LoginRequest>,
) -> ApiResponse<LoginResponse> {
    match auth_service.login(req.email, req.password).await {
        Ok(session_token) => ApiResponse::success(LoginResponse { session_token }),
        Err(error) => ApiResponse::error(error),
    }
}
```

Los servicios son un `struct <Dominio>Service` con sus colecciones como
campos; el `impl` empieza por `new`, sigue con los métodos públicos y termina
con los privados.

Las colecciones envuelven una única base:

```rust
pub struct UsersCollection {
    db: Database,
}

impl UsersCollection {
    pub async fn new(db_client: &Client) -> Self {
        Self {
            db: db_client
                .db("users")
                .await
                .expect("Couldn't obtain 'users' database"),
        }
    }
}
```

Los modelos tienen campos privados, se crean con `::new(...)` (o un
constructor con nombre, como `Identity::email`) y se leen con getters que
devuelven una copia:

```rust
#[derive(Serialize, Deserialize, CouchDocument, Clone)]
pub struct User {
    #[serde(skip_serializing_if = "String::is_empty")]
    _id: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    _rev: String,

    email: String,
    password_hash: String,
}

impl User {
    pub fn new(id: String, email: String, password_hash: String) -> Self { ... }

    pub fn get_id(&self) -> String {
        self._id.clone()
    }
}
```

Los `*Response` exponen solo lo que el cliente necesita, nunca campos
internos o sensibles.

## Manejo de errores

Todo error es un `AppError` con una variante de `ErrorType` y un mensaje
corto en inglés terminado en punto:

```rust
AppError::new(ErrorType::DbError, "Couldn't get user.".to_string())
```

- Los errores se propagan con `?`. Si un mismo error se repite en una
  función, se define una vez como closure:
  `let unauthorized = || AppError::new(ErrorType::AuthError, "Unauthorized.".to_string());`
- Las colecciones traducen los errores de CouchDB: no encontrado →
  `Ok(None)`, conflicto en una creación con unicidad → `Ok(false)`, el resto →
  `ErrorType::DbError`.
- Los errores de librerías se descartan con `map_err` y se sustituyen por un
  `AppError`; nunca se reenvía su texto.
- Los handlers hacen `match` y responden con `ApiResponse::success(...)` o
  `ApiResponse::error(error)`.
- `unwrap()` / `expect()` solo en el arranque (`main.rs`, `config.rs` y los
  `new` de servicios y colecciones), nunca en el camino de una petición.
- `let _ =` solo para operaciones de compensación best-effort, con un
  comentario que lo explique.
- Nada de `println!` para errores de petición.

## Comentarios

Por defecto **no** se escriben. Solo se permiten cuando explican un *por qué*
no obvio (p. ej. defensa ante timing attacks, invariante de CouchDB). Los
`///` se usan en funciones públicas cuyo contrato no se deduce de la firma
(p. ej. qué significa `Ok(false)`). Los nombres deben hacer el resto.
