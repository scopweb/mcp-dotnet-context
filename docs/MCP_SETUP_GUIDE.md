# 🚀 Guía de Configuración del MCP .NET Context Server

## 📋 Requisitos Previos

1. **Rust 1.70+** instalado
   ```bash
   rustup update
   ```

2. **Claude Desktop** instalado
   - Descarga desde: https://claude.ai/download

3. **.NET 10 SDK** (para testing)

---

## 🔧 Paso 1: Compilar el Servidor

```bash
cd mcp-dotnet-context-rust
cargo build --release
```

El ejecutable se generará en:
```
target/release/mcp-dotnet-context.exe  (Windows)
target/release/mcp-dotnet-context      (Linux/Mac)
```

---

## ⚙️ Paso 2: Configurar Claude Desktop

### Windows

1. Abre el archivo de configuración de Claude Desktop:
   ```
   %APPDATA%\Claude\claude_desktop_config.json
   ```

2. Agrega la configuración del MCP:
   ```json
   {
     "mcpServers": {
       "dotnet-context": {
         "command": "C:\\ruta\\completa\\target\\release\\mcp-dotnet-context.exe",
         "args": [],
         "env": {
           "RUST_LOG": "info"
         }
       }
     }
   }
   ```

### Linux/Mac

1. Abre el archivo de configuración:
   ```
   ~/.config/Claude/claude_desktop_config.json
   ```

2. Agrega:
   ```json
   {
     "mcpServers": {
       "dotnet-context": {
         "command": "/ruta/completa/target/release/mcp-dotnet-context",
         "args": [],
         "env": {
           "RUST_LOG": "info"
         }
       }
     }
   }
   ```

---

## 🎯 Paso 3: Reiniciar Claude Desktop

1. Cierra completamente Claude Desktop
2. Abre Claude Desktop nuevamente
3. Verifica que el MCP esté conectado:
   - En la esquina inferior debe aparecer el icono del MCP
   - O ve a Settings → MCP Servers

---

## 🧪 Paso 4: Probar el MCP

### Test 1: Verificar Estadísticas

En Claude Desktop, escribe:

```
Usa el tool "get-statistics" para mostrarme las estadísticas de los patrones
```

**Salida esperada:**
```
# Pattern Database Statistics

**Total Patterns:** 6
**Total Usage:** 0
**Average Relevance:** 0.88

## Categories
- lifecycle
- dependency-injection
- state-management

## Frameworks
- blazor-server
```

---

### Test 2: Buscar Patrones

```
Muéstrame todos los patrones de lifecycle para blazor-server
```

Claude llamará internamente:
```json
{
  "tool": "get-patterns",
  "arguments": {
    "framework": "blazor-server",
    "category": "lifecycle"
  }
}
```

**Salida esperada:**
```
# Patterns for blazor-server

## Component Initialization Pattern

**Category:** lifecycle
**ID:** blazor-lifecycle-oninit

Proper way to initialize Blazor Server components with async operations

```csharp
@code {
    protected override async Task OnInitializedAsync()
    {
        await LoadDataAsync();
        await base.OnInitializedAsync();
    }
}
```

**Tags:** lifecycle, initialization, async, blazor-server
**Usage Count:** 0
**Relevance:** 0.95
```

---

### Test 3: Analizar un Proyecto

Crea un proyecto de prueba:

```bash
dotnet new blazor -o TestBlazorApp
```

Luego en Claude:
```
Analiza mi proyecto Blazor en C:\Projects\TestBlazorApp
```

Claude llamará:
```json
{
  "tool": "analyze-project",
  "arguments": {
    "project_path": "C:\\Projects\\TestBlazorApp"
  }
}
```

**Salida esperada:**
```
# .NET Project Analysis

**Project:** TestBlazorApp
**Framework:** net10.0
**Language:** C# 10.0

## Dependencies

- Microsoft.AspNetCore.Components (10.0.0)
- Microsoft.AspNetCore.Components.Web (10.0.0)

## Project Statistics

- Total Files: 8
- Total Classes: 5
- Total Methods: 12

## Relevant Patterns

### Component Initialization Pattern
[... patrones relevantes ...]

## Suggestions

ℹ️ **dependency-injection**: Consider using dependency injection
   for data access and external services.
```

---

### Test 4: Añadir un Nuevo Patrón

```
Guarda este patrón como best practice para blazor forms:

ID: blazor-forms-validation
Category: forms
Framework: blazor-server
Title: Form Validation Pattern
Description: Proper validation in Blazor forms with EditForm
Code:
<EditForm Model="@model" OnValidSubmit="@HandleSubmit">
    <DataAnnotationsValidator />
    <ValidationSummary />
    <InputText @bind-Value="model.Name" />
    <button type="submit">Submit</button>
</EditForm>
Tags: forms, validation, blazor
```

Claude llamará `train-pattern` y el patrón se guardará en:
```
data/patterns/blazor-server-patterns.json
```

---

## 🔍 Tools Disponibles

### 1. **analyze-project**
Analiza un proyecto .NET completo

**Parámetros:**
- `project_path` (string, required): Ruta al directorio del proyecto

**Ejemplo:**
```json
{
  "tool": "analyze-project",
  "arguments": {
    "project_path": "C:\\MyProjects\\WeatherApp"
  }
}
```

---

### 2. **get-patterns**
Obtiene patrones por framework y categoría

**Parámetros:**
- `framework` (string, required): Framework (ej: "blazor-server")
- `category` (string, optional): Categoría (ej: "lifecycle")

