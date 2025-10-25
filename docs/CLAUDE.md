# 🦀 MCP .NET Context - Claude Development Guide

## 📋 Información del Proyecto

**Nombre:** MCP .NET Context Server  
**Versión:** 0.1.0  
**Lenguaje:** Rust 🦀  
**Propósito:** MCP especializado para análisis de contexto y entrenamiento de código .NET 10 y Blazor Server

## 🎯 Objetivo Principal

Crear un servidor MCP (Model Context Protocol) en Rust que proporcione a Claude y otras IAs contexto inteligente sobre proyectos .NET 10 y Blazor Server, incluyendo:

1. **Análisis profundo de código C#**
2. **Extracción de patrones**
3. **Sistema de entrenamiento incremental**
4. **Sugerencias contextuales**
5. **Integración con tree-sitter para parsing**

## 🏗️ Arquitectura del Sistema

```
mcp-dotnet-context/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   ├── config.rs            # Configuración
│   ├── types.rs             # Tipos compartidos
│   ├── utils/               # Utilidades
│   ├── analyzer/            # Análisis de código
│   │   ├── mod.rs
│   │   ├── csharp.rs        # Parser C# con tree-sitter
│   │   └── project.rs       # Análisis de .csproj
│   ├── context/             # Generación de contexto
│   │   └── mod.rs
│   ├── training/            # Sistema de entrenamiento
│   │   └── mod.rs
│   └── mcp/                 # Protocolo MCP
│       └── mod.rs
├── data/                    # Datos de entrenamiento
│   └── patterns/
│       └── blazor-server-lifecycle.json
├── tests/                   # Tests
├── Cargo.toml
├── CLAUDE.md               # Este archivo
└── README.md
```

## 🔧 Tecnologías Clave

### Core
- **tokio**: Runtime async
- **serde**: Serialización JSON
- **tree-sitter**: Parsing de C#
- **anyhow/thiserror**: Manejo de errores

### Análisis
- **tree-sitter-c-sharp**: Parser C# AST
- **quick-xml**: Parser .csproj (XML)
- **walkdir/ignore**: Navegación de directorios

### MCP Protocol
- **jsonrpc-core**: Implementación JSON-RPC 2.0
- **stdio**: Transporte para Claude Desktop

## 📊 Fases de Desarrollo

### ✅ Fase 0: Estructura Base (COMPLETADO)
- [x] Crear workspace Rust
- [x] Definir tipos principales
- [x] Estructura de módulos
- [x] Configuración base
- [x] Este documento

### ✅ Fase 1: Análisis de Proyectos .NET (COMPLETADO)

**Objetivo:** Implementar análisis completo de proyectos .NET

#### Tareas Completadas:
1. **Parser de .csproj** ✅
   ```rust
   // Implementado en: src/analyzer/project.rs:40-122
   - parse_csproj() // Extrae TargetFramework, PackageReference con quick-xml
   - find_csproj()  // Localiza archivo .csproj con walkdir
   - find_csharp_files() // Encuentra todos los .cs recursivamente
   ```

2. **Parser de C# con tree-sitter** ✅
   ```rust
   // Implementado en: src/analyzer/csharp.rs:20-326
   - extract_namespace() // Soporta namespace y file-scoped namespace
   - extract_usings() // Extrae using directives
   - extract_classes() // Parsea clases con modifiers
   - extract_methods() // Parsea métodos (incluyendo async)
   - extract_properties() // Parsea propiedades con get/set
   - extract_interfaces() // Parsea interfaces
   ```

3. **Tests y Ejemplos** ✅
   ```rust
   // Creado: tests/test_analyzer.rs
   - test_csharp_analyzer() // Test completo de análisis C#
   - test_project_analyzer() // Test de análisis de proyecto
   ```

4. **Documentación Práctica** ✅
   ```
   // Creado: USAGE_EXAMPLES.md
   - Escenarios reales de uso con Claude Desktop
   - Flujo de datos completo
   - Ejemplos de interacción
   ```

#### Criterios de éxito:
- ✅ Parsear correctamente .csproj con SDK-style
- ✅ Extraer clases, métodos, propiedades de archivos .cs
- ✅ Detectar componentes (clases con modifiers public)
- ✅ Identificar PackageReferences con versiones
- ✅ Detectar métodos async (is_async flag)
- ✅ Soporta file-scoped namespaces (.NET 10)

---

### ✅ Fase 2: Sistema de Entrenamiento (COMPLETADO)

