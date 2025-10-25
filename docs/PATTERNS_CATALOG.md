# 📚 Catálogo de Patrones .NET 10 Blazor Server

Este documento lista todos los patrones de buenas prácticas incluidos en el MCP .NET Context Server, basados en documentación oficial de Microsoft y mejores prácticas de la comunidad.

---

## 📊 Resumen de Patrones

| Categoría | Patrones | Archivo |
|-----------|----------|---------|
| **Lifecycle** | 6 patrones | blazor-server-lifecycle.json + lifecycle-advanced.json |
| **Performance** | 5 patrones | blazor-server-performance.json |
| **JavaScript Interop** | 4 patrones | blazor-server-jsinterop.json |
| **Data & APIs** | 4 patrones | blazor-server-data-apis.json |
| **Security** | 4 patrones | blazor-server-security.json |
| **Dependency Injection** | 2 patrones | blazor-server-di.json |
| **State Management** | 2 patrones | blazor-server-state.json |
| **TOTAL** | **27 patrones** | 7 archivos JSON |

---

## 🔄 Lifecycle Patterns (6 patrones)

### 1. **Component Initialization** (`blazor-lifecycle-oninit`)
**Patrón básico de inicialización con OnInitializedAsync**
- Uso correcto de async/await
- Llamada a base.OnInitializedAsync()
- Relevancia: 0.95

### 2. **Parameter Change Handling** (`blazor-lifecycle-params`)
**Reaccionar a cambios de parámetros**
- OnParametersSetAsync
- Comparación de valores previos
- Relevancia: 0.90

### 3. **OnAfterRender for DOM** (`blazor-lifecycle-afterrender`)
**JavaScript interop después del render**
- Uso de firstRender parameter
- Inicialización única
- Relevancia: 0.92

### 4. **StateHasChanged Best Practice** (`blazor-lifecycle-statehaschanged`)
**Uso correcto de StateHasChanged**
- Cuándo llamarlo y cuándo no
- EventCallback automático
- Relevancia: 0.94

### 5. **Proper Resource Disposal** (`blazor-lifecycle-dispose`)
**IDisposable implementation**
- CancellationToken para async
- Limpieza de timers y eventos
- Prevención de memory leaks
- Relevancia: 0.96

### 6. **SetParametersAsync Advanced** (`blazor-lifecycle-setparametersasync`)
**Override de SetParametersAsync**
- Intercepción de parámetros
- Validación y transformación
- Relevancia: 0.88

---

## ⚡ Performance Patterns (5 patrones)

### 1. **ShouldRender Optimization** (`blazor-perf-shouldrender`)
**Control de re-renders**
- Return false para skip render
- Tracking de cambios
- Relevancia: 0.93

### 2. **Virtualize Component** (`blazor-perf-virtualization`)
**Listas grandes con Virtualize**
- Rendering de viewport only
- ItemsProvider async
- Relevancia: 0.95

### 3. **@key Directive** (`blazor-perf-key-attribute`)
**Tracking de elementos en listas**
- Previene recreación innecesaria
- Mejora diff de DOM
- Relevancia: 0.90

### 4. **Streaming Rendering** (`blazor-perf-streaming-rendering`)
**.NET 10: StreamRendering attribute**
- Initial HTML rápido
- Progressive enhancement
- Relevancia: 0.91

### 5. **PreserveWhitespace** (`blazor-perf-preserve-whitespace`)
**Reducción de payload HTML**
- 10-15% menos tamaño
- Mejora bandwidth
- Relevancia: 0.85

---

## 🔌 JavaScript Interop Patterns (4 patrones)

### 1. **Basic JS Interop** (`blazor-jsinterop-basic`)
**Llamadas básicas JS desde Blazor**
- InvokeVoidAsync / InvokeAsync<T>
- Paso de parámetros
- Relevancia: 0.90

### 2. **Calling .NET from JS** (`blazor-jsinterop-dotnetref`)
**DotNetObjectReference pattern**
- Callbacks desde JavaScript
- [JSInvokable] attribute
- Relevancia: 0.92

### 3. **JS Isolation Modules** (`blazor-jsinterop-isolation`)
**ES6 modules con IJSObjectReference**
- Aislamiento de código JS
- Tree-shaking support
- Relevancia: 0.94

### 4. **ElementReference** (`blazor-jsinterop-elementref`)
**Referencias a elementos DOM**
- @ref directive
- OnAfterRender timing
- Relevancia: 0.89

---

## 💾 Data & APIs Patterns (4 patrones)

### 1. **HttpClient Best Practice** (`blazor-data-httpclient`)
**Uso correcto de HttpClient**
- Error handling robusto
- Logging
- GetFromJsonAsync
- Relevancia: 0.93

### 2. **Streaming Data** (`blazor-data-streaming`)
**.NET 10: IAsyncEnumerable**
- Progressive data loading
- Memory efficiency
- Relevancia: 0.91

### 3. **Circuit State Persistence** (`blazor-data-circuit-state`)
**.NET 10: Persistent state**
- Sobrevive reconnections
- Previene pérdida de datos
- Relevancia: 0.88

### 4. **Form Validation** (`blazor-data-form-validation`)
**EditForm con DataAnnotations**
- Validación client-side
- ValidationSummary
- Relevancia: 0.95

---

## 🔒 Security Patterns (4 patrones)

### 1. **Component Authorization** (`blazor-security-authorize`)
**[Authorize] attribute**
- Role-based authorization
- Policy-based authorization
- Relevancia: 0.94

