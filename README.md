# llm-engine

Autonomous local LLM runtime implemented as a single Rust binary with local inference, deterministic tiny-model testing, efficient KV reuse, and crash-safe persistence.

## Quick start

```bash
cargo run --bin llm-engine -- generate --prompt "hello world" --max-tokens 8
cargo test --all-targets
```

## Features

- Local inference with no API key required
- Offline-first execution model
- Exact response cache and KV reuse
- SIMD-aware acceleration with scalar fallback
- Persistent columnar storage for crash-safe recovery
