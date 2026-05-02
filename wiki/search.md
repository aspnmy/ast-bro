# Semantic search

Two new subcommands — `search` and `find-related` — and a per-repo persistent index. This page documents the internal architecture. For the user-facing surface see the README. For network/TLS behaviour see [network-security.md](network-security.md). For what gets indexed see [file-filtering.md](file-filtering.md).

## Pipeline

```
search "<query>":
  tokenize(query)
    ├─ BM25.get_scores(tokens, mask)  → top_k × 5 candidates
    └─ encode_one(query) → cosine_topk(embeddings, mask)  → top_k × 5 candidates
  RRF normalize each (k = 60)
  combine(alpha-weighted)              alpha auto: 0.3 symbol query, 0.5 NL
  boost_multi_chunk_files(...)         file coherence (+20% × file_sum/max_file_sum)
  apply_query_boost(...)               definition (3×), embedded symbol (1.5×),
                                       file-stem matches for NL queries
  rerank_topk(top_k, penalise_paths=True)
                                       test files 0.3×, compat dirs 0.3×,
                                       __init__.py 0.5×, .d.ts 0.7×,
                                       file-saturation decay (0.5^extra)

find-related <file>:<line>:
  resolve_chunk(file, line)             prefer chunks where start ≤ line < end
  encode chunk.content
  cosine_topk(embeddings, mask)         mask: same language, exclude self
  return top_k
```

## Module layout

```
src/search/
├── tokens.rs      identifier extraction + camel/snake split
├── bm25.rs        sparse BM25 (lucene variant), get_scores(tokens, mask)
├── chunker.rs     AST-aware chunking via ast-grep + tree-sitter-md (markdown)
├── download.rs    HF probe + hf-mirror fallback + sha256 manifest
├── embed.rs       safetensors mmap + tokenizer.json + cosine_topk (SIMD via wide)
├── fusion.rs      RRF (k=60) + alpha resolver
├── ranking.rs     boosting + penalties + greedy top-k
├── cache.rs       mtime + xxhash3 delta detection
├── index.rs       orchestrator: build / open / search / find_related / persist
├── format.rs      text + JSON renderers (shared by CLI and MCP)
├── cli.rs         clap-side handlers — called from main.rs
└── mcp.rs         MCP-side handlers — called from mcp/tools.rs
```

The `cli.rs` / `mcp.rs` shims keep dispatch in [main.rs](../src/main.rs) and [mcp/tools.rs](../src/mcp/tools.rs) thin: each subcommand is a one-line forward into the shared `Index::search` / `Index::find_related`.

## Embedding model: model2vec / potion-code-16M

A "static" embedder — no neural-net inference, just a `vocab × 256` float32 lookup table:

```
encode_one(text):
  ids = tokenizer.encode(text, add_special_tokens=False)
  mean = average(embeddings[id] for id in ids)
  return L2_normalize(mean)
```

That's it. Cost is dominated by tokenization (~10–100 µs); embedding lookup is essentially free. Output is always L2-normalized so cosine similarity reduces to a dot product.

`Embedder::open(model_dir)` mmaps `model.safetensors` (~64 MB) — the matrix stays paged in but never copied. `vocab × 256 × 4 bytes` = ~64 MB regardless of repo size.

## Cosine top-k: brute-force SIMD

`cosine_topk(query, embeddings, mask, k)` walks every row of the chunk-embedding matrix:

- Pre-loads the query into 32 × `wide::f32x8` SIMD lanes (256 dims = 32 chunks of 8).
- Each row is a `&[f32; 256]` — one cache-friendly slice from the contiguous matrix.
- Dot product per row: 32 × 8-lane FMA + horizontal sum.
- For matrices ≥ 4096 rows, parallelizes via rayon over row-blocks of 256.
- Top-k via `select_nth_unstable_by` on indices, then sort the prefix.

Bench: ~25 ms single-threaded on a 10k-chunk repo, ~5 ms across 8 cores.

No HNSW or other ANN structure. At repo scale (≤100k chunks for monorepos), brute-force SIMD is faster than ANN setup time and trivial to maintain.

## BM25: hand-rolled lucene variant

We use `bm25s.BM25(method="lucene")`'s exact formula:

