# 🚀 MCP .NET Context Server Setup Guide

## 📋 Prerequisites

1. **Rust 1.70+** installed
   ```bash
   rustup update
   ```

2. **Claude Desktop** installed
   - Download from: https://claude.ai/download

3. **.NET 10 SDK** (for testing)

---

## 🔧 Step 1: Build the Server

```bash
cd mcp-rust-context
cargo build --release
```

The executable will be generated at:
```
target/release/mcp-rust-context.exe  (Windows)
target/release/mcp-rust-context      (Linux/Mac)
```

---

## ⚙️ Step 2: Configure Claude Desktop

### Windows

1. Open Claude Desktop configuration file:
   ```
   %APPDATA%\Claude\claude_desktop_config.json
   ```

2. Add the MCP configuration:
   ```json
   {
     "mcpServers": {
       "dotnet-context": {
         "command": "C:\\full\\path\\to\\target\\release\\mcp-rust-context.exe",
         "args": [],
         "env": {
           "RUST_LOG": "info"
         }
       }
     }
   }
   ```

### Linux/Mac

1. Open the configuration file:
   ```
   ~/.config/Claude/claude_desktop_config.json
   ```

2. Add:
   ```json
   {
     "mcpServers": {
       "dotnet-context": {
         "command": "/full/path/to/target/release/mcp-rust-context",
         "args": [],
         "env": {
           "RUST_LOG": "info"
         }
       }
     }
   }
   ```

---

## 🎯 Step 3: Restart Claude Desktop

1. Close Claude Desktop completely
2. Open Claude Desktop again
3. Verify the MCP is connected:
   - An MCP icon should appear in the lower corner
   - Or go to Settings → MCP Servers

---

## 🧪 Step 4: Test the MCP

### Test 1: Check Statistics

In Claude Desktop, type:

```
Use the "get-statistics" tool to show me the pattern statistics
```

**Expected output:**
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

### Test 2: Search Patterns

```
Show me all lifecycle patterns for blazor-server
```

Claude will internally call:
```json
{
  "tool": "get-patterns",
  "arguments": {
    "framework": "blazor-server",
    "category": "lifecycle"
  }
}
```

**Expected output:**
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

### Test 3: Analyze a Project

Create a test project:

```bash
dotnet new blazor -o TestBlazorApp
```

Then in Claude:
```
Analyze my Blazor project at C:\Projects\TestBlazorApp
```

Claude will call:
```json
{
  "tool": "analyze-project",
  "arguments": {
    "project_path": "C:\\Projects\\TestBlazorApp"
  }
}
```

**Expected output:**
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
[... relevant patterns ...]

## Suggestions

ℹ️ **dependency-injection**: Consider using dependency injection
   for data access and external services.
```

---

### Test 4: Add a New Pattern

```
Save this pattern as a best practice for blazor forms:

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

Claude will call `train-pattern` and the pattern will be saved to:
```
data/patterns/blazor-server-patterns.json
```

---

## 🔍 Available Tools

### 1. **analyze-project**
Analyzes a complete .NET project

**Parameters:**
- `project_path` (string, required): Path to project directory

**Example:**
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
Gets patterns by framework and category

**Parameters:**
- `framework` (string, required): Framework (e.g., "blazor-server")
- `category` (string, optional): Category (e.g., "lifecycle")

**Example:**
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
Advanced search with scoring

**Parameters:**
- `query` (string, optional): Search text
- `framework` (string, optional): Filter by framework
- `category` (string, optional): Filter by category
- `tags` (array, optional): Filter by tags
- `min_score` (number, optional): Minimum score (0.0-1.0)

**Example:**
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
Adds a new pattern to the system

**Parameters:**
- `id` (string, required): Unique ID
- `category` (string, required): Category
- `framework` (string, required): Framework
- `version` (string, optional): Version (default: "10.0")
- `title` (string, required): Title
- `description` (string, required): Description
- `code` (string, required): Example code
- `tags` (array, optional): Tags

**Example:**
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
Gets system statistics

**Parameters:** None

**Example:**
```json
{
  "tool": "get-statistics",
  "arguments": {}
}
```

---

## 🐛 Troubleshooting

### MCP doesn't appear in Claude Desktop

1. Verify the path in `claude_desktop_config.json` is correct
2. Check that the executable exists and has execution permissions
3. Review Claude Desktop logs:
   - Windows: `%APPDATA%\Claude\logs\`
   - Linux/Mac: `~/.config/Claude/logs/`

### Error: "Failed to start MCP server"

1. Rebuild:
   ```bash
   cargo clean
   cargo build --release
   ```

2. Try running manually:
   ```bash
   ./target/release/mcp-rust-context
   ```

3. Check logs with:
   ```bash
   RUST_LOG=debug ./target/release/mcp-rust-context
   ```

### Error: "Pattern file not found"

1. Verify the directory exists:
   ```
   data/patterns/
   ```

2. Verify there are .json files in that directory

3. Check configuration in `config.toml`:
   ```toml
   [patterns]
   storage_path = "data/patterns"
   ```

---

## 📊 Monitoring

### Server Logs

Logs are written to stderr. To view them:

```bash
RUST_LOG=debug ./target/release/mcp-rust-context 2> server.log
```

Log levels:
- `error`: Errors only
- `warn`: Warnings and errors
- `info`: General information
- `debug`: Detailed information
- `trace`: Maximum detail

### Verify Communication

The server uses stdio (stdin/stdout) to communicate with Claude Desktop.

**Request format (JSON-RPC 2.0):**
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

**Response format:**
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

## 🎓 Usage Examples in Claude Desktop

### Example 1: Complete Workflow

```
User:   I have a Blazor Server project at C:\Projects\WeatherApp,
        help me analyze it and give me suggestions

Claude: (Automatically calls analyze-project)

        I've analyzed your WeatherApp project:

        - It's Blazor Server .NET 10
        - It has 8 components
        - I detected you're using OnInitialized() in WeatherForecast.razor

        I suggest switching to OnInitializedAsync() for better
        performance. Here's the correct pattern:

        (Shows code from blazor-lifecycle-oninit pattern)

        Would you like me to help update that component?
```

### Example 2: Incremental Learning

```
User:   This pattern for handling errors in Blazor forms works
        really well, save it:

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

Claude: (Calls train-pattern)

        ✅ Pattern saved as 'blazor-forms-error-handling'!

        I'll use it in future suggestions about error handling
        in Blazor forms.
```

---

## 🚀 Next Steps

1. **Add more patterns:** Create JSON files in `data/patterns/`
2. **Customize configuration:** Edit `config.toml`
3. **Explore tools:** Try all available tools
4. **Integrate with your workflow:** Use the MCP while developing

---

## 📚 Resources

- [MCP Documentation](https://modelcontextprotocol.io/)
- [CLAUDE.md](CLAUDE.md) - Development guide
- [USAGE_EXAMPLES.md](USAGE_EXAMPLES.md) - Practical examples
- [PHASE2_SUMMARY.md](PHASE2_SUMMARY.md) - Pattern system

---

**🎉 Your MCP .NET Context Server is ready to use!**
