//! MCP tool catalogue and dispatch — wraps the existing CLI render functions.

use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::core::{
    self, DigestOptions, OutlineOptions,
};

/// Static descriptors returned to clients via `tools/list`.
pub fn list() -> Value {
    json!({
        "tools": [
            {
                "name": "outline",
                "description": "AST-based structural outline of source files — signatures with line ranges, no method bodies. Returns text by default (5–10× smaller than reading the file). Set `json: true` for the machine-readable schema `ast-outline.outline.v1`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "paths": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Files or directories to outline.",
                            "minItems": 1
                        },
                        "no_private": { "type": "boolean", "description": "Hide private declarations." },
                        "no_fields":  { "type": "boolean", "description": "Hide field declarations." },
                        "no_docs":    { "type": "boolean", "description": "Hide doc comments." },
                        "no_attrs":   { "type": "boolean", "description": "Hide attributes / decorators." },
                        "no_lines":   { "type": "boolean", "description": "Hide line-range suffixes." },
                        "glob":       { "type": "string",  "description": "Glob filter applied during directory walk." },
                        "json":       { "type": "boolean", "description": "Return JSON (schema `ast-outline.outline.v1`) instead of text." }
                    },
                    "required": ["paths"]
                }
            },
            {
                "name": "digest",
                "description": "One-page module map for an unfamiliar directory: every file's types and public methods. Returns text by default; set `json: true` for `ast-outline.outline.v1`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "paths": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Files or directories to digest.",
                            "minItems": 1
                        },
                        "include_private": { "type": "boolean" },
                        "include_fields":  { "type": "boolean" },
                        "max_members":     { "type": "integer", "description": "Cap members per type (default 50)." },
                        "json":            { "type": "boolean" }
                    },
                    "required": ["paths"]
                }
            },
            {
                "name": "show",
                "description": "Extract source of one or more symbols from a single file. Suffix matching: `TakeDamage`, or `Player.TakeDamage` when ambiguous. For markdown the symbol is a heading. Returns text by default; set `json: true` for `ast-outline.show.v1`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path":    { "type": "string", "description": "File to search." },
                        "symbols": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "One or more symbol names to extract.",
                            "minItems": 1
                        },
                        "json":    { "type": "boolean" }
                    },
                    "required": ["path", "symbols"]
                }
            },
            {
                "name": "implements",
                "description": "Find subclasses / implementations of a type using AST matching. Transitive by default — set `direct: true` for level-1 only. Returns text by default; set `json: true` for `ast-outline.implements.v1`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "target": { "type": "string", "description": "Type name to look up." },
                        "paths":  {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Files or directories to search.",
                            "minItems": 1
                        },
                        "direct": { "type": "boolean", "description": "Direct subtypes only (skip transitive)." },
                        "json":   { "type": "boolean" }
                    },
                    "required": ["target", "paths"]
                }
            },
            {
                "name": "surface",
                "description": "True public API surface — resolves `pub use` re-exports (Rust) and `__all__` (Python) to compute exactly what a downstream user sees, not just every `pub`/non-underscore item per file. Falls back to visibility-filtered output for Java/C#/Go/Kotlin (no real re-export concept). Returns text by default; set `json: true` for `ast-outline.surface.v1`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path":            { "type": "string",  "description": "Crate root file, package init, or directory to auto-detect (default \".\")." },
                        "tree":            { "type": "boolean", "description": "Render as a hierarchical tree grouped by module." },
                        "include_chain":   { "type": "boolean", "description": "Append the via-chain on each entry (text mode only)." },
                        "max_depth":       { "type": "integer", "description": "Recursion guard for re-export chains (default 16)." },
                        "include_private": { "type": "boolean", "description": "Include private items — only meaningful for the fallback resolver." },
                        "lang":            { "type": "string",  "description": "Force a resolver: `rust`, `python`, or `fallback`." },
                        "json":            { "type": "boolean" }
                    }
                }
            },
            {
                "name": "search",
                "description": "Hybrid BM25 + dense semantic search over the repo. First call builds a per-repo index at `.ast-outline/index/` (one-time, ~seconds for typical repos). Returns text by default; set `json: true` for `ast-outline.search.v1`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query":     { "type": "string",  "description": "Search query (free-form text or symbol name)." },
                        "path":      { "type": "string",  "description": "Repo root to search in (default \".\")." },
                        "top_k":     { "type": "integer", "description": "Max results to return (default 10).", "minimum": 1 },
                        "alpha":     { "type": "number",  "description": "Override semantic-vs-BM25 weight (0.0=pure BM25, 1.0=pure semantic). Default auto-detects from query type." },
                        "languages": { "type": "array", "items": { "type": "string" }, "description": "Restrict to chunks of these languages (e.g. [\"rust\", \"python\"])." },
                        "json":      { "type": "boolean", "description": "Return JSON (schema `ast-outline.search.v1`) instead of text." }
                    },
                    "required": ["query"]
                }
            },
            {
                "name": "find_related",
                "description": "Find chunks semantically similar to a given file:line. Useful for navigating to related code. Returns text by default; set `json: true` for `ast-outline.related.v1`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path":  { "type": "string",  "description": "Repo-relative path of the source chunk." },
                        "line":  { "type": "integer", "description": "1-indexed line within `path`.", "minimum": 1 },
                        "root":  { "type": "string",  "description": "Repo root containing the index (default \".\")." },
                        "top_k": { "type": "integer", "description": "Max results (default 10).", "minimum": 1 },
                        "json":  { "type": "boolean" }
                    },
                    "required": ["path", "line"]
                }
            },
            {
                "name": "index",
                "description": "Build, refresh, or inspect the per-repo search index. With `stats: true` returns index stats. With `rebuild: true` drops the cache and rebuilds. Otherwise just opens (and incrementally refreshes if files changed). Returns text by default; set `json: true` for `ast-outline.index-stats.v1`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path":    { "type": "string",  "description": "Repo root (default \".\")." },
                        "rebuild": { "type": "boolean", "description": "Drop existing cache and rebuild." },
                        "stats":   { "type": "boolean", "description": "Print index stats and return." },
                        "json":    { "type": "boolean" }
                    }
                }
            }
        ]
    })
}