```text
idf(t) = ln(1 + (N - df(t) + 0.5) / (df(t) + 0.5))
score(d, q) = Σ idf(t) · tf(t,d) · (k1+1)
                          / (tf(t,d) + k1 · (1 - b + b · |d| / avgdl))
                          k1 = 1.5, b = 0.75
```

`get_scores(tokens, mask)` returns one f32 per chunk. The mask is a *post-filter score multiplier* (matches `bm25s`'s `weight_mask` semantics) — not a slice — so IDF normalization over the full corpus is preserved when filtering by language.

The hand-roll is ~150 lines and lets us own the mask semantics. The `bm25` crate doesn't expose them.

## RRF + ranking

Combining BM25 and dense scores by raw magnitude doesn't work — they're on different scales. RRF (`1 / (k + rank)` with `k = 60`) normalizes both into the same band before alpha-weighted blending.

Then four boosting / penalty passes:

1. **`boost_multi_chunk_files`** — files with multiple high-scoring chunks get their top chunk lifted by `0.2 × max_score × (file_sum / max_file_sum)`.
2. **Symbol queries** trigger `_boost_symbol_definitions`: chunks that *define* the queried name get `3× max_score` (1.5× multiplier if the file stem matches the symbol). Also scans non-candidate chunks whose file stem matches.
3. **NL queries** trigger `_boost_stem_matches` (file/dir name overlap with query keywords) + `_boost_embedded_symbols` (PascalCase / camelCase identifiers in the query, half-strength definition boost).
4. **`rerank_topk`** applies multiplicative path penalties (test files 0.3×, compat/legacy dirs 0.3×, examples 0.3×, `.d.ts` 0.7×, `__init__.py` / `package-info.java` 0.5×) and greedy file-saturation decay (2nd chunk from the same file × 0.5, 3rd × 0.25, ...).

## On-disk format

```
.ast-outline/
├── .gitignore               # auto-written: "*"
└── index/
    ├── meta.json            # ~2 KB — schema, model id+revision, chunk_count, tombstones
    ├── chunks.bin           # bincode Vec<Chunk> (~1.5 KB/chunk × N)
    ├── embeddings.f32       # N × 256 × 4 bytes, header-less, little-endian
    ├── bm25.bin             # bincode Bm25Index (vocab + idf + postings)
    ├── files.bin            # bincode Vec<FileRecord> (path + mtime + size + hash + chunk range)
    └── lock                 # advisory exclusive lock during writes
```

Loader refuses if `meta.json.schema != "ast-outline.search-index.v1"`, model id mismatches, or `chunks.len() × 256 × 4 != len(embeddings.f32)`. Each binary is read via bincode with `serde::Deserialize`. Embeddings are read into memory in v1 (mmap is a v2 swap that won't change the format).

`embeddings.f32` is row-major so a single chunk's vector is one cache-friendly slice — friendly to both the in-memory and future-mmap paths.

The format reserves fields for incremental updates that v1 doesn't yet exercise:

- `meta.json.tombstones: Vec<u32>` — chunk ids logically deleted but not yet compacted.
- `FileRecord.chunk_range: (u32, u32)` — per-file `[start, end)` into `chunks.bin` so a partial rebuild can patch one file's range without rewriting the rest.

v1's update strategy is simpler: any non-empty delta triggers a full rebuild. The cheap detection path (mtime + size, only hashing on mismatch) keeps this affordable.

## Concurrency

`fs2` advisory lock at `.ast-outline/index/lock` — exclusive during writes. Two `search` calls at the same instant during a rebuild serialize; the loser sees the winner's update on its next read. All writes use `.tmp` + atomic rename so a SIGKILL mid-write leaves the previous index intact.

## Adding a new model

`ModelInfo::potion_code_16m()` is the only model wired in. To add another:

1. Add a constructor to `download::ModelInfo` listing its files (config.json, tokenizer.json, model.safetensors).
2. Verify the embedding tensor inside the safetensors is named `embeddings` and is f32 (model2vec convention).
3. If the dimension differs from 256, the `DIM` constant in [`src/search/embed.rs`](../src/search/embed.rs) needs to follow. Most of the code is generic over `DIM`, but the const is the single source of truth — bumping it requires re-indexing existing repos (the schema check in `Meta::model.dim` will catch this and force a rebuild).

The `AST_OUTLINE_MODEL_SOURCE` env var lets ops point at a custom HF-compatible mirror without code changes.
