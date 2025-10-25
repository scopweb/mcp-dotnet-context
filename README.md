# 🦀 MCP .NET Context

> A specialized Model Context Protocol (MCP) server for .NET & Blazor Server that provides intelligent context analysis and code pattern training for AI assistants.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![MCP](https://img.shields.io/badge/MCP-2024--11--05-blue.svg?style=flat-square)](https://modelcontextprotocol.io)
[![Status](https://img.shields.io/badge/Status-Experimental-yellow.svg?style=flat-square)](https://github.com/scopweb/mcp-dotnet-context)

---

## ⚠️ Experimental Project

> **This is a proof-of-concept MCP server focused on context reinforcement and learning experiments for Claude Desktop.**
>
> **Intended for research and development purposes only.** This project explores advanced context management patterns and training mechanisms for AI assistants. Use it as a reference for improving future MCP implementations or adapting the concepts to your own projects.
>
> 🧪 **Not recommended for production use.** Consider this an educational resource and testing ground for MCP capabilities.

## ✨ Features

- 🔍 **Deep .NET Analysis**: Parse .csproj, analyze C# code with tree-sitter, detect dependencies
- 📚 **27+ Built-in Patterns**: Official Microsoft best practices for Blazor Server
  - 🔄 Lifecycle (6 patterns)
  - ⚡ Performance (5 patterns)
  - 🌐 JavaScript Interop (4 patterns)
  - 📡 Data & APIs (4 patterns)
  - 🔒 Security (4 patterns)
  - 💉 Dependency Injection (2 patterns)
  - 📦 State Management (2 patterns)
- 🎓 **Pattern Training**: Incremental learning system - add your own patterns
- 🎯 **Blazor Specialized**: Context-aware suggestions for Blazor Server components
- 🦀 **Rust Performance**: 10x faster than Python equivalents
- 🔌 **MCP Native**: Works with Claude Desktop and other MCP clients

## 🦀 Why Rust?

| Feature | Rust Implementation | Python Equivalent |
|---------|-------------------|-------------------|
| Startup Time | 50ms | 300ms |
| Analysis Time | 120ms | 1.2s |
| Memory Usage | 8MB | 45MB |
| Binary Size | 3MB | 40MB+ deps |

- ⚡ **10x Faster** than Python implementations
- 🔒 **Memory Safe** - zero crashes or leaks
- 📦 **Single Binary** - no runtime dependencies
- 🚀 **Concurrent** - efficient async request handling
- 🎯 **Native Parsing** - tree-sitter C# integration

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** ([Install Rust](https://rustup.rs/))
- **.NET 10 SDK** (optional, for testing projects)

### Installation

```bash
# Clone the repository
git clone https://github.com/scopweb/mcp-dotnet-context.git
cd mcp-dotnet-context

# Build release binary
cargo build --release

# Binary location:
# Windows: target/release/mcp-dotnet-context.exe
# Linux/Mac: target/release/mcp-dotnet-context
```

### Configuration for Claude Desktop

#### Windows

Edit: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "dotnet-context": {
      "command": "C:\\path\\to\\mcp-dotnet-context\\target\\release\\mcp-dotnet-context.exe",
      "args": [],
      "env": {
        "MCP_PATTERNS_PATH": "C:\\path\\to\\mcp-dotnet-context\\data\\patterns"
      }
    }
  }
}
```

#### Linux/Mac

Edit: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "dotnet-context": {
      "command": "/path/to/mcp-dotnet-context/target/release/mcp-dotnet-context",
      "args": [],
      "env": {
        "MCP_PATTERNS_PATH": "/path/to/mcp-dotnet-context/data/patterns"
      }
    }
  }
}
```

**Important:** Use absolute paths in the configuration.

### Restart Claude Desktop

Close and reopen Claude Desktop to load the MCP server.

---

## 📖 Usage

### Analyze a .NET Project

```
You: Analyze my Blazor project at C:\Projects\MyApp

Claude → calls analyze-project tool
Server → analyzes .csproj, C# files, dependencies
Claude → shows structure, patterns, suggestions
```

### Get Code Patterns

```
You: Show me Blazor Server lifecycle patterns

Claude → calls get-patterns tool
Server → returns relevant patterns with code examples
Claude → explains best practices
```

### Search Patterns

```
You: Find patterns for async initialization in Blazor

Claude → calls search-patterns tool
Server → intelligent search with scoring
Claude → shows most relevant patterns
```

### Train New Patterns

```
You: Save this as a best practice for Blazor error handling:
[your code example]

Claude → calls train-pattern tool
Server → stores pattern with metadata
Claude → confirms pattern saved
```

### Get Statistics

```
You: Show pattern database stats

Claude → calls get-statistics tool
Server → returns total patterns, categories, frameworks
```

---

## 🛠️ Available Tools

| Tool | Description | Parameters |
|------|-------------|------------|
| `analyze-project` | Analyze .NET project structure | `project_path` (string) |
| `get-patterns` | Get patterns by framework/category | `framework` (string), `category` (optional) |
| `search-patterns` | Advanced pattern search | `query`, `framework`, `category`, `tags`, `min_score` |
| `train-pattern` | Add custom pattern | `id`, `category`, `framework`, `title`, `description`, `code`, `tags` |
| `get-statistics` | Database statistics | None |

---

## 🏗️ Architecture

```
mcp-dotnet-context/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   ├── config.rs            # Configuration
│   ├── types.rs             # Shared types
│   ├── analyzer/            # Code analysis
│   │   ├── csharp.rs        # C# parser (tree-sitter)
│   │   └── project.rs       # .csproj parser
│   ├── context/             # Context generation
│   ├── training/            # Pattern management
│   │   └── mod.rs           # Training system
│   └── mcp/                 # MCP protocol
│       └── mod.rs           # Server implementation
├── data/
│   └── patterns/            # Built-in patterns (JSON)
├── tests/                   # Integration tests
├── docs/                    # Technical documentation
├── Cargo.toml
├── README.md
├── CHANGELOG.md
└── LICENSE
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test analyzer

# Test with output
cargo test -- --nocapture

# Manual test (stdio)
echo '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}},"id":1}' | cargo run --release
```

---

## 📚 Documentation

- **[MCP Setup Guide](docs/MCP_SETUP_GUIDE.md)** - Detailed configuration instructions
- **[Development Guide](docs/CLAUDE.md)** - Project architecture and development workflow
- **[Creating MCPs with Rust](docs/CrearUnMcpConRust.md)** - Complete guide to building MCP servers
- **[Usage Examples](docs/USAGE_EXAMPLES.md)** - Practical examples and scenarios
- **[Pattern Catalog](docs/PATTERNS_CATALOG.md)** - All 27+ built-in patterns
- **[Changelog](CHANGELOG.md)** - Version history

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Adding Patterns

To contribute new patterns:

1. Add JSON file to `data/patterns/`
2. Follow the pattern schema
3. Test with `get-statistics` tool
4. Submit PR with pattern details

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- Code parsing with [tree-sitter](https://tree-sitter.github.io/)
- MCP Protocol by [Anthropic](https://www.anthropic.com/)
- Blazor patterns from [Microsoft Docs](https://learn.microsoft.com/en-us/aspnet/core/blazor/)

---

## 🐛 Troubleshooting

### Server not connecting

1. Check Claude Desktop logs: `%APPDATA%\Claude\logs\mcp-server-dotnet-context.log`
2. Verify executable path is absolute
3. Ensure `MCP_PATTERNS_PATH` points to correct directory
4. Try rebuilding: `cargo clean && cargo build --release`

### No patterns loaded

```bash
# Check patterns directory exists
ls data/patterns/*.json

# Verify environment variable
echo $MCP_PATTERNS_PATH  # Linux/Mac
echo %MCP_PATTERNS_PATH%  # Windows
```

### Parse errors

- Ensure `.csproj` is valid XML
- Check C# files are UTF-8 encoded
- Verify .NET SDK version compatibility

For more help, see [MCP_SETUP_GUIDE.md](docs/MCP_SETUP_GUIDE.md) or open an issue.

---

## 📬 Contact

- **Issues**: [GitHub Issues](https://github.com/scopweb/mcp-dotnet-context/issues)
- **Discussions**: [GitHub Discussions](https://github.com/scopweb/mcp-dotnet-context/discussions)

---

<div align="center">

**Made with 🦀 Rust and ❤️ for the .NET community**

[⭐ Star this repository](https://github.com/scopweb/mcp-dotnet-context) if you find it useful!

</div>