/// Result of dispatching a tool — either textual content or an error message
/// surfaced as `isError: true` on the MCP response.
pub enum CallResult {
    Text(String),
    Error(String),
}

pub fn call(name: &str, args: Value) -> CallResult {
    match name {
        "outline"      => run_outline(args),
        "digest"       => run_digest(args),
        "show"         => run_show(args),
        "implements"   => run_implements(args),
        "surface"      => run_surface(args),
        "search"       => crate::search::mcp::run_search(args),
        "find_related" => crate::search::mcp::run_find_related(args),
        "index"        => crate::search::mcp::run_index(args),
        other => CallResult::Error(format!("unknown tool: {}", other)),
    }
}

#[derive(Deserialize, Default)]
struct OutlineArgs {
    paths: Vec<PathBuf>,
    #[serde(default)] no_private: bool,
    #[serde(default)] no_fields: bool,
    #[serde(default)] no_docs: bool,
    #[serde(default)] no_attrs: bool,
    #[serde(default)] no_lines: bool,
    #[serde(default)] glob: Option<String>,
    #[serde(default)] json: bool,
}

fn run_outline(args: Value) -> CallResult {
    let a: OutlineArgs = match serde_json::from_value(args) {
        Ok(v) => v,
        Err(e) => return CallResult::Error(format!("invalid arguments: {}", e)),
    };
    if a.paths.is_empty() {
        return CallResult::Error("`paths` must not be empty".into());
    }
    let results = crate::walk_and_parse(&a.paths, a.glob.as_deref());
    let opts = OutlineOptions {
        include_private: !a.no_private,
        include_fields: !a.no_fields,
        include_docs: !a.no_docs,
        include_attributes: !a.no_attrs,
        include_line_numbers: !a.no_lines,
        max_doc_lines: 6,
        max_members: None,
    };
    if a.json {
        CallResult::Text(core::render_json_outline(&results, &opts, true))
    } else {
        let mut out = String::new();
        for res in &results {
            out.push_str(&core::render_outline(res, &opts));
            out.push('\n');
        }
        CallResult::Text(out)
    }
}

#[derive(Deserialize, Default)]
struct DigestArgs {
    paths: Vec<PathBuf>,
    #[serde(default)] include_private: bool,
    #[serde(default)] include_fields: bool,
    #[serde(default = "default_max_members")] max_members: usize,
    #[serde(default)] json: bool,
}

fn default_max_members() -> usize { 50 }

fn run_digest(args: Value) -> CallResult {
    let a: DigestArgs = match serde_json::from_value(args) {
        Ok(v) => v,
        Err(e) => return CallResult::Error(format!("invalid arguments: {}", e)),
    };
    if a.paths.is_empty() {
        return CallResult::Error("`paths` must not be empty".into());
    }
    let results = crate::walk_and_parse(&a.paths, None);
    if a.json {
        let opts = OutlineOptions {
            include_private: a.include_private,
            include_fields: a.include_fields,
            include_docs: true,
            include_attributes: true,
            include_line_numbers: true,
            max_doc_lines: 6,
            max_members: Some(a.max_members),
        };
        CallResult::Text(core::render_json_outline(&results, &opts, true))
    } else {
        let opts = DigestOptions {
            include_private: a.include_private,
            include_fields: a.include_fields,
            max_members_per_type: a.max_members,
            max_heading_depth: 3,
        };
        let root = if a.paths.len() == 1 && a.paths[0].is_dir() {
            Some(a.paths[0].as_path())
        } else {
            None
        };
        CallResult::Text(core::render_digest(&results, &opts, root))
    }
}

