# 👨‍💻 Guía para Desarrolladores Junior - Ghost Dashboard

## 🎯 ¿Qué es este proyecto?

**Ghost Dashboard** es una aplicación web que monitorea la API de inBestia. Está construida con:
- **Frontend**: Rust + Yew (compila a WebAssembly)
- **Backend**: Rust + Axum (API Gateway/Proxy)
- **Estilos**: Bulma CSS
- **Build**: Trunk

## 📁 Estructura del Proyecto

```
ghost/
├── 📄 README.md              # Documentación principal del proyecto
├── 📄 PROGRESS.md            # Seguimiento de cambios y progreso
├── 📄 .cursorrules           # Reglas de desarrollo para Cursor IDE
├── 📄 .env                   # Variables de entorno (NO subir a git)
├── 📄 .env.example           # Plantilla de variables de entorno
├── 📄 Cargo.toml             # Configuración del workspace Rust
├── 📄 Trunk.toml             # Configuración del servidor de desarrollo
├── 📄 Makefile               # Comandos útiles para desarrollo
├── 📁 backend/               # Servidor API Gateway
└── 📁 frontend/              # Aplicación web (WASM)
```

---

## 🔧 Backend (API Gateway)

### 📍 Ubicación: `backend/`

### 🎯 ¿Para qué sirve?
El backend actúa como un **proxy/gateway** que:
- Recibe peticiones del frontend
- Las reenvía a la API de inBestia con autenticación
- Devuelve las respuestas al frontend
- **NUNCA expone las claves API al navegador**

### 📁 Estructura del Backend:
```
backend/
├── 📄 Cargo.toml             # Dependencias del backend
├── 📁 src/
│   ├── 📄 main.rs            # ⭐ PUNTO DE ENTRADA - Aquí empieza todo
│   ├── 📁 handlers/          # Manejadores de rutas (futuro)
│   ├── 📁 middleware/        # CORS, auth, logging (futuro)
│   └── 📁 config/            # Configuración (futuro)
└── 📁 tests/                 # Tests de integración
```

### 🔑 Archivos Importantes:

#### `backend/src/main.rs` - **EL ARCHIVO MÁS IMPORTANTE**
```rust
// Aquí se definen:
// - Las rutas de la API
// - La configuración del servidor
// - Los handlers (funciones que manejan peticiones)
// - CORS y middleware
```

### 🚀 Cómo ejecutar el backend:
```bash
cd backend
cargo run
# Se ejecuta en: http://127.0.0.1:8085
```

---

## 🎨 Frontend (Aplicación Web)

### 📍 Ubicación: `frontend/`

### 🎯 ¿Para qué sirve?
El frontend es la **interfaz de usuario** que:
- Muestra datos de la API de inBestia
- Permite navegar entre páginas
- Cambia entre tema claro/oscuro
- Hace peticiones al backend (nunca directamente a inBestia)

### 📁 Estructura del Frontend:
```
frontend/
├── 📄 Cargo.toml             # Dependencias del frontend
├── 📄 index.html             # ⭐ HTML base de la aplicación
├── 📁 src/
│   ├── 📄 main.rs            # ⭐ PUNTO DE ENTRADA
│   ├── 📄 app.rs             # Componente raíz de la aplicación
│   ├── 📁 components/        # 🧩 Componentes reutilizables
│   │   ├── 📄 mod.rs         # Lista de componentes
│   │   ├── 📄 layout.rs      # Layout principal (sidebar + contenido)
│   │   ├── 📄 info_card.rs   # Card de información de API
│   │   ├── 📄 api_status_card.rs # Card de estado de API
│   │   └── 📄 system_metrics_card.rs # Card de métricas del sistema
│   ├── 📁 routes/            # 🛣️ Páginas de la aplicación
│   │   ├── 📄 mod.rs         # Lista de rutas
│   │   ├── 📄 route.rs       # Definición de rutas
│   │   ├── 📄 dashboard.rs   # Página Home
│   │   └── 📄 asset.rs       # Páginas API Status y Asset Analysis
│   ├── 📁 services/          # 🌐 Llamadas a APIs
│   │   ├── 📄 mod.rs         # Lista de servicios
│   │   └── 📄 api.rs         # Funciones para llamar al backend
│   ├── 📁 domain/            # 📊 Tipos de datos y estructuras
│   │   ├── 📄 mod.rs         # Lista de tipos
│   │   └── 📄 types.rs       # Structs para datos de la API
│   ├── 📁 stores/            # 💾 Estado global (futuro)
│   ├── 📁 utils/             # 🛠️ Funciones auxiliares
│   └── 📄 styles.css         # 🎨 Estilos personalizados
└── 📁 dist/                  # Archivos compilados (generado automáticamente)
```