**Objetivo:** Crear sistema de patrones de código incrementales

#### Tareas Completadas:
1. **TrainingManager** ✅
   ```rust
   // Implementado en: src/training/mod.rs:11-329
   - load_patterns() // Carga patrones desde múltiples archivos JSON
   - save_patterns() // Guarda patrones agrupados por framework
   - add_pattern() // Añade nuevos patrones con timestamps
   - search_patterns() // Búsqueda avanzada con scoring
   - increment_usage() // Tracking de uso de patrones
   - get_statistics() // Estadísticas completas
   ```

2. **Sistema de Scoring Inteligente** ✅
   ```rust
   // Implementado en: src/training/mod.rs:207-256
   - Relevance score base del patrón
   - Boost por popularidad (usage_count)
   - Matches en title/description/code
   - Tag matching con scoring proporcional
   - Recency boost (patrones recientes)
   ```

3. **Índices para Búsqueda Rápida** ✅
   ```rust
   // Estructuras de indexación:
   - category_index: HashMap<String, Vec<usize>>
   - framework_index: HashMap<String, Vec<usize>>
   - Rebuild automático al cargar patrones
   ```

4. **ContextBuilder Inteligente** ✅
   ```rust
   // Implementado en: src/context/mod.rs:6-290
   - detect_framework() // Auto-detección de Blazor/ASP.NET/EF
   - get_relevant_patterns() // Patrones contextuales
   - generate_suggestions() // Sugerencias automáticas
   - check_blazor_patterns() // Validaciones Blazor-specific
   - check_async_patterns() // Validaciones async/await
   - build_context_string() // Contexto formateado para AI
   ```

5. **Patrones de Ejemplo** ✅
   ```
   // Creados:
   - data/patterns/blazor-server-lifecycle.json (2 patrones)
   - data/patterns/blazor-server-di.json (2 patrones)
   - data/patterns/blazor-server-state.json (2 patrones)
   ```

6. **Tests Completos** ✅
   ```rust
   // Creado: tests/test_training.rs
   - test_load_patterns() // Carga básica
   - test_load_multiple_pattern_files() // Múltiples archivos
   - test_search_by_framework() // Búsqueda por framework
   - test_search_by_category() // Búsqueda por categoría
   - test_search_with_scoring() // Sistema de scoring
   - test_add_and_save_pattern() // Persistencia
   - test_increment_usage() // Tracking de uso
   - test_statistics() // Estadísticas
   ```

#### Características Implementadas:
- ✅ Carga automática de patrones desde directorio
- ✅ Búsqueda multi-criterio (framework, category, tags, query)
- ✅ Sistema de scoring con múltiples factores
- ✅ Guardado automático agrupado por framework
- ✅ Detección automática de framework del proyecto
- ✅ Sugerencias contextuales (Blazor, async/await, DI)
- ✅ Validaciones específicas por framework
- ✅ Tracking de uso y popularidad
- ✅ Índices para búsqueda O(1)
- ✅ Soporte para patrones incrementales

---

### ✅ Fase 3: Protocolo MCP (COMPLETADO)

**Objetivo:** Implementar protocolo MCP para Claude Desktop

#### Tareas Completadas:
1. **Servidor JSON-RPC sobre stdio** ✅
   ```rust
   // Implementado en: src/mcp/mod.rs:13-562
   - Transporte stdio (stdin/stdout)
   - Parsing de requests JSON-RPC 2.0
   - Manejo de errores estandarizado
   - Logging con tracing
   ```

2. **Tools MCP Implementados** ✅
   ```rust
   // Tool: analyze-project (línea 324-357)
   - Analiza proyecto completo (.csproj + .cs)
   - Detecta framework automáticamente
   - Obtiene patrones relevantes
   - Genera sugerencias contextuales
   - Retorna contexto formateado para AI

   // Tool: get-patterns (línea 360-410)
   - Búsqueda por framework + categoría
   - Retorna patrones con ejemplos de código
   - Incluye metadata (tags, usage, relevance)

   // Tool: search-patterns (línea 413-452)
   - Búsqueda avanzada multi-criterio
   - Sistema de scoring
   - Filtros: query, framework, category, tags, min_score
   - Resultados ordenados por score

   // Tool: train-pattern (línea 455-519)
   - Añade nuevos patrones
   - Auto-timestamps (created_at, updated_at)
   - Persistencia automática
   - Indexación automática

   // Tool: get-statistics (línea 522-560)
   - Estadísticas del sistema
   - Total patterns, usage, avg_relevance
   - Categories y frameworks disponibles
   ```

