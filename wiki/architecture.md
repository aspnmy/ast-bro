# Architecture

`ast-outline` is a fast, structurally-aware CLI tool built to extract the shape of source code files without bringing the heavy baggage of method bodies.

It is written natively in Rust, relying heavily on the [tree-sitter](https://tree-sitter.github.io/tree-sitter/) parsing framework via the excellent [`ast-grep`](https://ast-grep.github.io/) ecosystem bindings, achieving incredibly fast speeds while still taking advantage of `rayon` for massive multithreading across directories.

## Core Flow

1. **Routing (`src/main.rs`)**: `ast-outline` iterates through files using the `ignore` crate (which handles `.gitignore` automatically in parallel). Each file extension is identified by `ast-grep`'s `SupportLang::from_path(path)`.
2. **Parsing (`src/adapters/*`)**: The raw source string is handed to `ast-grep` which returns a tree of `ast_grep_core::Node`. A language-specific adapter (e.g. `rust.rs`, `python.rs`) performs a highly tailored AST traversal over these nodes.
3. **IR Generation (`src/core.rs`)**: The traversal emits a canonical `Declaration` tree. This is the Intermediate Representation (IR) shared across every language. It encapsulates `kind`, `name`, `signature`, `docs`, `visibility`, etc.
4. **Rendering (`src/core.rs`)**: 
   - `outline` iterates the declarations to print a hierarchical file breakdown.
   - `digest` squashes the tree into a concise module-level API map.
   - `show` walks the tree for a specific suffix match and extracts the raw string boundaries.
   - `implements` performs a generic Breadth-First-Search across the IR trees of the entire repository to find inheritance hierarchies.

## Adding a New Language

Adding a new language is incredibly straightforward due to the foundation provided by `ast-grep-language`.

1. Identify the target language from the `SupportLang` enum in `ast-grep` (e.g. `SupportLang::Cpp`). If not present, you may need to implement a native fallback like we do for `MarkdownLang` in `src/adapters/markdown.rs`.
2. Create a new `src/adapters/mylang.rs` file.
3. Implement the `LanguageAdapter` trait.
4. Write a `_walk_top` function to perform depth-first traversal of the `ast_grep_core::Node` children.
5. Identify AST kinds by matching `node.kind()` and retrieve source values using `node.field("name")` or slicing `src[node.range().start .. node.range().end]`.
6. Convert them to generic `Declaration` objects representing Classes, Functions, Fields, Interfaces, etc.
7. Wire your new adapter into the `parse_file` routing match block in `src/main.rs`!
