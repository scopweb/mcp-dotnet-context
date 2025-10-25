# 🎯 Ejemplos Prácticos de Uso del MCP .NET Context

## Escenario Real: Trabajando con Claude Desktop

### 1. Análisis Automático de Proyecto

**Situación:** Tienes un proyecto Blazor Server en `C:\MyProjects\WeatherApp` y quieres que Claude entienda su estructura.

```
TÚ en Claude Desktop:
"Claude, analiza mi proyecto Blazor en C:\MyProjects\WeatherApp"

CLAUDE internamente:
1. Llama al tool MCP: analyze-project("C:\MyProjects\WeatherApp")
2. MCP Server ejecuta:
   - ProjectAnalyzer.find_csproj() → encuentra WeatherApp.csproj
   - ProjectAnalyzer.parse_csproj() → extrae:
     * TargetFramework: net10.0
     * Packages: Microsoft.AspNetCore.Components v10.0.0
     * Packages: Microsoft.EntityFrameworkCore v10.0.0
   - ProjectAnalyzer.find_csharp_files() → encuentra todos los .cs
   - CSharpAnalyzer.analyze_file() por cada archivo:
     * WeatherForecast.cs → clase con propiedades
     * DataService.cs → servicio con métodos async
     * Index.razor.cs → ComponentBase con lifecycle

CLAUDE responde con contexto:
"Tu proyecto WeatherApp es una aplicación Blazor Server con .NET 10.0.
He detectado:
- 3 componentes Blazor (heredan de ComponentBase)
- 1 servicio de datos (DataService)
- Usas Entity Framework Core para datos
- Tienes métodos lifecycle async en los componentes"
```

---

### 2. Sugerencias Contextuales

**Situación:** Quieres cargar datos en un componente Blazor.

```
TÚ:
"¿Cómo cargo datos cuando el componente se inicializa?"

CLAUDE internamente:
1. Detecta que el contexto es: Blazor Server + .NET 10
2. Llama: get-patterns(framework="blazor-server", category="lifecycle")
3. MCP Server busca en data/patterns/blazor-server-lifecycle.json
4. Devuelve el pattern "blazor-lifecycle-oninit"

CLAUDE responde:
"Para tu proyecto Blazor Server .NET 10, usa el método OnInitializedAsync():

```csharp
@inject IDataService DataService

@code {
    private List<WeatherForecast> forecasts;

    protected override async Task OnInitializedAsync()
    {
        // ✅ Usa Task, no void
        // ✅ Siempre async para operaciones I/O
        forecasts = await DataService.GetForecastsAsync();

        // ✅ Llama al base al final
        await base.OnInitializedAsync();
    }
}
```

⚠️ NO uses OnInitialized() (sin Async) para operaciones async
⚠️ Recuerda inyectar el servicio con @inject
```

---

### 3. Detección de Patrones Incorrectos

**Situación:** Claude detecta que estás usando un patrón desactualizado.

```
TU CÓDIGO actual:
```csharp
@code {
    protected override void OnInitialized() // ❌ Sin async
    {
        var data = DataService.GetData(); // ❌ Bloqueante
        Summary = data.Summary;
    }
}
```

CLAUDE analiza tu código:
1. Llama: analyze-file("WeatherComponent.razor.cs")
2. CSharpAnalyzer detecta:
   - Método: OnInitialized (no async)
   - Llama a: DataService.GetData() (puede ser async)
3. Llama: get-patterns("blazor-server", "lifecycle")
4. Compara con best practices

CLAUDE sugiere:
"❌ Detecté que usas OnInitialized() sin async.

Para .NET 10 + Blazor Server, deberías cambiarlo a:

```csharp
@code {
    protected override async Task OnInitializedAsync() // ✅ Async Task
    {
        var data = await DataService.GetDataAsync(); // ✅ Await
        Summary = data.Summary;
        await base.OnInitializedAsync(); // ✅ Llama base
    }
}
```

Razones:
- Evita bloquear el UI thread
- Mejora la responsividad de la app
- Es el patrón recomendado en .NET 10
```

---

### 4. Entrenamiento Incremental

**Situación:** Descubres un nuevo patrón útil y quieres que Claude lo recuerde.

