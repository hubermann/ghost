# 🚀 Plan de Migración: inBestia Dashboard → Ghost

**Fecha de creación**: 2025-01-27
**Estado**: Planificación completada
**Autor**: Análisis automatizado del ecosistema inBestia

## 📋 **Resumen Ejecutivo**

Este documento detalla el plan completo para migrar funcionalidades del dashboard legacy (Rails + React) al nuevo dashboard Ghost (Rust + Yew). La migración se enfoca en preservar las funcionalidades críticas mientras se moderniza la arquitectura.

---

## 🎯 **OBJETIVOS PRINCIPALES**

### **Objetivos Técnicos**
- [ ] Migrar Asset Analysis Multi-Temporal (funcionalidad crown jewel)
- [ ] Implementar sistema de métricas unificado
- [ ] Resolver fragmentación de timeframes en el ecosistema
- [ ] Modernizar stack tecnológico (Rails→Rust, React→Yew)

### **Objetivos de Negocio**
- [ ] Mantener continuidad operacional durante migración
- [ ] Mejorar performance (WASM vs JavaScript)
- [ ] Reducir complejidad de mantenimiento
- [ ] Establecer base sólida para features futuras

---

## 🏗️ **ANÁLISIS DEL ESTADO ACTUAL**

### **inBestia Dashboard (Legacy) - Funcionalidades Identificadas**

#### **🔥 Funcionalidades Críticas**
- **Asset Analysis Multi-Temporal**: Análisis en 7 timeframes simultáneos
- **Indicadores Técnicos**: 26 indicadores tradicionales + 5 ICT avanzados
- **Sistema de Confluencia**: Algoritmo matemático complejo (-100 a +100)
- **Métricas del Sistema**: Health checks, latencia, limits de API

#### **📊 Funcionalidades Importantes**
- **Dashboard Principal**: Overview general del sistema
- **Análisis de Calidad**: Discrepancias entre proveedores
- **Monitoreo en Tiempo Real**: Alertas y actividad reciente
- **Análisis Financiero**: Puntuaciones y comparativas

#### **🎨 Funcionalidades Secundarias**
- **Sistema de Temas**: Dark/Light mode
- **Navegación Turbo**: Performance optimizations
- **Sistema de Caché**: Optimizaciones de rendimiento

### **Ghost (Nuevo) - Estado Actual**

#### **✅ Implementado**
- Arquitectura base Rust + Yew
- Proxy backend seguro (Axum)
- Layout básico con Bulma CSS
- Componentes base: InfoCard, SystemMetricsCard
- TimeframeSelector parcial
- Estructura modular organizada

#### **🔄 En Desarrollo**
- AssetAnalysisCard básico
- Integración completa con inBestia API
- Sistema de routing avanzado

---

## ⚠️ **PROBLEMA CRÍTICO IDENTIFICADO**

### **Fragmentación de Timeframes**

**Descripción**: Inconsistencia total en manejo de timeframes entre servicios del ecosistema.

**Evidencia**:
- **inBestia (Rust)**: `Timeframe::Hour1`, `Timeframe::Daily` (enum)
- **Dashboard (Rails)**: `"1h"`, `"4h"`, `"1d"` (strings hardcodeados)
- **inBestia_ML (Rust)**: Parsing custom inconsistente
- **Macarena (Python)**: Sin estándar definido

**Impacto**:
- MultiTemporalAnalysis falla (solo 5min funciona)
- 40-60% más requests innecesarios
- Cache hit rate bajo
- Debugging complejo

**Requisito**: **DEBE resolverse ANTES de cualquier migración**

---

## 📅 **PLAN DE EJECUCIÓN DETALLADO**

## **PRE-REQUISITO ABSOLUTO**

### **Semana 0: Resolución de Fragmentación de Timeframes**

> ⚠️ **CRÍTICO**: Esta fase DEBE completarse antes de proceder con cualquier migración

#### **Tarea P.1: Auditoría Completa de Timeframes**
- [ ] **inBestia Core**: Documentar enum `Timeframe` y todos sus parsing methods
- [ ] **Dashboard Rails**: Mapear todos los hardcoded timeframe strings
- [ ] **inBestia_ML**: Revisar lógica de parsing custom
- [ ] **Macarena**: Identificar cómo maneja timeframes actualmente
- [ ] Crear documento de inconsistencias encontradas

#### **Tarea P.2: Diseño de Estándar Unificado**
- [ ] Definir enum canónico en inBestia core
- [ ] Especificar aliases soportados (`1h`, `hour1`, `1hour`, etc.)
- [ ] Crear metadata: duración, peso, categoría (short/medium/long term)
- [ ] Diseñar JSON schema exportable a otros servicios

