# Ghost Dashboard - Development Guidelines

## 🎯 Arquitectura y Estructura

### Backend (Axum Gateway)

#### Estructura Modular Obligatoria
```
backend/src/
├─ main.rs              # Entry point y configuración de rutas
├─ config/              # Configuración centralizada
│  └─ mod.rs           # Validación de variables de entorno
├─ handlers/            # Lógica de endpoints específicos
│  └─ mod.rs           # Health checks, métricas, proxies
├─ middleware/          # Middleware reutilizable
│  └─ mod.rs           # CORS, seguridad, logging
└─ tests/              # Tests de integración
   └─ integration_test.rs
```

#### Reglas de Configuración
- ✅ **SIEMPRE** validar variables de entorno al inicio
- ✅ **SIEMPRE** usar valores por defecto sensatos
- ✅ **SIEMPRE** documentar variables en `.env.example`
- ✅ **NUNCA** hardcodear URLs o claves en el código

```rust
// ✅ CORRECTO
api_base_url: env::var("INBESTIA_API_URL")
    .map_err(|_| anyhow::anyhow!("INBESTIA_API_URL environment variable is required"))?,

// ❌ INCORRECTO
let api_base = "http://localhost:8080".to_string();
```

#### Reglas de Seguridad
- ✅ **SIEMPRE** implementar headers de seguridad
- ✅ **SIEMPRE** limitar tamaño de requests
- ✅ **SIEMPRE** validar CORS por origen específico
- ✅ **NUNCA** usar `CorsLayer::permissive()` en producción

```rust
// ✅ Headers de seguridad obligatorios
headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
```

#### Reglas de Observabilidad
- ✅ **SIEMPRE** usar trace IDs únicos
- ✅ **SIEMPRE** loggear duración de requests
- ✅ **SIEMPRE** implementar health checks detallados
- ✅ **SIEMPRE** incluir métricas del sistema

```rust
// ✅ Logging estructurado obligatorio
let trace_id = Uuid::new_v4().to_string();
info!(
    method = %method,
    uri = %uri,
    status = %status,
    duration_ms = %duration.as_millis(),
    trace_id = %trace_id,
    "request completed"
);
```

### Frontend (Yew + WASM)

#### Estructura de Componentes
```
frontend/src/
├─ components/          # UI reutilizable
│  ├─ mod.rs
│  ├─ common/          # Componentes base
│  └─ [feature]/       # Componentes específicos
├─ services/           # Llamadas HTTP
│  ├─ mod.rs
│  └─ [api].rs        # APIs específicas
├─ domain/             # DTOs y tipos
│  ├─ mod.rs
│  ├─ types.rs
│  └─ mappers.rs
├─ stores/             # Estado global
└─ routes/             # Páginas/vistas
```

#### Reglas de Manejo de Estado
- ✅ **SIEMPRE** usar `function_component` sobre `struct_component`
- ✅ **SIEMPRE** manejar estados de loading/error/success
- ✅ **SIEMPRE** usar `use_state` para estado local
- ✅ **SIEMPRE** usar `use_effect_with` para side effects

```rust
// ✅ Patrón de estado obligatorio
#[derive(Debug, Clone, PartialEq)]
pub enum DataState<T> {
    Loading,
    Loaded(T),
    Error(String),
}
```

#### Reglas de Manejo de Errores
- ✅ **SIEMPRE** usar `Result<T, String>` para errores
- ✅ **SIEMPRE** mostrar errores al usuario
- ✅ **SIEMPRE** incluir fallbacks para errores de red
- ✅ **NUNCA** silenciar errores

```rust
// ✅ Manejo de errores obligatorio
match fetch_data().await {
    Ok(data) => state.set(DataState::Loaded(data)),
    Err(error) => state.set(DataState::Error(error)),
}
```

## 🔧 Patrones de Desarrollo

### Proxy Pattern (Backend)
```rust
// ✅ Patrón de proxy obligatorio
pub async fn proxy_request(
    State(state): State<AppState>,
    method: Method,
    path: &str,
    body: Option<Body>,
) -> impl IntoResponse {
    let trace_id = Uuid::new_v4().to_string();
    
    // 1. Validar request
    // 2. Añadir headers de autenticación
    // 3. Hacer proxy a API externa
    // 4. Mapear errores
    // 5. Loggear con trace_id
}
```