### 2. **Cascading Auth State** (`blazor-security-cascading-auth`)
**CascadingAuthenticationState**
- Auth state en jerarquía
- Claims access
- Relevancia: 0.92

### 3. **CSRF Protection** (`blazor-security-antiforgery`)
**.NET 10: Automatic antiforgery**
- Protección CSRF automática
- Manual token handling
- Relevancia: 0.89

### 4. **Secure Configuration** (`blazor-security-secrets`)
**Secrets management**
- NUNCA exponer secrets al cliente
- Azure Key Vault integration
- Relevancia: 0.96

---

## 💉 Dependency Injection Patterns (2 patrones)

### 1. **Service Injection** (`blazor-di-service-injection`)
**@inject directive**
- Inyección de servicios
- Logger integration
- Relevancia: 0.92

### 2. **Scoped Service Registration** (`blazor-di-scoped-service`)
**AddScoped para Blazor Server**
- Circuit lifetime
- Service isolation
- Relevancia: 0.88

---

## 📊 State Management Patterns (2 patrones)

### 1. **Cascading Values** (`blazor-state-cascading-value`)
**CascadingValue/Parameter**
- State sharing en jerarquía
- Evita prop drilling
- Relevancia: 0.85

### 2. **EventCallback** (`blazor-state-event-callback`)
**Child-to-Parent communication**
- EventCallback<T>
- Automatic StateHasChanged
- Relevancia: 0.90

---

## 🎯 Patrones por Nivel de Prioridad

### 🔴 Críticos (Relevance ≥ 0.94)
1. **Proper Resource Disposal** (0.96) - Previene memory leaks
2. **Secure Configuration** (0.96) - Seguridad crítica
3. **Form Validation** (0.95) - UX y seguridad
4. **Virtualize Component** (0.95) - Performance con listas grandes
5. **Component Initialization** (0.95) - Base fundamental
6. **StateHasChanged** (0.94) - Rendering correcto
7. **JS Isolation Modules** (0.94) - Best practice moderno
8. **Component Authorization** (0.94) - Seguridad

### 🟡 Importantes (Relevance 0.90-0.93)
- HttpClient Best Practice (0.93)
- ShouldRender Optimization (0.93)
- OnAfterRender (0.92)
- Calling .NET from JS (0.92)
- Cascading Auth State (0.92)
- Service Injection (0.92)
- Streaming Rendering (0.91)
- Streaming Data (0.91)
- Parameter Change Handling (0.90)
- @key Directive (0.90)
- Basic JS Interop (0.90)
- EventCallback (0.90)

### 🟢 Útiles (Relevance < 0.90)
- ElementReference (0.89)
- CSRF Protection (0.89)
- SetParametersAsync Advanced (0.88)
- Circuit State Persistence (0.88)
- Scoped Service (0.88)
- PreserveWhitespace (0.85)
- Cascading Values (0.85)

---

## 📖 Cómo Usar los Patrones

### En Claude Desktop

```
Usuario: "Muéstrame patrones de lifecycle para Blazor"

Claude: (llama get-patterns)

        Aquí tienes 6 patrones de lifecycle:

        1. Component Initialization (0.95)
           - OnInitializedAsync correcto
           - [código del patrón]

        2. StateHasChanged Best Practice (0.94)
           - Cuándo llamarlo
           - [código del patrón]
        ...
```

### Búsqueda por Query

```
Usuario: "Busca patrones sobre async y performance"

Claude: (llama search-patterns con query="async performance")

        Encontré 4 patrones:

        1. StateHasChanged (score: 0.98)
        2. OnAfterRender (score: 0.95)
        3. Streaming Data (score: 0.92)
        4. HttpClient Best Practice (score: 0.90)
```

### Por Categoría

```
Usuario: "Patrones de seguridad para Blazor"

Claude: (llama get-patterns framework="blazor-server" category="security")

        4 patrones de seguridad:

        1. Secure Configuration (0.96) ⭐
           - NUNCA expongas secrets
           - Usa Azure Key Vault
        ...
```

---

## 🔄 Actualización de Patrones

Los patrones se actualizan automáticamente cuando:
1. Se usa `train-pattern` tool para añadir nuevos
2. Se incrementa `usage_count` al usarlos
3. Se ajusta `relevance_score` basado en feedback

---

## 🎓 Fuentes

Estos patrones están basados en:
- Microsoft Learn documentation oficial
- ASP.NET Core Blazor performance best practices
- .NET 10 release notes y previews
- Blazor component lifecycle documentation
- Community best practices y code reviews

---

## 🚀 Para Desarrolladores

### Añadir un Nuevo Patrón

```bash
# Opción 1: Usar el MCP tool
"Claude, guarda este patrón como best practice: [código]"

# Opción 2: Editar JSON manualmente
# Añadir a data/patterns/blazor-server-[category].json

# Opción 3: Programáticamente
let pattern = CodePattern { ... };
training_manager.add_pattern(pattern);
training_manager.save_patterns().await?;
```

### Estructura de un Patrón

```json
{
  "id": "unique-id",
  "category": "lifecycle|performance|security|...",
  "framework": "blazor-server",
  "version": "10.0",
  "title": "Short Title",
  "description": "Detailed description with context",
  "code": "// Code example with comments",
  "tags": ["tag1", "tag2"],
  "usage_count": 0,
  "relevance_score": 0.0-1.0,
  "created_at": "ISO8601",
  "updated_at": "ISO8601"
}
```

---

**Total de patrones disponibles: 27**
**Última actualización: 2025-10-25**
**Versión del catálogo: 1.0**
