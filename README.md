# Ghost Dashboard

Dashboard financiero construido con Rust + Yew para consumir la inBestia API.
Incluye buenas prácticas de organización, uso de Bulma CSS, y un proxy backend seguro para manejar credenciales.

## 🚀 Tecnologías Principales

**Frontend:**
- **Yew** (Rust + WASM, SPA)
- **yew-router** para ruteo
- **Bulma** para estilos (CSS puro, sin JS)

**Backend:**
- **Rust + Axum** para proxy/API
- **Manejo de claves** vía .env usando dotenvy
- **PostgreSQL** como base de datos (DATABASE_URL)

## 📂 Estructura del Proyecto

```
ghost/
├─ .env.example            # Plantilla de variables (subir a git)
├─ .env                    # Variables reales (NO subir a git)
├─ .gitignore              # Excluir .env, target/, etc.
├─ Cargo.toml              # Workspace con resolver "2"
├─ Trunk.toml              # Proxy config para desarrollo
├─ Makefile                # Comandos de desarrollo
├─ README.md               # Documentación principal
├─ PROGRESS.md             # Seguimiento del proyecto
├─ .cursorrules            # Reglas para Cursor
├─ backend/                # Crate Axum (proxy/API)
│  ├─ Cargo.toml
│  ├─ src/
│  │  ├─ main.rs           # Gateway principal con proxy
│  │  ├─ handlers/         # Manejadores de rutas
│  │  ├─ middleware/       # CORS, auth, logging
│  │  └─ config/           # Configuración
│  └─ tests/               # Tests de integración
└─ frontend/               # Crate Yew (WASM/SPA)
   ├─ Cargo.toml
   ├─ index.html           # HTML base con Bulma CSS + CSP
   └─ src/
      ├─ main.rs           # Entry point Yew
      ├─ app.rs            # Componente raíz
      ├─ components/       # UI reutilizable
      │  ├─ mod.rs
      │  ├─ info_card.rs
      │  └─ common/        # Componentes comunes
      ├─ services/         # HTTP, API calls
      │  ├─ mod.rs
      │  └─ api.rs
      ├─ domain/           # DTOs, tipos, mappers
      │  ├─ mod.rs
      │  ├─ types.rs
      │  └─ mappers.rs
      ├─ stores/           # Estado global
      │  ├─ mod.rs
      │  └─ portfolio.rs
      ├─ routes/           # Páginas/vistas
      │  ├─ mod.rs
      │  ├─ dashboard.rs
      │  └─ asset.rs
      └─ utils/            # Helpers, errores
         ├─ mod.rs
         ├─ format.rs
         └─ errors.rs
```

## 🔑 Variables de Entorno

Copia `.env.example` a `.env` y configura tus valores:

```bash
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/ghost_dev
INBESTIA_API_URL=http://localhost:8080
INBESTIA_API_KEY=CHANGEME
BIND_ADDR=127.0.0.1:8085
CORS_ALLOWED_ORIGINS=http://127.0.0.1:3001
```

## 🚀 Comandos de Desarrollo

### Usando Makefile (recomendado)
```bash
# Formatear código
make format

# Linting
make lint

# Tests
make test

# Ejecutar backend
make run-backend

# Ejecutar frontend
make run-frontend
```

### Comandos manuales
```bash
# Terminal 1 - Backend
cd ghost/backend
cargo run

# Terminal 2 - Frontend  
cd ghost/frontend
trunk serve --open
```

## 🔒 Seguridad

- **NUNCA exponer credenciales en el frontend (WASM)**
- **Prohibido INBESTIA_API_KEY en frontend y commits**
- Usar proxy backend para todas las llamadas autenticadas
- Variables sensibles solo en `.env` (backend)
- Frontend solo recibe URLs base, no claves
- **CORS: permissive solo en dev. En prod: CORS_ALLOWED_ORIGINS**
- **Passthrough público solo: /, /health, /api/v1/info**

## 🌐 Puertos del Sistema

- **Frontend (Yew)**: http://127.0.0.1:3001
- **Backend (Axum)**: http://127.0.0.1:8085 
- **inBestia API**: http://127.0.0.1:8080

## 🧪 Testing

```bash
# Tests completos
cargo test --workspace

# Solo backend
cd backend && cargo test

# Solo frontend
cd frontend && cargo test
```

## 📋 CI y Calidad

```bash
# Comandos obligatorios
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## 🛠️ Desarrollo

### Nuevo componente Yew
```bash
# Cursor puede generar automáticamente usando .cursorrules
# Crear frontend/src/components/{name}.rs como function_component
```

### Nuevo handler Axum
```bash
# Cursor puede generar automáticamente usando .cursorrules
# Crear handler con timeout, retry y mapeo de errores
```

## 📚 Documentación

- **README.md**: Documentación principal
- **PROGRESS.md**: Seguimiento del proyecto
- **.cursorrules**: Reglas y estándares para Cursor
- **docs/**: Documentación detallada (futuro)

## 🔄 Flujo de Datos

1. **Frontend (WASM)** → **Backend (Rust)** → **inBestia API**
2. Las credenciales nunca llegan al navegador
3. El backend actúa como proxy seguro
4. Todas las llamadas pasan por `/api/*`

## 📝 Notas Importantes

- El frontend es WASM, no puede acceder directamente a APIs externas
- Todas las llamadas pasan por el proxy backend
- Las credenciales nunca llegan al navegador
- Usar Bulma para UI consistente y responsive
- Seguir principios de Rust: ownership, borrowing, lifetimes
- **Frontend habla solo con /api/*. Nada directo a proveedores**
- **Cache corto y métricas pueden agregarse luego en middleware/**
- **Mantener README.md y PROGRESS.md al día**