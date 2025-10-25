# 🎯 Practical Usage Examples for MCP .NET Context

## Real Scenario: Working with Claude Desktop

### 1. Automatic Project Analysis

**Situation:** You have a Blazor Server project at `C:\MyProjects\WeatherApp` and you want Claude to understand its structure.

```
YOU in Claude Desktop:
"Claude, analyze my Blazor project at C:\MyProjects\WeatherApp"

CLAUDE internally:
1. Calls the MCP tool: analyze-project("C:\MyProjects\WeatherApp")
2. MCP Server executes:
   - ProjectAnalyzer.find_csproj() → finds WeatherApp.csproj
   - ProjectAnalyzer.parse_csproj() → extracts:
     * TargetFramework: net10.0
     * Packages: Microsoft.AspNetCore.Components v10.0.0
     * Packages: Microsoft.EntityFrameworkCore v10.0.0
   - ProjectAnalyzer.find_csharp_files() → finds all .cs files
   - CSharpAnalyzer.analyze_file() for each file:
     * WeatherForecast.cs → class with properties
     * DataService.cs → service with async methods
     * Index.razor.cs → ComponentBase with lifecycle

CLAUDE responds with context:
"Your WeatherApp project is a Blazor Server application with .NET 10.0.
I detected:
- 3 Blazor components (inherit from ComponentBase)
- 1 data service (DataService)
- You're using Entity Framework Core for data
- You have async lifecycle methods in the components"
```

---

### 2. Contextual Suggestions

**Situation:** You want to load data in a Blazor component.

```
YOU:
"How do I load data when the component initializes?"

CLAUDE internally:
1. Detects the context is: Blazor Server + .NET 10
2. Calls: get-patterns(framework="blazor-server", category="lifecycle")
3. MCP Server searches in data/patterns/blazor-server-lifecycle.json
4. Returns the pattern "blazor-lifecycle-oninit"

CLAUDE responds:
"For your Blazor Server .NET 10 project, use the OnInitializedAsync() method:

```csharp
@inject IDataService DataService

@code {
    private List<WeatherForecast> forecasts;

    protected override async Task OnInitializedAsync()
    {
        // ✅ Use Task, not void
        // ✅ Always async for I/O operations
        forecasts = await DataService.GetForecastsAsync();

        // ✅ Call base at the end
        await base.OnInitializedAsync();
    }
}
```

⚠️ DON'T use OnInitialized() (without Async) for async operations
⚠️ Remember to inject the service with @inject
```

---

### 3. Incorrect Pattern Detection

**Situation:** Claude detects you're using an outdated pattern.

```
YOUR CURRENT CODE:
```csharp
@code {
    protected override void OnInitialized() // ❌ No async
    {
        var data = DataService.GetData(); // ❌ Blocking
        Summary = data.Summary;
    }
}
```

CLAUDE analyzes your code:
1. Calls: analyze-file("WeatherComponent.razor.cs")
2. CSharpAnalyzer detects:
   - Method: OnInitialized (not async)
   - Calls: DataService.GetData() (might be async)
3. Calls: get-patterns("blazor-server", "lifecycle")
4. Compares with best practices

CLAUDE suggests:
"❌ I detected you're using OnInitialized() without async.

For .NET 10 + Blazor Server, you should change it to:

```csharp
@code {
    protected override async Task OnInitializedAsync() // ✅ Async Task
    {
        var data = await DataService.GetDataAsync(); // ✅ Await
        Summary = data.Summary;
        await base.OnInitializedAsync(); // ✅ Call base
    }
}
```

Reasons:
- Avoids blocking the UI thread
- Improves app responsiveness
- It's the recommended pattern in .NET 10
```

---

### 4. Incremental Training

**Situation:** You discover a new useful pattern and want Claude to remember it.

```
YOU:
"Claude, this pattern for error handling in Blazor works really well,
save it as a best practice:

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

CLAUDE internally:
1. Calls: train-pattern({
     framework: "blazor-server",
     version: "10.0",
     category: "error-handling",
     title: "Error Handling in Component Initialization",
     code: <your code>,
     tags: ["error-handling", "try-catch", "logging"]
   })
2. MCP Server saves to: data/patterns/user-patterns/error-handling-init.json
3. Increments relevance_score based on usage

CLAUDE confirms:
"✅ Pattern saved as 'blazor-error-handling-init'
I'll use it in future suggestions for error handling in Blazor components."
```

---

## 📊 Complete Data Flow

```
┌─────────────────────────────────────────────────────────┐
│ User in Claude Desktop                                  │
│ "Help me with my Blazor component"                     │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ Claude Desktop (LLM)                                    │
│ - Decides to call MCP tools                            │
│ - JSON-RPC request via stdio                           │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ MCP .NET Context Server (Rust)                         │
│                                                         │
│ ┌─────────────────────────────────────────────────┐   │
│ │ ProjectAnalyzer                                 │   │
│ │ - find_csproj() → reads .csproj                │   │
│ │ - parse_csproj() → extracts TargetFramework    │   │
│ │ - find_csharp_files() → lists .cs files        │   │
│ └─────────────────────────────────────────────────┘   │
│                      │                                  │
│ ┌─────────────────────────────────────────────────┐   │
│ │ CSharpAnalyzer (tree-sitter)                   │   │
│ │ - extract_namespace() → gets namespace         │   │
│ │ - extract_usings() → lists using directives    │   │
│ │ - extract_classes() → parses classes           │   │
│ │ - extract_methods() → parses methods           │   │
│ │ - extract_properties() → parses properties     │   │
│ └─────────────────────────────────────────────────┘   │
│                      │                                  │
│ ┌─────────────────────────────────────────────────┐   │
│ │ PatternMatcher                                  │   │
│ │ - search_patterns() → searches data/patterns/  │   │
│ │ - score_relevance() → similarity scoring       │   │
│ │ - filter_by_framework() → filters by .NET 10   │   │
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
│     "Use OnInitializedAsync for async loads"          │
│   ]                                                     │
│ }                                                       │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ Claude Desktop receives the context                     │
│ - Now KNOWS the project structure                      │
│ - Knows the specific .NET 10 patterns                  │
│ - Can make precise suggestions                         │
│ - Generates contextually correct code                  │
└─────────────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│ Response to User                                        │
│ "For your WeatherForecast component that inherits from │
│  ComponentBase, use this pattern to load data..."      │
│  [specific and relevant code]                          │
└─────────────────────────────────────────────────────────┘
```

---

## 🎓 Concrete Benefits

### Without MCP
- Claude gives generic responses
- Doesn't know your project structure
- Suggests outdated code
- Doesn't learn from your patterns

### With MCP .NET Context
- ✅ Deep analysis of your project
- ✅ Specific suggestions for .NET 10
- ✅ Updated Blazor Server patterns
- ✅ Learns from your preferences
- ✅ Detects contextual errors
- ✅ Generates code that works in your stack

---

## 🚀 Next Steps

Once Phases 2 and 3 are implemented, you'll have:
- Complete pattern system
- Functional MCP protocol
- Claude Desktop integration
- Automatic training

**The MCP will make Claude an expert in YOUR .NET 10 + Blazor project!**
