# Pruebas manuales — app-auth

> Para ejecutar a mano cuando todo esté terminado. Rellena `Resultado` con
> `OK` o con lo que falla.

## Preparación

1. `./init.sh` (crea `api/.env` si no existe), `docker compose up -d db` y
   `cd api && cargo run` (escucha en `http://localhost:3000`).
2. Comprueba que `app/.env` contiene `VITE_API_URL=http://localhost:3000`
   (sin `/` final; si no está, añádelo).
3. `cd app && npm run dev` y abre `http://localhost:5173`.
4. Usa un email que no exista aún, por ejemplo `manual-<fecha>@example.org`, y
   la contraseña `password123`. En los pasos se llaman `<email>` y
   `<password>`.

## P1 — Pantalla de login inicial

Cubre: R14, R15

1. Abre la app sin haber iniciado sesión.
2. Escribe `secreto12` en "Password".

**Esperado:** se ven el título "Log in to continue", los campos "Email" y
"Password" y los botones "Log in" (azul) y "Sign up" (verde). En "Password"
los caracteres se ven como puntos, no como texto.
**Resultado:** _pendiente_

## P2 — Email o contraseña no válidos deshabilitan los botones

Cubre: R21, R22

1. Con los dos campos vacíos, mira los botones.
2. Escribe `no-es-un-email` en "Email" y `password123` en "Password".
3. Corrige el email a `<email>` y deja `1234567` (7 caracteres) en "Password".
4. Completa la contraseña a `12345678`.

**Esperado:** en 1, 2 y 3 "Log in" y "Sign up" están grises y no responden al
pulsarlos; en 2 el campo "Email" se marca en rojo y en 3 el campo "Password".
En 4 los dos botones se habilitan (azul y verde).
**Resultado:** _pendiente_

## P3 — Hover de los botones

Cubre: R23, R24

1. Con email y contraseña válidos, pasa el puntero por encima de "Log in" y
   luego de "Sign up".
2. Vacía "Password" y vuelve a pasar el puntero por encima de los dos botones.

**Esperado:** en 1 cada botón se ve más claro (más brillante) mientras el
puntero está encima. En 2 los botones siguen grises, con el cursor de
"no permitido", y su aspecto no cambia al pasar el puntero.
**Resultado:** _pendiente_

## P4 — Registro de un usuario nuevo

Cubre: R1, R2, R3, R4, R5, R6, R7, R9, R10, R11, R18, R25, R26, R27, R33, R34

1. Escribe `<email>` y `<password>`.
2. Pulsa "Sign up" y mira los botones mientras espera la respuesta (si la
   respuesta es demasiado rápida, en las herramientas de desarrollador del
   navegador, pestaña "Network", activa "Slow 3G" y repite con otro email).

**Esperado:** mientras la petición está en curso, "Sign up" muestra un
spinner en lugar del texto y los dos botones están deshabilitados. Después
desaparece el formulario de login y se ve "Signed up as `<email>`" y el botón
"Check session".
**Resultado:** _pendiente_

## P5 — Registro con un email ya registrado

Cubre: R29, R32

1. Recarga la página (la sesión se pierde y vuelve el login).
2. Escribe el mismo `<email>` de P4 y cualquier contraseña válida.
3. Pulsa "Sign up".

**Esperado:** se queda en la pantalla de login con el mensaje rojo
"Could not sign up with this email and password", sin otro texto técnico.
Al cambiar el email o la contraseña, el mensaje desaparece.
**Resultado:** _pendiente_

## P6 — Login correcto e incorrecto

Cubre: R8, R16, R17, R18, R19, R28, R32, R33, R34

1. Recarga la página. Escribe `<email>` y una contraseña válida distinta de
   `<password>`, por ejemplo `otraclave99`, y pulsa "Log in".
2. Cambia la contraseña a `<password>` y pulsa "Log in" (con "Slow 3G"
   activado si quieres ver el estado de carga).

**Esperado:** en 1 aparece el mensaje rojo "Incorrect email or password" y
nada más. En 2, mientras la petición está en curso, "Log in" muestra un
spinner y los dos botones están deshabilitados; después se ve
"Logged in as `<email>`" y el botón "Check session".
**Resultado:** _pendiente_

## P7 — Sin conexión con api

Cubre: R1, R2, R3, R5, R30, R32

1. Recarga la página y para `api` (Ctrl+C en la terminal de `cargo run`).
2. Escribe `<email>` y `<password>` y pulsa "Log in".
3. Pulsa "Sign up".

**Esperado:** en ambos casos aparece el mensaje rojo
"Could not connect to the server", sin textos de excepciones
(como "Failed to fetch"). Vuelve a arrancar `api` con `cargo run`.
**Resultado:** _pendiente_

## P8 — Error inesperado del servidor

Cubre: R31, R32

1. Con `api` en marcha, para la base de datos: `docker compose stop db`.
2. Recarga la página, escribe `<email>` y `<password>` y pulsa "Log in".
3. Pulsa "Sign up".
4. Vuelve a arrancar la base: `docker compose start db`.

**Esperado:** en 2 y 3 aparece el mensaje rojo
"Something went wrong, try again later", sin textos de `api` (como
"Internal server error") ni de excepciones.
**Resultado:** _pendiente_

## P9 — Comprobar la sesión

Cubre: R13, R38, R39, R40, R41, R42

1. Inicia sesión con `<email>` y `<password>` (como en P6.2).
2. Pulsa "Check session" (con "Slow 3G" activado si quieres ver el estado de
   carga).

**Esperado:** mientras la petición está en curso, "Check session" muestra un
spinner y está deshabilitado. Después aparece en verde "Session is active" y
la pantalla sigue mostrando "Logged in as `<email>`".
**Resultado:** _pendiente_

## P10 — Comprobar la sesión con errores

Cubre: R43, R44

1. Con la sesión abierta de P9, para `api` y pulsa "Check session".
2. Vuelve a arrancar `api`; como la sesión vive en CouchDB, sigue siendo
   válida. Para la base (`docker compose stop db`) y pulsa "Check session".
3. Vuelve a arrancar la base (`docker compose start db`).

**Esperado:** en 1 aparece en rojo "Could not connect to the server"; en 2,
"Something went wrong, try again later". En ningún caso se cierra la sesión:
sigue la pantalla "Logged in as `<email>`".
**Resultado:** _pendiente_

## P11 — Sesión caducada (401 en una ruta protegida)

Cubre: R12, R35, R36, R37

1. Con una sesión abierta, abre Fauxton en `http://localhost:5984/_utils`
   (usuario `admin`, contraseña `password`), entra en la base `sessions` y
   borra todos sus documentos.
2. Vuelve a la app y pulsa "Check session".
3. Recarga la página.

**Esperado:** en 2 vuelve la pantalla de login con el mensaje rojo
"Your session has expired, log in again". En 3 la app sigue en la pantalla
de login: la sesión no se ha guardado en el navegador.
**Resultado:** _pendiente_

## P12 — Ninguna respuesta es aleatoria

Cubre: R20

1. Recarga la página, escribe `<email>` y `otraclave99` y pulsa "Log in"
   cinco veces seguidas.
2. Cambia a `<password>` y pulsa "Log in"; recarga y repítelo tres veces.

**Esperado:** en 1 las cinco veces aparece "Incorrect email or password"; en
2 las tres veces se abre la sesión con "Logged in as `<email>`".
**Resultado:** _pendiente_
