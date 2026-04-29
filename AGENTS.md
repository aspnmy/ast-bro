# Code exploration — prefer `ast-outline` over full reads

For `.rs`, `.cs`, `.py`, `.pyi`, `.ts`, `.tsx`, `.js`, `.jsx`, `.java`, `.kt`, `.kts`,
`.scala`, `.sc`, `.go`, and `.md` files, read structure with `ast-outline`
before opening full contents.
Pull method bodies only once you know which ones you need.

Stop at the step that answers the question:

1. **Unfamiliar directory** — `ast-outline digest <dir>`: one-page map
   of every file's types and public methods.

2. **One file's shape** — `ast-outline <file>`: signatures with line
   ranges, no bodies (5–10× smaller than a full read).

3. **One method, class, or markdown section** — `ast-outline show <file>
   <Symbol>`. Suffix matching: `TakeDamage`, or `Player.TakeDamage` when
   ambiguous. Multiple at once: `ast-outline show Player.cs TakeDamage
   Heal Die`. For markdown, the symbol is the heading text.

4. **Who implements/extends a type** — `ast-outline implements <Type>
   <dir>`: AST-accurate (skip `grep`), transitive by default with
   `[via Parent]` tags on indirect matches. Add `--direct` for level-1 only.

Fall back to a full read only when you need context beyond the body
`show` returned.

If the outline header contains `# WARNING: N parse errors`, the outline
for that file is partial — read the source directly for the affected region.

`ast-outline help` for flags and rare options.

## JSON / machine-readable output

Add `--json` to any command for stable, structured output instead of
formatted text — useful when you are parsing the result programmatically
or piping into another tool.

```
ast-outline src/player.rs --json          # outline as JSON
ast-outline digest src/ --json            # digest as JSON
ast-outline show Player.cs TakeDamage --json
ast-outline implements IDamageable src/ --json
```

All JSON documents carry a `schema` field (e.g. `ast-outline.outline.v1`)
that will be bumped on breaking changes. Add `--compact` for single-line
output when pretty-printing is not needed.

## Preparing Release

1. Bump the version in `Cargo.toml`.
2. Build the release binary: `cargo build --release`
3. Zip the binary inside the release folder: `zip -j target/release/ast-outline-macos-arm64.zip target/release/ast-outline`
4. Calculate the SHA256: `shasum -a 256 target/release/ast-outline-macos-arm64.zip`
5. Update `Formula/ast-outline.rb` with the new version, URL, and SHA256.

## Documentation

If you want to understand how `ast-outline` works internally, how to add new languages, or learn about its architecture, please refer to the [`wiki/`](./wiki) directory.
