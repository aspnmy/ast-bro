# @ast-outline/cli (final release)

[![npm](https://img.shields.io/npm/v/@ast-outline/cli)](https://www.npmjs.com/package/@ast-outline/cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/aeroxy/ast-bro/blob/main/LICENSE)

> **This is the final release of `@ast-outline/cli`.** The project has been renamed to **ast-bro** — the old name no longer fits a toolkit that now covers dep & call graphs, hybrid semantic search, and AST-aware structural rewrite (and the name collided with a couple of unrelated packages).
>
> **Switch to:**
>
> ```bash
> npm install -g @ast-bro/cli
> ```
>
> See [github.com/aeroxy/ast-bro](https://github.com/aeroxy/ast-bro) for the new home.

## What this 2.1.1 release does

It's the same code as `ast-bro` 2.2.0 and installs **three** commands so you can start using the new names before switching package:

- `ast-outline` — the original CLI (still works, will not receive further updates)
- `ast-bro` — the new canonical name
- `sb` — a shorter alias

Every `ast-outline --help` invocation prints the discontinuation notice with the upgrade path. Once you're ready, `npm uninstall -g @ast-outline/cli && npm install -g @ast-bro/cli` and your scripts that call `ast-bro` or `sb` keep working.

## Install (for now)

```bash
npm install -g @ast-outline/cli
```

On first run the wrapper downloads the pre-built Rust binary from [GitHub releases](https://github.com/aeroxy/ast-bro/releases) and caches it locally.

| Platform | Cache directory |
|---|---|
| macOS | `~/Library/Caches/ast-bro-<version>/` |
| Linux | `~/.cache/ast-bro-<version>/` |

## Supported Platforms

| Platform | Status |
|---|---|
| macOS ARM64 | Pre-built binary available |
| Other platforms | Build from source |

For other platforms:

```bash
cargo install ast-bro
```

## Links

- [@ast-bro/cli on npm](https://www.npmjs.com/package/@ast-bro/cli) — the renamed package
- [ast-bro on GitHub](https://github.com/aeroxy/ast-bro)
- [ast-bro on PyPI](https://pypi.org/project/ast-bro/)
- [crates.io](https://crates.io/crates/ast-bro)

## License

[MIT](https://github.com/aeroxy/ast-bro/blob/main/LICENSE)
