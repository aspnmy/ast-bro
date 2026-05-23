# ast-outline (final release — 2.1.1)

[![crates.io](https://img.shields.io/crates/v/ast-outline.svg)](https://crates.io/crates/ast-outline)
[![npm](https://img.shields.io/npm/v/@ast-outline/cli)](https://www.npmjs.com/package/@ast-outline/cli)
[![PyPI](https://img.shields.io/pypi/v/ast-outline-cli)](https://pypi.org/project/ast-outline-cli/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

> ## This project has been renamed to [ast-bro](https://github.com/aeroxy/ast-bro)
>
> `ast-outline` started as a shape extractor. It grew into a multi-subsystem toolkit covering dep & call graphs, hybrid semantic search, and AST-aware structural rewrite. "Outline" no longer fit, and the name collided with an unrelated [VS Code extension](https://marketplace.visualstudio.com/items?itemName=cancerberosgx.vscode-typescript-ast-outline) and an [npm package](https://www.npmjs.com/package/ast-outline). The project lives at **[github.com/aeroxy/ast-bro](https://github.com/aeroxy/ast-bro)** going forward.
>
> **This 2.1.1 release is the final version under the `ast-outline` name.** It ships the same code as `ast-bro` 2.2.0, and bundles the new `ast-bro` and `sb` binaries alongside `ast-outline` so existing users can start using the new names before switching packages. Every `ast-outline --help` invocation prints the discontinuation notice with the upgrade path.

## Switching to ast-bro

Pick the installer for your ecosystem:

```bash
brew install aeroxy/tap/ast-bro
npm install -g @ast-bro/cli
pip install ast-bro
cargo install ast-bro
```

After install, run `ast-bro --help`, `sb --help`, or any of the subcommands documented in [the ast-bro README](https://github.com/aeroxy/ast-bro#readme). All scripts you have today that call `ast-outline` will keep working as long as the `ast-outline` package or formula stays installed; once you uninstall it, your `ast-outline` shim disappears and your scripts should call `ast-bro` (or `sb`) instead.

## What this 2.1.1 release ships

Every install method (crate, brew, npm, pip) installs three commands. They're the same binary:

- `ast-outline` — the original CLI (still works, no further updates)
- `ast-bro` — the new canonical name
- `sb` — a shorter alias

Capabilities (unchanged from `ast-bro` 2.2.0):

- **File shape** — `map` / `digest` / `show` / `implements` — signatures with line ranges instead of method bodies; AST-accurate subclass lookup.
- **True public API** — `surface` resolves re-export graphs across Rust, Python, TypeScript, Scala.
- **Dependency graph** — `deps` / `reverse-deps` / `cycles` / `graph` — file-level imports for nine languages.
- **Call graph** — `callers` / `callees` — AST-accurate, with confidence tags from a three-pass resolver.
- **Hybrid semantic search** — `search` / `find-related` — BM25 + dense embeddings via `potion-code-16M`.
- **Structural rewrite** — `run -p '<pattern>' -r '<replacement>'` — AST-aware find/replace with metavariables.
- **MCP server** — `mcp` — every subcommand exposed as an MCP tool.

Supported languages: Rust, Python, TypeScript, JavaScript, Java, C#, C++, Kotlin, Scala, Go, PHP, Ruby, SQL, Markdown.

For full feature documentation, design rationale, and architecture, see the **[ast-bro README and wiki](https://github.com/aeroxy/ast-bro#readme)**.

## Install (for now, if you really want this final ast-outline release)

```bash
# Homebrew (macOS)
brew install aeroxy/tap/ast-outline

# npm
npm install -g @ast-outline/cli

# pip
pip install ast-outline-cli

# Cargo
cargo install ast-outline
```

## License

[MIT](./LICENSE)