#[derive(Deserialize)]
struct ShowArgs {
    path: PathBuf,
    symbols: Vec<String>,
    #[serde(default)] json: bool,
}

fn run_show(args: Value) -> CallResult {
    let a: ShowArgs = match serde_json::from_value(args) {
        Ok(v) => v,
        Err(e) => return CallResult::Error(format!("invalid arguments: {}", e)),
    };
    if a.symbols.is_empty() {
        return CallResult::Error("`symbols` must not be empty".into());
    }
    let res = match crate::parse_file(&a.path) {
        Some(r) => r,
        None => return CallResult::Error(format!("could not parse file: {}", a.path.display())),
    };

    let mut seen = std::collections::HashSet::new();
    let mut all = Vec::new();
    for sym in &a.symbols {
        for m in core::find_symbols(&res, sym) {
            let key = (m.start_line, m.end_line, m.qualified_name.clone());
            if seen.insert(key) {
                all.push(m);
            }
        }
    }

    if a.json {
        CallResult::Text(core::render_json_show(&res, &all, true))
    } else {
        let mut out = String::new();
        for m in &all {
            out.push_str(&format!(
                "# {}:{}-{} {} ({})\n",
                res.path.display(), m.start_line, m.end_line, m.qualified_name, m.kind
            ));
            if !m.ancestor_signatures.is_empty() {
                out.push_str(&format!("# in: {}\n", m.ancestor_signatures.join(" → ")));
            }
            out.push_str(&m.source);
            out.push('\n');
        }
        CallResult::Text(out)
    }
}

#[derive(Deserialize)]
struct ImplementsArgs {
    target: String,
    paths: Vec<PathBuf>,
    #[serde(default)] direct: bool,
    #[serde(default)] json: bool,
}

#[derive(Deserialize, Default)]
struct SurfaceArgs {
    #[serde(default = "default_surface_path")]
    path: PathBuf,
    #[serde(default)] tree: bool,
    #[serde(default)] include_chain: bool,
    #[serde(default = "default_surface_max_depth")] max_depth: usize,
    #[serde(default)] include_private: bool,
    #[serde(default)] lang: Option<String>,
    #[serde(default)] json: bool,
}

fn default_surface_path() -> PathBuf {
    PathBuf::from(".")
}
fn default_surface_max_depth() -> usize {
    16
}

fn run_surface(args: Value) -> CallResult {
    let a: SurfaceArgs = match serde_json::from_value(args) {
        Ok(v) => v,
        Err(e) => return CallResult::Error(format!("invalid arguments: {}", e)),
    };
    let lang_override = match a.lang {
        Some(s) => match crate::surface::LangOverride::parse(&s) {
            Some(l) => Some(l),
            None => return CallResult::Error(format!("unknown lang: {}", s)),
        },
        None => None,
    };
    let output = if a.json {
        crate::surface::OutputMode::Json { compact: false }
    } else if a.tree {
        crate::surface::OutputMode::Tree
    } else {
        crate::surface::OutputMode::Flat
    };
    let opts = crate::surface::SurfaceOptions {
        output,
        include_private: a.include_private,
        max_depth: a.max_depth,
        include_chain: a.include_chain,
        lang_override,
    };
    match crate::surface::resolve_surface(&a.path, &opts) {
        Ok(entries) => {
            CallResult::Text(crate::surface::render::render(&entries, output, a.include_chain))
        }
        Err(e) => CallResult::Error(format!("{e}")),
    }
}

fn run_implements(args: Value) -> CallResult {
    let a: ImplementsArgs = match serde_json::from_value(args) {
        Ok(v) => v,
        Err(e) => return CallResult::Error(format!("invalid arguments: {}", e)),
    };
    if a.paths.is_empty() {
        return CallResult::Error("`paths` must not be empty".into());
    }
    let results = crate::walk_and_parse(&a.paths, None);
    let transitive = !a.direct;
    let matches = core::find_implementations(&results, &a.target, transitive);

    if a.json {
        CallResult::Text(core::render_json_implements(&a.target, &matches, transitive, true))
    } else {
        let mut out = format!(
            "# {} match(es) for '{}' (incl. transitive):\n",
            matches.len(), a.target
        );
        for m in &matches {
            let via = if m.via.is_empty() {
                String::new()
            } else {
                format!(" [via {}]", m.via.last().unwrap())
            };
            out.push_str(&format!("{}:{}  {} {}{}\n", m.path, m.start_line, m.kind, m.name, via));
        }
        CallResult::Text(out)
    }
}