3. **Protocolo MCP Estándar** ✅
   ```rust
   // Métodos implementados:
   - initialize → Handshake con Claude Desktop
   - tools/list → Lista de tools disponibles
   - tools/call → Ejecutar tool específico
   ```

4. **Configuración para Claude Desktop** ✅
   ```
   // Creado: claude_desktop_config.json
   // Creado: MCP_SETUP_GUIDE.md
   - Configuración Windows/Linux/Mac
   - Instrucciones paso a paso
   - Troubleshooting completo
   - Ejemplos de uso
   ```

5. **Input Schemas** ✅
   ```rust
   // Schemas JSON para cada tool:
   - Definición de parámetros
   - Tipos y descripciones
   - Required vs optional
   - Validación automática
   ```

#### Características Implementadas:
- ✅ Protocolo JSON-RPC 2.0 completo
- ✅ Transporte stdio bidireccional
- ✅ 5 tools MCP funcionales
- ✅ Manejo de errores robusto
- ✅ Logging estructurado
- ✅ Input validation
- ✅ Configuración para Claude Desktop
- ✅ Documentación completa de setup
- ✅ Integración con Fase 1 y Fase 2
- ✅ Formato de salida optimizado para AI

---

### 🔮 Fase 4: Optimizaciones

**Objetivo:** Mejorar rendimiento y experiencia

---

## 🚀 Próximos Pasos

1. ✅ ~~Ejecutar `cargo build` para compilar~~
2. ✅ ~~Implementar parser de .csproj~~
3. ✅ ~~Implementar extracción básica de C#~~
4. ✅ ~~Crear tests~~
5. ✅ ~~Implementar sistema de patrones (Fase 2)~~
6. ✅ ~~Implementar protocolo MCP (Fase 3)~~
7. 🎯 **LISTO PARA USAR:** Configurar en Claude Desktop
8. 🔮 Optimizaciones futuras (Fase 4)

---

**Última actualización:** 2025-10-25
**Fase actual:** Fase 3 - Protocolo MCP COMPLETADA ✅
**Estado:** ✅ **Listo para Producción**
**Próxima fase:** Fase 4 - Optimizaciones (opcional)

---

🦀 **¡Proyecto COMPLETO y listo para producción!** 🚀

---

## 📦 Archivos del Proyecto

### Código Fuente
- [src/main.rs](src/main.rs) - Entry point
- [src/lib.rs](src/lib.rs) - Library root
- [src/types.rs](src/types.rs) - Tipos compartidos
- [src/config.rs](src/config.rs) - Configuración
- [src/analyzer/](src/analyzer/) - Análisis de .NET
  - [project.rs](src/analyzer/project.rs) - Parser de .csproj
  - [csharp.rs](src/analyzer/csharp.rs) - Parser de C# con tree-sitter
- [src/training/](src/training/) - Sistema de patrones
  - [mod.rs](src/training/mod.rs) - TrainingManager completo
- [src/context/](src/context/) - Generación de contexto
  - [mod.rs](src/context/mod.rs) - ContextBuilder inteligente
- [src/mcp/](src/mcp/) - Protocolo MCP
  - [mod.rs](src/mcp/mod.rs) - Servidor JSON-RPC + 5 tools

### Tests
- [tests/test_analyzer.rs](tests/test_analyzer.rs) - Tests de análisis
- [tests/test_training.rs](tests/test_training.rs) - Tests de patrones

### Datos
- [data/patterns/blazor-server-lifecycle.json](data/patterns/blazor-server-lifecycle.json)
- [data/patterns/blazor-server-di.json](data/patterns/blazor-server-di.json)
- [data/patterns/blazor-server-state.json](data/patterns/blazor-server-state.json)

### Documentación
- [README.md](README.md) - Documentación principal
- [CLAUDE.md](CLAUDE.md) - Este archivo
- [MCP_SETUP_GUIDE.md](MCP_SETUP_GUIDE.md) - Guía de configuración
- [USAGE_EXAMPLES.md](USAGE_EXAMPLES.md) - Ejemplos de uso
- [PHASE2_SUMMARY.md](PHASE2_SUMMARY.md) - Detalles técnicos Fase 2
- [claude_desktop_config.json](claude_desktop_config.json) - Config ejemplo