```
TÚ:
"Claude, este patrón para manejo de errores en Blazor funciona muy bien,
guárdalo como best practice:

```csharp
@code {
    private string errorMessage;

    protected override async Task OnInitializedAsync()
    {
        try
        {
            await LoadDataAsync();
        }
        catch (Exception ex)
        {
            errorMessage = $"Error: {ex.Message}";
            Logger.LogError(ex, "Error loading data");
        }
    }
}
```

CLAUDE internamente:
1. Llama: train-pattern({
     framework: "blazor-server",
     version: "10.0",
     category: "error-handling",
     title: "Error Handling in Component Initialization",
     code: <tu código>,
     tags: ["error-handling", "try-catch", "logging"]
   })
2. MCP Server guarda en: data/patterns/user-patterns/error-handling-init.json
3. Incrementa relevance_score basado en uso

CLAUDE confirma:
"✅ Patrón guardado como 'blazor-error-handling-init'
Lo usaré en futuras sugerencias para manejo de errores en componentes Blazor."
```

---

## 📊 Flujo de Datos Completo

```
┌─────────────────────────────────────────────────────────┐
│ Usuario en Claude Desktop                               │
│ "Ayúdame con mi componente Blazor"                     │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ Claude Desktop (LLM)                                    │
│ - Decide llamar tools MCP                              │
│ - JSON-RPC request via stdio                           │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ MCP .NET Context Server (Rust)                         │
│                                                         │
│ ┌─────────────────────────────────────────────────┐   │
│ │ ProjectAnalyzer                                 │   │
│ │ - find_csproj() → lee .csproj                  │   │
│ │ - parse_csproj() → extrae TargetFramework      │   │
│ │ - find_csharp_files() → lista archivos .cs     │   │
│ └─────────────────────────────────────────────────┘   │
│                      │                                  │
│ ┌─────────────────────────────────────────────────┐   │
│ │ CSharpAnalyzer (tree-sitter)                   │   │
│ │ - extract_namespace() → obtiene namespace      │   │
│ │ - extract_usings() → lista using directives    │   │
│ │ - extract_classes() → parsea clases            │   │
│ │ - extract_methods() → parsea métodos           │   │
│ │ - extract_properties() → parsea propiedades    │   │
│ └─────────────────────────────────────────────────┘   │
│                      │                                  │
│ ┌─────────────────────────────────────────────────┐   │
│ │ PatternMatcher                                  │   │
│ │ - search_patterns() → busca en data/patterns/  │   │
│ │ - score_relevance() → scoring por similitud    │   │
│ │ - filter_by_framework() → filtra por .NET 10   │   │
│ └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ JSON Response                                           │
│ {                                                       │
│   "project": {                                          │
│     "name": "WeatherApp",                              │
│     "framework": "net10.0",                            │
│     "packages": [...],                                  │
│     "classes": [                                        │
│       {                                                 │
│         "name": "WeatherForecast",                     │
│         "base_class": "ComponentBase",                 │
│         "methods": [                                    │
│           {                                             │
│             "name": "OnInitializedAsync",              │
│             "is_async": true,                          │
│             "return_type": "Task"                      │
│           }                                             │
│         ]                                               │
│       }                                                 │
│     ]                                                   │
│   },                                                    │
│   "patterns": [                                         │
│     {                                                   │
│       "id": "blazor-lifecycle-oninit",                 │
│       "code": "...",                                    │
│       "relevance_score": 0.95                          │
│     }                                                   │
│   ],                                                    │
│   "suggestions": [                                      │
│     "Usa OnInitializedAsync para cargas async"        │
│   ]                                                     │
│ }                                                       │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ Claude Desktop recibe el contexto                       │
│ - Ahora SABE la estructura del proyecto                │
│ - Conoce los patrones específicos de .NET 10           │
│ - Puede hacer sugerencias precisas                     │
│ - Genera código contextualmente correcto               │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ Respuesta al Usuario                                    │
│ "Para tu componente WeatherForecast que hereda de      │
│  ComponentBase, usa este patrón para cargar datos..."  │
│  [código específico y relevante]                       │
└─────────────────────────────────────────────────────────┘
```

---

## 🎓 Beneficios Concretos

### Sin MCP
- Claude da respuestas genéricas
- No conoce tu estructura de proyecto
- Sugiere código desactualizado
- No aprende de tus patrones

### Con MCP .NET Context
- ✅ Análisis profundo de tu proyecto
- ✅ Sugerencias específicas para .NET 10
- ✅ Patrones actualizados de Blazor Server
- ✅ Aprende de tus preferencias
- ✅ Detecta errores contextuales
- ✅ Genera código que funciona en tu stack

---

## 🚀 Próximos Pasos

Una vez implementadas las Fases 2 y 3, tendrás:
- Sistema completo de patterns
- Protocolo MCP funcional
- Integración con Claude Desktop
- Entrenamiento automático

**¡El MCP hará que Claude sea un experto en TU proyecto .NET 10 + Blazor!**