### 🚀 Cómo ejecutar el frontend:
```bash
cd frontend
trunk serve --port 3001 --open
# Se ejecuta en: http://127.0.0.1:3001
```

---

## 🛠️ Cómo Hacer Modificaciones

### 1. 🆕 Agregar una Nueva Ruta

#### Paso 1: Definir la ruta en `frontend/src/routes/route.rs`
```rust
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/api-status")]
    ApiStatus,
    #[at("/asset-analysis")]
    AssetAnalysis,
    #[at("/nueva-pagina")]  // ← AGREGAR AQUÍ
    NuevaPagina,            // ← AGREGAR AQUÍ
    #[not_found]
    #[at("/404")]
    NotFound,
}
```

#### Paso 2: Crear el componente en `frontend/src/routes/`
```rust
// Crear archivo: frontend/src/routes/nueva_pagina.rs
use yew::prelude::*;

#[function_component]
pub fn NuevaPagina() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{ "Nueva Página" }</h1>
            <p>{ "Contenido de la nueva página" }</p>
        </div>
    }
}
```

#### Paso 3: Agregar a `frontend/src/routes/mod.rs`
```rust
pub mod dashboard;
pub mod asset;
pub mod route;
pub mod nueva_pagina;  // ← AGREGAR AQUÍ

pub use dashboard::Home;
pub use asset::{ApiStatus, AssetAnalysis};
pub use nueva_pagina::NuevaPagina;  // ← AGREGAR AQUÍ
pub use route::Route;
```

#### Paso 4: Agregar al switch en `frontend/src/components/layout.rs`
```rust
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::ApiStatus => html! { <ApiStatus /> },
        Route::AssetAnalysis => html! { <AssetAnalysis /> },
        Route::NuevaPagina => html! { <NuevaPagina /> },  // ← AGREGAR AQUÍ
        Route::NotFound => html! { /* ... */ },
    }
}
```

#### Paso 5: Agregar enlace en el menú (opcional)
En `frontend/src/components/layout.rs`, en la sección del menú:
```rust
<li>
    <Link<Route> to={Route::NuevaPagina} classes={if current_route == Route::NuevaPagina { "is-active" } else { "" }}>
        <span>{ "Nueva Página" }</span>
    </Link<Route>>
</li>
```

---

### 2. 🧩 Agregar un Nuevo Card/Componente

#### Paso 1: Crear el componente en `frontend/src/components/`
```rust
// Crear archivo: frontend/src/components/mi_card.rs
use yew::prelude::*;

#[function_component]
pub fn MiCard() -> Html {
    html! {
        <div class="card">
            <div class="card-content">
                <h3 class="title is-5">{ "Mi Card" }</h3>
                <p>{ "Contenido del card" }</p>
            </div>
        </div>
    }
}
```

#### Paso 2: Agregar a `frontend/src/components/mod.rs`
```rust
pub mod info_card;
pub mod layout;
pub mod api_status_card;
pub mod system_metrics_card;
pub mod mi_card;  // ← AGREGAR AQUÍ
pub mod common;
```

#### Paso 3: Usar el componente en una página
```rust
// En cualquier archivo de ruta, por ejemplo frontend/src/routes/asset.rs
use crate::components::mi_card::MiCard;

// En el HTML:
<div class="column is-full">
    <MiCard />
</div>
```

---

### 3. 🌐 Consumir una Nueva API

#### Paso 1: Agregar el tipo de datos en `frontend/src/domain/types.rs`
```rust
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MiNuevoTipo {
    pub campo1: String,
    pub campo2: i32,
    pub campo3: Option<String>,
}
```

#### Paso 2: Crear función de API en `frontend/src/services/api.rs`
```rust
use crate::domain::types::MiNuevoTipo;

pub async fn fetch_mi_dato() -> Result<MiNuevoTipo, String> {
    Request::get("http://127.0.0.1:8085/api/mi-endpoint")
        .send().await.map_err(|e| e.to_string())?
        .json::<MiNuevoTipo>().await.map_err(|e| e.to_string())
}
```

#### Paso 3: Agregar endpoint en el backend `backend/src/main.rs`
```rust
// En la función main(), agregar la ruta:
.route("/api/mi-endpoint", get(mi_handler))

// Crear el handler:
async fn mi_handler(State(state): State<AppState>) -> impl IntoResponse {
    // Lógica para obtener datos de inBestia
    // y devolverlos al frontend
}
```

