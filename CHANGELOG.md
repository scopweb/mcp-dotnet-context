# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

> **⚠️ Note:** This is an experimental project for research and learning purposes.
> Not intended for production use.

## [Unreleased]

### Added (Phase 2 - Documentation & Security)
- **Security Auditing**
  - cargo-audit integration for dependency vulnerability scanning
  - RustSec Advisory Database monitoring (861 advisories)
  - Automated security checks in GitHub Actions
  - cargo-geiger for unsafe code detection
  - Daily security scans (2 AM UTC)
  - Security audit report showing 0 vulnerabilities ✅

- **Documentation**
  - English translations: MCP_SETUP_GUIDE.en.md, USAGE_EXAMPLES.en.md
  - Security audit guide: docs/SECURITY_AUDIT.md
  - Honest assessment: HONEST_ASSESSMENT.md (does it save time?)
  - Roadmap: ROADMAP.md (path to production with 420% ROI)
  - Security audit report: SECURITY_AUDIT_REPORT.md
  - Bilingual documentation (ES/EN)

- **CI/CD Pipeline**
  - GitHub Actions workflow: .github/workflows/security-audit.yml
  - Automated security scanning on every push
  - Scheduled daily vulnerability checks
  - cargo fmt enforcement (code formatting)
  - cargo clippy enforcement (linting)
  - cargo test enforcement (unit tests)
  - Build verification

- **Metadata**
  - `.gitignore` improvements
  - Updated `Cargo.toml` with proper metadata
  - `LICENSE` (MIT)
  - Complete project structure documentation

### Core Features (Phase 1)
- MCP protocol implementation (2024-11-05)
- .NET project analysis with tree-sitter
- C# code parsing (classes, methods, properties, interfaces)
- .csproj parsing with dependency detection
- 27+ built-in Blazor Server patterns
  - Lifecycle patterns (6)
  - Performance patterns (5)
  - JavaScript Interop patterns (4)
  - Data & APIs patterns (4)
  - Security patterns (4)
  - Dependency Injection patterns (2)
  - State Management patterns (2)
- Pattern training system with incremental learning
- Pattern search with intelligent scoring
- Context-aware suggestions for Blazor Server
- 5 MCP tools:
  - `analyze-project` - Full .NET project analysis
  - `get-patterns` - Retrieve patterns by framework/category
  - `search-patterns` - Advanced pattern search
  - `train-pattern` - Add custom patterns
  - `get-statistics` - Pattern database statistics

### Technical Details
- Written in Rust for performance (10x faster than Python)
- Async/await with Tokio runtime
- Tree-sitter for accurate C# parsing
- JSON-RPC 2.0 over stdio transport
- Environment variable configuration support
- GitHub Actions CI/CD
- Automated security scanning with cargo-audit

## [0.1.0] - 2025-10-25

### Added
- Initial project structure
- Basic MCP server implementation
- .NET 10 and Blazor Server support

---

[Unreleased]: https://github.com/scopweb/mcp-dotnet-context/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/scopweb/mcp-dotnet-context/releases/tag/v0.1.0
