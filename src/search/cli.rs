//! Clap-side handlers for `search`, `find-related`, `index`. Called from
//! `main.rs` after the `Commands` enum has been parsed.
//!
//! Keeps `main.rs` thin: each subcommand is a one-line dispatch into here.

use crate::search::format::{
    render_index_stats_json, render_index_stats_text, render_related_json, render_related_text,
    render_search_json, render_search_text,
};
use crate::search::fusion::resolve_alpha;
use crate::search::index::{Index, SearchOptions};
use std::path::PathBuf;

/// Run `ast-outline search <QUERY> [PATH]`. Returns process exit code.
pub fn run_search(
    query: &str,
    path: &PathBuf,
    top_k: usize,
    alpha: Option<f32>,
    languages: Vec<String>,
    json: bool,
    pretty: bool,
) -> i32 {
    let index = match Index::open(path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("ast-outline: failed to open index: {e}");
            return 1;
        }
    };
    let opts = SearchOptions {
        top_k,
        alpha,
        languages: if languages.is_empty() { None } else { Some(languages) },
    };
    let hits = index.search(query, &opts);
    if json {
        let alpha_used = resolve_alpha(query, alpha);
        println!("{}", render_search_json(query, alpha_used, &hits, pretty));
    } else {
        print!("{}", render_search_text(query, &hits));
    }
    0
}

/// Run `ast-outline find-related <FILE>:<LINE> [PATH]`.
pub fn run_find_related(
    file_path: &str,
    line: u32,
    path: &PathBuf,
    top_k: usize,
    json: bool,
    pretty: bool,
) -> i32 {
    let index = match Index::open(path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("ast-outline: failed to open index: {e}");
            return 1;
        }
    };
    let hits = match index.find_related(file_path, line, top_k) {
        Some(h) => h,
        None => {
            eprintln!(
                "ast-outline: no chunk at {file_path}:{line} (was the file indexed?)"
            );
            return 2;
        }
    };
    if json {
        println!("{}", render_related_json(file_path, line, &hits, pretty));
    } else {
        print!("{}", render_related_text(file_path, line, &hits));
    }
    0
}

/// Run `ast-outline index [PATH]`. With `--rebuild`, drops any existing
/// cache and rebuilds from scratch. With `--stats`, just prints stats and
/// exits 0 if an index exists, 2 if not.
pub fn run_index(path: &PathBuf, rebuild: bool, stats: bool, json: bool, pretty: bool) -> i32 {
    if stats {
        let index = match Index::open(path) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("ast-outline: no readable index at {}: {e}", path.display());
                return 2;
            }
        };
        let file_count = std::fs::read(&index.paths.files_bin)
            .ok()
            .and_then(|b| {
                let cfg = bincode::config::standard();
                bincode::serde::decode_from_slice::<Vec<crate::search::cache::FileRecord>, _>(
                    &b, cfg,
                )
                .ok()
                .map(|(v, _)| v.len())
            })
            .unwrap_or(0);
        if json {
            println!("{}", render_index_stats_json(&index.meta, file_count, pretty));
        } else {
            print!("{}", render_index_stats_text(&index.meta, file_count));
        }
        return 0;
    }

    let result = if rebuild {
        Index::build(path)
    } else {
        Index::open(path)
    };
    match result {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("ast-outline: index build failed: {e}");
            1
        }
    }
}