#### **Tarea P.3: Implementación del Estándar**
- [ ] **inBestia**: Crear endpoint `/api/v1/timeframes/config`
- [ ] **inBestia**: Consolidar parsing y validación
- [ ] **Dashboard**: Crear `TimeframeService` centralizado
- [ ] **inBestia_ML**: Adoptar timeframes de inBestia core
- [ ] **Macarena**: Implementar dataclass/enum para timeframes

#### **Tarea P.4: Validación Cross-Service**
- [ ] Tests de integración entre todos los servicios
- [ ] Verificación de que MultiTemporalAnalysis funciona con todos los timeframes
- [ ] Performance testing del nuevo sistema
- [ ] Rollback plan si algo falla

**Entregables**:
- [ ] Documento de estándar de timeframes
- [ ] Endpoint `/api/v1/timeframes/config` funcionando
- [ ] MultiTemporalAnalysis working con 7 timeframes
- [ ] Tests de integración pasando

---

## **MIGRACIÓN PRINCIPAL**

### **FASE 1: Foundation (Semanas 1-2)**

#### **Tarea 1.1: Setup Inicial en Ghost**
- [ ] Configurar estructura de datos para análisis técnico
- [ ] Crear types/structs para todos los indicadores técnicos
- [ ] Implementar client HTTP para comunicación con inBestia API
- [ ] Setup logging y error handling robusto

#### **Tarea 1.2: Migrar Indicadores Básicos**
- [ ] **RSI (Relative Strength Index)**
  - [ ] Struct para datos RSI
  - [ ] Lógica de interpretación (overbought/oversold)
  - [ ] Componente visual con progress bar
- [ ] **MACD**
  - [ ] Struct para line, signal, histogram
  - [ ] Interpretación de señales buy/sell
  - [ ] Visualización básica
- [ ] **Bollinger Bands**
  - [ ] Struct para upper, middle, lower
  - [ ] Lógica de posición (above/below/inside)

#### **Tarea 1.3: Asset Analysis Card Básico**
- [ ] Crear `AssetAnalysisCard` component completo
- [ ] Input para búsqueda de símbolos
- [ ] Display de indicadores básicos
- [ ] Sistema de tabs para diferentes vistas
- [ ] Error handling y loading states

**Entregables**:
- [ ] AssetAnalysisCard funcional con indicadores básicos
- [ ] Tests unitarios para componentes
- [ ] Documentación de nuevos components

### **FASE 2: Multi-Temporal Core (Semanas 3-4)**

#### **Tarea 2.1: Infraestructura Multi-Temporal**
- [ ] Crear `MultiTemporalAnalyzer` service
- [ ] Implementar batch requests para múltiples timeframes
- [ ] Sistema de caché inteligente para timeframes
- [ ] Parallel processing de múltiples timeframes

#### **Tarea 2.2: Algoritmo de Confluencia**
- [ ] **Migrar lógica matemática** del Rails dashboard
- [ ] Implementar cálculo de score (-100 a +100)
- [ ] Sistema de pesos por timeframe
- [ ] Interpretación automática de señales

#### **Tarea 2.3: Visualización Multi-Temporal**
- [ ] Componente `MultiTemporalView`
- [ ] Cards por timeframe con colores según tendencia
- [ ] Display de confluencia score prominente
- [ ] Interpretación educativa de resultados

**Entregables**:
- [ ] MultiTemporalAnalysis completamente funcional
- [ ] Performance igual o mejor que versión Rails
- [ ] Tests de todos los timeframes configurados

### **FASE 3: Indicadores Avanzados (Semanas 5-6)**

#### **Tarea 3.1: Indicadores ICT/Smart Money**
- [ ] **Fibonacci Retracements**
  - [ ] Struct para swing high/low, trend direction
  - [ ] Cálculo de niveles (23.6%, 38.2%, 50%, 61.8%, 78.6%)
  - [ ] Visualización de niveles más cercanos
- [ ] **Fair Value Gaps (FVG)**
  - [ ] Struct para bullish/bearish gaps
  - [ ] Estado: unfilled, partially_filled, filled
  - [ ] Strength assessment
- [ ] **Order Blocks**
  - [ ] Struct para bullish/bearish blocks
  - [ ] Strength: fresh, tested, broken
  - [ ] Timeframe association

#### **Tarea 3.2: Indicadores de Momentum Avanzados**
- [ ] **ADX (Average Directional Index)**
  - [ ] Struct para ADX value, +DI, -DI
  - [ ] Interpretación de fuerza de tendencia
  - [ ] Visualización con tags dinámicos
