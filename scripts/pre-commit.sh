#!/bin/bash

# Ghost Dashboard - Pre-commit Quality Check
# Ejecutar antes de cada commit para verificar calidad del código

set -e

echo "🔍 Ghost Dashboard - Pre-commit Quality Check"
echo "=============================================="

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Función para imprimir mensajes
print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Verificar que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ] || [ ! -f "README.md" ]; then
    print_error "No se encontró el proyecto Ghost Dashboard. Ejecutar desde la raíz del proyecto."
    exit 1
fi

echo ""
echo "📋 Verificando Backend..."
echo "------------------------"

# Backend checks
cd backend

# 1. Formato de código
echo "1. Verificando formato de código..."
if cargo fmt --all -- --check; then
    print_success "Formato de código correcto"
else
    print_error "Formato de código incorrecto. Ejecutar: cargo fmt --all"
    exit 1
fi

# 2. Clippy
echo "2. Ejecutando clippy..."
if cargo clippy --all-targets -- -D warnings; then
    print_success "Clippy passed"
else
    print_error "Clippy encontró warnings. Corregir antes de commit."
    exit 1
fi

# 3. Tests
echo "3. Ejecutando tests..."
if cargo test --all-targets; then
    print_success "Tests pasaron"
else
    print_error "Tests fallaron. Corregir antes de commit."
    exit 1
fi

# 4. Verificar hardcoded values
echo "4. Verificando valores hardcodeados..."
if grep -r "localhost:8080" src/; then
    print_error "Encontrado localhost:8080 hardcodeado - usar variables de entorno"
    exit 1
fi

if grep -r "127.0.0.1:8080" src/; then
    print_error "Encontrado 127.0.0.1:8080 hardcodeado - usar variables de entorno"
    exit 1
fi

# 5. Verificar headers de seguridad
echo "5. Verificando headers de seguridad..."
if ! grep -r "X-Content-Type-Options" src/; then
    print_error "Headers de seguridad no implementados"
    exit 1
fi

print_success "Headers de seguridad encontrados"

cd ..

echo ""
echo "📋 Verificando Frontend..."
echo "-------------------------"

# Frontend checks
cd frontend

# 1. Formato de código
echo "1. Verificando formato de código..."
if cargo fmt --all -- --check; then
    print_success "Formato de código correcto"
else
    print_error "Formato de código incorrecto. Ejecutar: cargo fmt --all"
    exit 1
fi

# 2. Clippy
echo "2. Ejecutando clippy..."
if cargo clippy --all-targets -- -D warnings; then
    print_success "Clippy passed"
else
    print_error "Clippy encontró warnings. Corregir antes de commit."
    exit 1
fi

# 3. Tests
echo "3. Ejecutando tests..."
if cargo test --all-targets; then
    print_success "Tests pasaron"
else
    print_error "Tests fallaron. Corregir antes de commit."
    exit 1
fi

# 4. Verificar llamadas directas a API
echo "4. Verificando llamadas directas a API..."
if grep -r "http://localhost:8080" src/; then
    print_error "Encontradas llamadas directas a API - usar proxy /api/*"
    exit 1
fi

if grep -r "http://127.0.0.1:8080" src/; then
    print_error "Encontradas llamadas directas a API - usar proxy /api/*"
    exit 1
fi

# 5. Verificar credenciales expuestas
echo "5. Verificando credenciales expuestas..."
if grep -r "INBESTIA_API_KEY" src/; then
    print_error "Encontrada API key expuesta en frontend"
    exit 1
fi

# 6. Verificar uso de HtmlSelectElement
echo "6. Verificando uso de HtmlSelectElement..."
if grep -r "HtmlSelectElement" src/; then
    print_error "Encontrado uso de HtmlSelectElement - usar HtmlElement con js_sys"
    exit 1
fi

print_success "No se encontraron problemas en frontend"

cd ..

echo ""
echo "📋 Verificando Documentación..."
echo "------------------------------"

# Documentation checks
if [ ! -f "README.md" ]; then
    print_error "README.md faltante"
    exit 1
fi

if [ ! -f "PROGRESS.md" ]; then
    print_error "PROGRESS.md faltante"
    exit 1
fi

if [ ! -f ".env.example" ]; then
    print_error ".env.example faltante"
    exit 1
fi

if [ ! -f "DEVELOPMENT_GUIDELINES.md" ]; then
    print_error "DEVELOPMENT_GUIDELINES.md faltante"
    exit 1
fi

print_success "Documentación completa"

echo ""
echo "📋 Verificando Seguridad..."
echo "--------------------------"

# Security checks
echo "1. Verificando .env en .gitignore..."
if git check-ignore .env; then
    print_success ".env está correctamente en .gitignore"
else
    print_error ".env no está en .gitignore"
    exit 1
fi

echo "2. Verificando CORS permissive..."
if grep -r "CorsLayer::permissive" backend/src/; then
    print_error "Encontrado CORS permissive - usar orígenes específicos"
    exit 1
fi

print_success "No se encontró CORS permissive"

echo ""
echo "📋 Verificando Estructura Modular..."
echo "-----------------------------------"

# Structure checks
if [ ! -d "backend/src/config" ]; then
    print_error "Directorio backend/src/config faltante"
    exit 1
fi

if [ ! -d "backend/src/handlers" ]; then
    print_error "Directorio backend/src/handlers faltante"
    exit 1
fi

if [ ! -d "backend/src/middleware" ]; then
    print_error "Directorio backend/src/middleware faltante"
    exit 1
fi

print_success "Estructura modular correcta"

echo ""
echo "🎉 ¡Todas las verificaciones pasaron!"
echo "====================================="
print_success "El código está listo para commit"
echo ""
echo "📝 Recordatorios:"
echo "- Mantener documentación actualizada"
echo "- Seguir las reglas en DEVELOPMENT_GUIDELINES.md"
echo "- Usar trace IDs en logs"
echo "- Validar variables de entorno"
echo ""
