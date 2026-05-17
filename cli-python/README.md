# ast-outline-cli

[![PyPI](https://img.shields.io/pypi/v/ast-outline-cli)](https://pypi.org/project/ast-outline-cli/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/aeroxy/ast-outline/blob/main/LICENSE)

CLI installer for [ast-outline](https://github.com/aeroxy/ast-outline) — a fast, AST-based code-navigation toolkit for source files. Downloads the pre-built Rust binary on first run.

## Install

```bash
pip install ast-outline-cli
```

## Usage

```bash
# Map the structure of a file (signatures + line ranges, no bodies)
ast-outline map src/player.rs

# Show the exact source of a specific method
ast-outline show Player.cs TakeDamage

# Compact digest of a whole module
ast-outline digest src/services/

# True public API (resolves pub use / __all__ re-exports)
ast-outline surface .

# Find all implementations of a type
ast-outline implements IDamageable src/

# Dependency graph
ast-outline deps src/auth.rs --depth 2
ast-outline reverse-deps src/auth.rs
ast-outline cycles

# Call graph (AST-accurate)
ast-outline callers TakeDamage
ast-outline callees Player.TakeDamage

# Hybrid BM25 + dense semantic search
ast-outline search "how does login work"

# Find semantically similar code
ast-outline find-related src/auth/login.rs:42
```

On first run, the CLI downloads the pre-built binary for your platform from [GitHub releases](https://github.com/aeroxy/ast-outline/releases) and caches it locally.

| Platform | Cache directory |
|---|---|
| macOS | `~/Library/Caches/ast-outline/` |
| Linux | `~/.cache/ast-outline/` |

## Supported Platforms

| Platform | Status |
|---|---|
| macOS ARM64 | Pre-built binary available |
| Other platforms | Build from source (see below) |

For unsupported platforms, build from source:

```bash
cargo install ast-outline
```

## What is ast-outline?

[ast-outline](https://github.com/aeroxy/ast-outline) is a fast, AST-based code-navigation toolkit built for LLM coding agents and humans. It uses [tree-sitter](https://github.com/tree-sitter/tree-sitter) via [ast-grep](https://github.com/ast-grep/ast-grep) to parse source files and provide:

- **File shape** — `map` / `digest` / `show` for signatures with line ranges (95% token savings vs reading full files)
- **True public API** — `surface` resolves re-export graphs across Rust, Python, TypeScript, and more
- **Dependency graph** — `deps` / `reverse-deps` / `cycles` / `graph` for import analysis
- **Call graph** — `callers` / `callees` with AST accuracy across 14 languages
- **Semantic search** — hybrid BM25 + dense embeddings via `search` and `find-related`
- **MCP server** — every command exposed as an MCP tool for LLM agents

Supports Rust, Python, TypeScript, JavaScript, Java, C#, Kotlin, Scala, Go, PHP, Ruby, SQL, and Markdown.

## Links

- [ast-outline source code](https://github.com/aeroxy/ast-outline)
- [npm package](https://www.npmjs.com/package/@ast-outline/cli) (Node.js installer)
- [crates.io](https://crates.io/crates/ast-outline) (Rust library)

## License

[MIT](https://github.com/aeroxy/ast-outline/blob/main/LICENSE)