**Ejemplo:**
```json
{
  "tool": "get-patterns",
  "arguments": {
    "framework": "blazor-server",
    "category": "lifecycle"
  }
}
```

---

### 3. **search-patterns**
Búsqueda avanzada con scoring

**Parámetros:**
- `query` (string, optional): Texto de búsqueda
- `framework` (string, optional): Filtro por framework
- `category` (string, optional): Filtro por categoría
- `tags` (array, optional): Filtro por tags
- `min_score` (number, optional): Score mínimo (0.0-1.0)

**Ejemplo:**
```json
{
  "tool": "search-patterns",
  "arguments": {
    "query": "async initialization",
    "framework": "blazor-server",
    "min_score": 0.8
  }
}
```

---

### 4. **train-pattern**
Añade un nuevo patrón al sistema

**Parámetros:**
- `id` (string, required): ID único
- `category` (string, required): Categoría
- `framework` (string, required): Framework
- `version` (string, optional): Versión (default: "10.0")
- `title` (string, required): Título
- `description` (string, required): Descripción
- `code` (string, required): Código de ejemplo
- `tags` (array, optional): Tags

**Ejemplo:**
```json
{
  "tool": "train-pattern",
  "arguments": {
    "id": "my-custom-pattern",
    "category": "custom",
    "framework": "blazor-server",
    "title": "My Pattern",
    "description": "A custom pattern",
    "code": "// code here",
    "tags": ["custom", "example"]
  }
}
```

---

### 5. **get-statistics**
Obtiene estadísticas del sistema

**Parámetros:** Ninguno

**Ejemplo:**
```json
{
  "tool": "get-statistics",
  "arguments": {}
}
```

---

## 🐛 Troubleshooting

### El MCP no aparece en Claude Desktop

1. Verifica que la ruta en `claude_desktop_config.json` sea correcta
2. Comprueba que el ejecutable exista y tenga permisos de ejecución
3. Revisa los logs de Claude Desktop:
   - Windows: `%APPDATA%\Claude\logs\`
   - Linux/Mac: `~/.config/Claude/logs/`

### Error: "Failed to start MCP server"

1. Compila nuevamente:
   ```bash
   cargo clean
   cargo build --release
   ```

2. Prueba ejecutar manualmente:
   ```bash
   ./target/release/mcp-dotnet-context
   ```

3. Verifica los logs con:
   ```bash
   RUST_LOG=debug ./target/release/mcp-dotnet-context
   ```

### Error: "Pattern file not found"

1. Verifica que exista el directorio:
   ```
   data/patterns/
   ```

2. Verifica que haya archivos .json en ese directorio

3. Comprueba la configuración en `config.toml`:
   ```toml
   [patterns]
   storage_path = "data/patterns"
   ```

---

## 📊 Monitoreo

### Logs del Servidor

Los logs se escriben a stderr. Para verlos:

```bash
RUST_LOG=debug ./target/release/mcp-dotnet-context 2> server.log
```

Niveles de log:
- `error`: Solo errores
- `warn`: Advertencias y errores
- `info`: Información general
- `debug`: Información detallada
- `trace`: Máximo detalle

### Verificar Comunicación

El servidor usa stdio (stdin/stdout) para comunicarse con Claude Desktop.

**Formato de request (JSON-RPC 2.0):**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "get-statistics",
    "arguments": {}
  }
}
```

**Formato de response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "# Pattern Database Statistics\n..."
    }],
    "isError": false
  }
}
```

---

## 🎓 Ejemplos de Uso en Claude Desktop

### Ejemplo 1: Workflow Completo

```
Usuario: Tengo un proyecto Blazor Server en C:\Projects\WeatherApp,
         ayúdame a analizarlo y dame sugerencias

Claude:  (Automáticamente llama analyze-project)

         He analizado tu proyecto WeatherApp:

         - Es Blazor Server .NET 10
         - Tiene 8 componentes
         - Detecté que usas OnInitialized() en WeatherForecast.razor

         Te sugiero cambiar a OnInitializedAsync() para mejor
         rendimiento. Te muestro el patrón correcto:

         (Muestra código del pattern blazor-lifecycle-oninit)

         ¿Quieres que te ayude a actualizar ese componente?
```

### Ejemplo 2: Aprendizaje Incremental

```
Usuario: Este patrón para manejar errores en formularios Blazor
         funciona muy bien, guárdalo:

         <EditForm...>
         @code {
             private string? errorMessage;

             private async Task HandleSubmit() {
                 try {
                     await SaveData();
                 } catch (Exception ex) {
                     errorMessage = ex.Message;
                     Logger.LogError(ex, "Error saving");
                 }
             }
         }

Claude:  (Llama train-pattern)

         ✅ Patrón guardado como 'blazor-forms-error-handling'!

         Lo usaré en futuras sugerencias sobre manejo de errores
         en formularios Blazor.
```

---

## 🚀 Siguientes Pasos

1. **Añadir más patrones:** Crea archivos JSON en `data/patterns/`
2. **Personalizar configuración:** Edita `config.toml`
3. **Explorar herramientas:** Prueba todos los tools disponibles
4. **Integrar con tu workflow:** Usa el MCP mientras desarrollas

---

## 📚 Recursos

- [Documentación MCP](https://modelcontextprotocol.io/)
- [CLAUDE.md](CLAUDE.md) - Guía de desarrollo
- [USAGE_EXAMPLES.md](USAGE_EXAMPLES.md) - Ejemplos prácticos
- [PHASE2_SUMMARY.md](PHASE2_SUMMARY.md) - Sistema de patrones

---

**🎉 ¡Tu MCP .NET Context Server está listo para usar!**