#### Paso 4: Usar en un componente
```rust
use crate::services::api::fetch_mi_dato;

#[function_component]
pub fn MiComponente() -> Html {
    let data = use_state(|| None::<MiNuevoTipo>);
    
    use_effect(move || {
        let data = data.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match fetch_mi_dato().await {
                Ok(result) => data.set(Some(result)),
                Err(e) => log::error!("Error: {}", e),
            }
        });
        || {}
    });
    
    html! {
        <div>
            {if let Some(d) = (*data).as_ref() {
                html! { <p>{ &d.campo1 }</p> }
            } else {
                html! { <p>{ "Cargando..." }</p> }
            }}
        </div>
    }
}
```

---

### 4. 🎨 Modificar Estilos

#### Archivo principal: `frontend/src/styles.css`

#### Para cambiar colores del tema:
```css
/* Tema claro */
html:not(.is-dark) {
  background-color: #ffffff !important;
  color: #363636 !important;
}

/* Tema oscuro */
html.is-dark {
  background-color: #0a0a0a !important;
  color: #f5f5f5 !important;
}
```

#### Para cambiar font size:
```css
/* Tema claro */
.app:not(.is-dark) {
  font-size: 14px; /* Cambiar aquí */
}

/* Tema oscuro */
.app.is-dark {
  font-size: 14px; /* Cambiar aquí */
}
```

#### Para agregar estilos a un componente específico:
```css
/* En styles.css */
.mi-componente {
  background-color: #f0f0f0;
  padding: 1rem;
  border-radius: 8px;
}

/* En el componente Rust */
<div class="mi-componente">
  { "Mi contenido" }
</div>
```

---

### 5. 🔧 Comandos Útiles

#### Ejecutar todo el proyecto:
```bash
# Terminal 1 - Backend
cd backend && cargo run

# Terminal 2 - Frontend
cd frontend && trunk serve --port 3001 --open
```

#### Verificar que compile:
```bash
# Backend
cd backend && cargo check

# Frontend
cd frontend && cargo check
```

#### Limpiar y reconstruir:
```bash
# Limpiar todo
cargo clean

# Reconstruir
cargo build
```

---

## 🚨 Errores Comunes y Soluciones

### ❌ "No such file or directory" al ejecutar Trunk
**Problema**: Ejecutar Trunk desde el directorio raíz
**Solución**: Siempre ejecutar desde `frontend/`
```bash
cd frontend
trunk serve --port 3001 --open
```

### ❌ "Expected a closure that implements Fn trait"
**Problema**: Error en closures de Yew
**Solución**: Usar `Rc` para compartir estado
```rust
let data = Rc::new(data.clone());
Callback::from(move |_| {
    let data = data.clone();
    // ... resto del código
})
```

### ❌ Página no carga
**Problema**: Error en el routing o componente
**Solución**: 
1. Verificar que compile: `cargo check`
2. Verificar que la ruta esté en `route.rs`
3. Verificar que el componente esté en `mod.rs`
4. Verificar que esté en el `switch`

### ❌ API no responde
**Problema**: Backend no está corriendo o hay error de conexión
**Solución**:
1. Verificar que el backend esté corriendo en puerto 8085
2. Verificar que la API de inBestia esté corriendo en puerto 8080
3. Verificar las variables en `.env`

---

## 📚 Recursos Adicionales

### Documentación oficial:
- [Yew](https://yew.rs/) - Framework de frontend
- [Axum](https://docs.rs/axum/) - Framework de backend
- [Bulma CSS](https://bulma.io/) - Framework de estilos
- [Trunk](https://trunkrs.dev/) - Build tool

### Estructura de archivos importante:
- `main.rs` - Punto de entrada (tanto frontend como backend)
- `mod.rs` - Lista de módulos
- `route.rs` - Definición de rutas
- `types.rs` - Tipos de datos
- `api.rs` - Llamadas a APIs
- `styles.css` - Estilos personalizados

### Flujo de datos:
1. **Frontend** hace petición a **Backend**
2. **Backend** reenvía petición a **API inBestia** con autenticación
3. **API inBestia** responde al **Backend**
4. **Backend** responde al **Frontend**
5. **Frontend** muestra los datos en la UI

---

## 🎯 Resumen Rápido

- **Nueva ruta**: `route.rs` → crear componente → `mod.rs` → `layout.rs`
- **Nuevo card**: crear componente → `mod.rs` → usar en página
- **Nueva API**: `types.rs` → `api.rs` → `main.rs` (backend) → usar en componente
- **Estilos**: `styles.css`
- **Ejecutar**: `cd frontend && trunk serve` y `cd backend && cargo run`

¡Recuerda: siempre verificar que compile con `cargo check` antes de probar!