- [ ] **Stochastic (K y D)**
- [ ] **Williams %R**
- [ ] **CCI (Commodity Channel Index)**

#### **Tarea 3.3: Indicadores de Volumen**
- [ ] **OBV (On-Balance Volume)**
- [ ] **VWAP (Volume Weighted Average Price)**
- [ ] **Accumulation/Distribution Line**

**Entregables**:
- [ ] Todos los indicadores del dashboard legacy implementados
- [ ] Paridad funcional completa
- [ ] Performance benchmarks vs versión original

### **FASE 4: Dashboard Principal (Semanas 7-8)**

#### **Tarea 4.1: Dashboard Overview**
- [ ] Migrar layout principal del dashboard
- [ ] Cards de resumen del sistema
- [ ] Health checks y status indicators
- [ ] Recent activity feed

#### **Tarea 4.2: Sistema de Métricas**
- [ ] Completar `SystemMetricsCard`
- [ ] API limits monitoring
- [ ] Latency metrics
- [ ] Provider status indicators

#### **Tarea 4.3: Navegación y Routing**
- [ ] Setup completo de yew-router
- [ ] Navegación entre secciones
- [ ] Breadcrumbs y navigation state
- [ ] URL persistence para análisis

**Entregables**:
- [ ] Dashboard principal completamente funcional
- [ ] Navegación fluida entre secciones
- [ ] Monitoring básico operacional

### **FASE 5: Features Avanzados (Semanas 9-10)**

#### **Tarea 5.1: Análisis de Calidad de Datos**
- [ ] Data quality metrics visualization
- [ ] Provider comparison tools
- [ ] Discrepancy detection y alerting
- [ ] Reconciliation metrics

#### **Tarea 5.2: Monitoreo en Tiempo Real**
- [ ] Real-time updates sistema
- [ ] Alerting system
- [ ] WebSocket integration (si requerido)
- [ ] Activity logging y audit trail

#### **Tarea 5.3: Análisis Financiero Extendido**
- [ ] Portfolio analysis tools
- [ ] Advanced scoring algorithms
- [ ] Comparative analysis features
- [ ] Export/reporting capabilities

**Entregables**:
- [ ] Feature parity completa con dashboard legacy
- [ ] Advanced analytics operacional
- [ ] Sistema de reportes funcional

### **FASE 6: Polish y Optimización (Semanas 11-12)**

#### **Tarea 6.1: UI/UX Enhancements**
- [ ] Implementar dark/light theme toggle
- [ ] Responsive design optimization
- [ ] Accessibility improvements
- [ ] Loading states y animations

#### **Tarea 6.2: Performance Optimization**
- [ ] WASM bundle size optimization
- [ ] Lazy loading de componentes
- [ ] Cache strategies optimization
- [ ] Performance monitoring setup

#### **Tarea 6.3: Testing y QA**
- [ ] Comprehensive integration tests
- [ ] Performance regression tests
- [ ] User acceptance testing
- [ ] Documentation completion

**Entregables**:
- [ ] Producto production-ready
- [ ] Performance superior al legacy dashboard
- [ ] Documentación completa

---

## 🛠️ **REQUISITOS TÉCNICOS**

### **Dependencias de Software**
- [ ] Rust 1.75+ con Cargo
- [ ] wasm-pack para compilación WASM
- [ ] trunk para development server
- [ ] Node.js (para tooling auxiliar)

### **Configuración de Entorno**
```bash
# Variables de entorno requeridas
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/ghost_dev
INBESTIA_API_URL=http://localhost:8080
INBESTIA_API_KEY=inbestia2025key
BIND_ADDR=127.0.0.1:8085
CORS_ALLOWED_ORIGINS=http://127.0.0.1:3001
```

### **Servicios Externos**
- [ ] inBestia API (puerto 8080) - DEBE estar funcionando
- [ ] PostgreSQL database
- [ ] Redis (para caché, opcional pero recomendado)

---

## 📊 **MÉTRICAS DE ÉXITO**

### **Métricas Técnicas**
- [ ] **Performance**: Tiempo de carga ≤ 2s (vs ~3-4s legacy)
- [ ] **Bundle Size**: WASM bundle ≤ 2MB
- [ ] **Memory Usage**: ≤ 50MB RAM en browser
- [ ] **API Calls**: Reducir 30% requests vs legacy (gracias a mejor caché)

### **Métricas Funcionales**
- [ ] **Feature Parity**: 100% funcionalidades críticas migradas
- [ ] **Data Accuracy**: 100% paridad en cálculos vs legacy
- [ ] **Uptime**: 99.9% availability durante migración
- [ ] **User Experience**: ≤ 2 clicks para cualquier análisis