### Error Mapping Pattern
```rust
// ✅ Mapeo de errores obligatorio
fn map_error(error: reqwest::Error, trace_id: &str) -> (StatusCode, Json<serde_json::Value>) {
    match error.status() {
        Some(status) if status.is_client_error() => {
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": "Invalid request",
                "trace_id": trace_id
            })))
        }
        _ => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "Internal server error",
                "trace_id": trace_id
            })))
        }
    }
}
```

### Timeframe Mapping Pattern (Frontend)
```rust
// ✅ Mapeo de timeframes obligatorio
let analysis_timeframe = match (*timeframes_state).clone() {
    TimeframesState::Loaded(configs) => {
        configs.iter()
            .find(|config| config.name == *timeframe)
            .map(|config| config.to_analysis_format())
            .unwrap_or_else(|| (*timeframe).clone())
    }
    _ => (*timeframe).clone(),
};
```

## 🚫 Anti-Patrones Prohibidos

### Backend
- ❌ **NUNCA** hardcodear URLs o puertos
- ❌ **NUNCA** usar `unwrap()` sin manejo de errores
- ❌ **NUNCA** exponer claves en logs
- ❌ **NUNCA** usar `CorsLayer::permissive()` en producción
- ❌ **NUNCA** matar procesos en puerto 8080 (API externa)

### Frontend
- ❌ **NUNCA** exponer credenciales en WASM
- ❌ **NUNCA** hacer llamadas directas a APIs externas
- ❌ **NUNCA** usar `HtmlSelectElement` (no existe en web_sys)
- ❌ **NUNCA** silenciar errores de compilación
- ❌ **NUNCA** hardcodear URLs de API

## 📋 Checklist de Desarrollo

### Antes de cada commit
- [ ] Variables de entorno validadas
- [ ] Headers de seguridad implementados
- [ ] Trace IDs en todos los logs
- [ ] Manejo de errores consistente
- [ ] Tests de integración pasando
- [ ] No hay warnings de compilación
- [ ] Documentación actualizada

### Antes de cada PR
- [ ] Código organizado en módulos
- [ ] Configuración centralizada
- [ ] Observabilidad implementada
- [ ] Seguridad validada
- [ ] Performance optimizada
- [ ] Tests completos
- [ ] README actualizado

## 🔍 Debugging Guidelines

### Backend
1. **Verificar configuración**: `curl http://localhost:8085/health`
2. **Revisar logs**: Buscar trace_id en logs
3. **Verificar conectividad**: `curl http://localhost:8080/health`
4. **Validar variables**: Verificar `.env` vs `.env.example`

### Frontend
1. **Verificar compilación**: Revisar warnings/errors
2. **Verificar red**: Network tab en DevTools
3. **Verificar estado**: Console logs de estado
4. **Verificar mapeo**: Timeframes y formatos correctos

## 📚 Documentación Obligatoria

### Cada módulo debe tener:
- Docstring con propósito
- Ejemplos de uso
- Casos de error
- Dependencias

### Cada endpoint debe tener:
- Descripción de funcionalidad
- Parámetros de entrada
- Respuestas esperadas
- Códigos de error
- Ejemplos de request/response

## 🎯 Métricas de Calidad

### Código
- **Coverage**: >80% en tests
- **Warnings**: 0 warnings de compilación
- **Complexity**: <10 por función
- **Duplication**: <5% de código duplicado

### Performance
- **Response time**: <500ms para endpoints
- **Memory usage**: <100MB para backend
- **Bundle size**: <2MB para frontend
- **Load time**: <3s para frontend

### Seguridad
- **Headers**: Todos los headers de seguridad
- **Validation**: Validación de entrada
- **Authentication**: Headers de auth correctos
- **CORS**: Configuración restrictiva

---

**Nota**: Estas reglas son obligatorias y deben seguirse en todo el desarrollo del proyecto. Cualquier excepción debe ser documentada y justificada.
