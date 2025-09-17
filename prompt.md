Crea un workspace Rust llamado `ghost` con dos crates: `frontend` (Yew) y `backend` (Axum). 
Estructura exacta y contenidos:

- `ghost/Cargo.toml` con [workspace].
- `ghost/.env` con DATABASE_URL, INBESTIA_API_URL, INBESTIA_API_KEY, BIND_ADDR.
- `ghost/Trunk.toml` con proxy a http://127.0.0.1:8081 reescribiendo `/api/`.
- Crate `backend` con Cargo.toml y un `src/main.rs` que implemente un gateway Axum:
  rutas públicas `/health`, `/api/v1/info`; rutas autenticadas `/api/v1/*` (analyze, historical, indicators, compare, providers/status, metrics/*) que reenvían a `INBESTIA_API_URL` e inyectan `Authorization: Bearer $INBESTIA_API_KEY`.
  Agrega CORS permisivo y TraceLayer.
- Crate `frontend` con Cargo.toml y archivos:
  `index.html` (Bulma por CDN),
  `src/main.rs`, `src/app.rs`,
  `src/components/{mod.rs,info_card.rs}`,
  `src/services/{mod.rs,api.rs}`,
  `src/domain/{mod.rs,types.rs}`,
  `src/utils/{mod.rs,errors.rs}`.
  `info_card.rs` debe llamar a `/api/v1/info` y renderizar tarjeta Bulma.

Asegúrate de que `trunk serve` sirva el frontend en http://127.0.0.1:8080 y el backend en http://127.0.0.1:8081, comunicándose vía proxy de Trunk.


# terminal 1
cd ghost/backend
cargo run

# terminal 2
cd ghost/frontend
trunk serve --open