### **Métricas de Desarrollo**
- [ ] **Test Coverage**: ≥ 80% code coverage
- [ ] **Build Time**: ≤ 30s para rebuilds completos
- [ ] **Documentation**: 100% endpoints y components documentados

---

## 🚨 **RIESGOS Y MITIGACIONES**

### **Riesgo Alto: Algoritmo de Confluencia Complejo**
- **Descripción**: La lógica matemática del MultiTemporalAnalysis es compleja
- **Mitigación**:
  - [ ] Extraer lógica exacta del Rails code
  - [ ] Crear tests unitarios exhaustivos
  - [ ] Validar resultados side-by-side con legacy

### **Riesgo Medio: Performance de WASM**
- **Descripción**: WASM puede ser slower para ciertos cálculos
- **Mitigación**:
  - [ ] Benchmarking early y frequent
  - [ ] Optimización específica de hot paths
  - [ ] Fallback a JavaScript si necesario

### **Riesgo Medio: Timeframes Standardization**
- **Descripción**: El pre-requisito puede tomar más tiempo
- **Mitigación**:
  - [ ] Start con subset de timeframes working
  - [ ] Parallel development donde posible
  - [ ] Rollback plan detallado

### **Riesgo Bajo: Learning Curve Yew**
- **Descripción**: Team puede requerir tiempo para adoptar Yew
- **Mitigación**:
  - [ ] Training sessions planificadas
  - [ ] Documentation y examples
  - [ ] Code reviews frecuentes

---

## 📋 **CHECKLIST DE PREPARACIÓN**

### **Antes de Comenzar**
- [ ] ✅ Análisis del ecosistema completado
- [ ] ✅ Plan de migración aprobado
- [ ] [ ] Team training en Rust/Yew completado
- [ ] [ ] Environment setup verificado
- [ ] [ ] Backup del dashboard legacy creado

### **Pre-Requisito Critical Path**
- [ ] [ ] Fragmentación de timeframes resuelta
- [ ] [ ] MultiTemporalAnalysis working en legacy
- [ ] [ ] Tests de integración cross-service passing
- [ ] [ ] Performance baseline establecido

### **Ready for Migration**
- [ ] [ ] Todas las dependencias resueltas
- [ ] [ ] Development environment configured
- [ ] [ ] CI/CD pipeline setup
- [ ] [ ] Monitoring y alerting configurado

---

## 📚 **RECURSOS Y REFERENCIAS**

### **Documentación Existente**
- `inbestia-ecosystem/README.md` - Arquitectura general
- `inbestia_dashboard/README.md` - Funcionalidades legacy
- `inbestia_dashboard/refactor_temporalidades.md` - Problema timeframes
- `ghost/README.md` - Arquitectura nuevo dashboard

### **APIs y Endpoints**
- `inbestia-ecosystem/API_ENDPOINTS_REFERENCE.md`
- `/api/v1/analysis/technical` - Endpoint principal
- `/api/v1/timeframes/config` - A crear en pre-requisito

### **Herramientas de Desarrollo**
- **Makefile** en Ghost para comandos comunes
- **Scripts** de verificación en `ghost/scripts/`
- **Pre-commit hooks** para calidad de código

---

## 🔄 **PROCESO DE REVISIÓN**

### **Revisiones Semanales**
- [ ] Review de progress vs plan
- [ ] Identificación de blockers
- [ ] Ajuste de timeline si necesario
- [ ] Testing de funcionalidades completadas

### **Milestones de Validación**
- [ ] **Semana 0**: Pre-requisito completado
- [ ] **Semana 2**: Foundation solid
- [ ] **Semana 4**: Multi-temporal working
- [ ] **Semana 6**: Feature parity básica
- [ ] **Semana 8**: Dashboard completo
- [ ] **Semana 12**: Production ready

### **Criterios de Go/No-Go**
- Cada fase requiere approval para continuar
- Performance benchmarks deben ser met
- Critical functionality debe estar working
- No regressions vs legacy permitidas

---

## ✅ **PRÓXIMOS PASOS INMEDIATOS**

1. **[ ] Revisar y aprobar este plan**
2. **[ ] Comenzar con Semana 0: Resolver fragmentación timeframes**
3. **[ ] Setup development environment para Ghost**
4. **[ ] Crear branching strategy para migration work**
5. **[ ] Schedule team training sessions**

---

**Contacto**: Para questions sobre este plan o clarificaciones técnicas
**Última actualización**: 2025-01-27
**Próxima revisión**: Al completar Semana 0