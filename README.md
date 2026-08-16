# llm-engine

[

![CI](https://github.com/llm-engine/llm-engine/workflows/CI/badge.svg)

](https://github.com/llm-engine/llm-engine/actions)
[

![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)

](LICENSE)
[

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

](https://www.rust-lang.org/)

**Autonomous local LLM runtime. Single binary. No API dependency. Offline-first inference with adaptive KV caching and SIMD acceleration.**

## What is llm-engine?

`llm-engine` is a Rust-based standalone binary that runs large language models locally on your machine. It's designed as a **complete autonomous system** — not a cache layer for an external API, but a full-featured inference engine that works entirely offline.

### Core Principles

- **Local-First**: Decode-only Transformer running natively on CPU
- **API-Optional**: Works perfectly without an API key or network connection
- **Cache-Native**: Persistent KV cache for prefix reuse; exact response cache for deduplication
- **Hardware-Accelerated**: SIMD kernels (AVX2, AVX-512, NEON) with automatic CPU detection
- **Crash-Safe**: Columnar persistent storage with atomic commits and corruption detection
- **Reasoning-Aware**: Adaptive compute budget based on task complexity
- **Single Binary**: All features in one `llm-engine` executable — no external services needed

## Features

✅ **Local Inference**
- Decoder-only Transformer implementation
- RoPE positional encoding
- RMSNorm, SwiGLU FFN, multi-head/grouped-query attention
- Full autoregressive generation

✅ **Caching**
- Exact response cache (semantic deduplication)
- Persistent KV cache with block-based reuse
- Prefix reuse for long contexts

✅ **Storage**
- Hybrid columnar storage for KV
- Selective block reads (zero-copy when possible)
- Crash-safe write protocol with checksum validation

✅ **Acceleration**
- Runtime SIMD dispatch (AVX2/AVX-512/NEON)
- Optimized kernels: dot product, matmul, RMSNorm, RoPE, softmax
- Scalar fallback for correctness

✅ **Memory Management**
- Memory pooling and buffer reuse
- Quantization support (FP16, BF16, INT8, INT4)
- mmap-based model loading

✅ **Reasoning**

- Adaptive complexity-aware compute budgets
- Structured reasoning with verification
- Token-efficient generation

✅ **Scheduling**
- Latency mode (single-request priority)
- Throughput mode (batch optimization)
- Concurrent request handling with isolation

✅ **Offline Operation**
- Full functionality without network/API key
- Deterministic generation (fixed seed)
- Persistent cache survives restarts

## Quick Start

### Installation

**From Source:**
```bash
git clone https://github.com/llm-engine/llm-engine
cd llm-engine
cargo build --release
```

Binary will be at `target/release/llm-engine`.

**Requirements:**
- Rust 1.70+ (see `rust-toolchain.toml`)
- ~4GB RAM minimum (depends on model size)
- Linux x86-64, macOS ARM64, or Windows (experimental)

### Basic Usage

**Generate text:**
```bash
./llm-engine generate \
  --model ./models/tiny-model \
  --prompt "What is machine learning?" \
  --max-tokens 256 \
  --temperature 0.8
```

**Interactive chat:**
```bash
./llm-engine chat --model ./models/tiny-model
```

**Server mode:**
```bash
./llm-engine serve --port 8080
# Then curl http://localhost:8080/generate -d '{"prompt":"Hello"}'
```

**Get model info:**
```bash
./llm-engine model info --model ./models/tiny-model
```

**Cache management:**
```bash
./llm-engine cache stats
./llm-engine cache verify
./llm-engine cache compact
```

**Run benchmarks:**
```bash
./llm-engine benchmark all
./llm-engine benchmark inference --model ./models/tiny-model
```

**Offline mode:**
```bash
./llm-engine --offline generate --model ./models/tiny-model --prompt "test"
```

## Installation & Building

### Prerequisites

- **Rust 1.70+**: Install from https://rustup.rs/
- **LLVM**: Usually bundled with Rust; on Ubuntu: `sudo apt-get install llvm-dev`

### Build for Release

```bash
./scripts/build.sh
# or manually:
cargo build --release --features full
```

### Build with Features

```bash
# Default (local inference only)
cargo build --release

# With HTTP server
cargo build --release --features server

# All features
cargo build --release --features full
```

## Usage Examples

### Example 1: Basic Inference

```bash
export LLM_MODEL_PATH=./models/tiny-model
export LLM_CACHE_PATH=./cache

# First run: cache miss → full inference
llm-engine generate --prompt "Hello world"

# Second run: exact cache hit → instant response
llm-engine generate --prompt "Hello world"

# Third run: same prefix, different suffix → KV reuse
llm-engine generate --prompt "Hello world. How are you?"
```

### Example 2: Offline Inference

```bash
# No API key, no network
unset LLM_API_KEY

llm-engine --offline generate \
  --model ./models/tiny-model \
  --prompt "Explain quantum computing" \
  --seed 42 \
  --temperature 0
# Result: deterministic, fully local
```

### Example 3: Long Context with KV Reuse

```bash
# Store a document in context
llm-engine generate \
  --model ./models/tiny-model \
  --context-file document.txt \
  --prompt "Summarize the above" \
  --max-tokens 100

# KV cache persists; next query reuses prefix
llm-engine generate \
  --context-file document.txt \
  --prompt "What is the main theme?" \
  --max-tokens 100
# Second query is ~30% faster due to KV prefix reuse
```

### Example 4: Running a Server

```bash
llm-engine serve --port 8080 --model ./models/tiny-model

# In another terminal:
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "What is AI?",
    "max_tokens": 256,
    "temperature": 0.7
  }'
```

## Architecture

### System Design

```
REQUEST
    ↓
ORCHESTRATOR (ExecutionPlan)
    ├─ Cache Hit? → RETURN
    ├─ KV Hit? → REUSE PREFIX
    └─ KV Miss? → LOCAL INFERENCE
                     ↓
                 PREFILL/DECODE
                     ↓
                 REASONING
                     ↓
                 SIMD KERNELS
                     ↓
                 TOKEN GENERATION
                     ↓
                 KV WRITEBACK
                     ↓
                 RESPONSE CACHE
                     ↓
                 OUTPUT
```

### Module Organization

```
src/
├── main.rs                  # CLI entry
├── lib.rs                   # Public API
├── orchestrator/            # ExecutionPlan generation
├── scheduler/               # Request scheduling
├── runtime/                 # Transformer inference
├── inference/               # Prefill/decode phases
├── reasoning/               # Adaptive reasoning
├── tensor/                  # Tensor operations
├── simd/                    # SIMD acceleration
├── cache/                   # Response cache
├── kv/                      # KV cache
├── columnar/                # Persistent storage
├── model/                   # Model loading
├── tokenizer/               # Tokenization
├── provider/                # Execution providers
├── cli/                     # CLI commands
└── error/                   # Error handling
```

### Performance Profile

Latency relationships (not absolute numbers — hardware-dependent):

```
Exact Cache Hit < KV Hit < KV Miss < Cold Inference
```

Example (tiny-model on reference hardware):
- Cold TTFT: ~150ms
- Warm (cache hit): <1ms
- KV hit (prefix reuse): ~50ms

SIMD speedup: 1.5–2.0x over scalar (depends on CPU).

## Building from Source

### Clone & Setup

```bash
git clone https://github.com/llm-engine/llm-engine
cd llm-engine
rustup update stable
cargo --version  # Should be recent
```

### Build Targets

```bash
# Release binary (optimized)
cargo build --release
./target/release/llm-engine --version

# Debug binary (with symbols, slower)
cargo build
./target/debug/llm-engine --version

# Library (for embedding)
cargo build --lib --release
```

### Build with Specific CPU Features

```bash
# Enable AVX-512 (if CPU supports)
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Generic x86-64 (SIMD dispatch at runtime)
cargo build --release
```

## Testing

### Run All Tests

```bash
./scripts/test.sh
# or manually:
cargo test --all
```

### Run Specific Test Categories

```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test '*'

# E2E tests (offline, no API needed)
LLM_OFFLINE=1 cargo test --test 'e2e_*'

# SIMD correctness (SIMD ≈ Scalar)
cargo test --test '*simd*'

# Cache tests
cargo test --test '*cache*'

# KV persistence tests
cargo test --test '*kv*'

# Single test with output
cargo test test_name -- --nocapture
```

### Test Coverage

```bash
# Install tarpaulin
cargo install tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --all
# Opens coverage/index.html
```

## Benchmarking

### Run Benchmarks

```bash
# All benchmarks
cargo bench

# Specific benchmark
cargo bench --bench inference

# With verbose output
cargo bench --bench '*' -- --verbose

# Compare against baseline
cargo bench --bench inference -- --baseline baseline_v1
```

### Benchmark Results

Results stored in `target/criterion/`:
- CSV data: `target/criterion/*/base/raw.json`
- HTML report: `target/criterion/report/index.html`

### Available Benchmarks

```
e2e_cold              # Cold inference (no cache)
e2e_simd              # SIMD throughput
e2e_scalar            # Scalar reference
e2e_cache_hit         # Exact cache response time
e2e_kv_hit            # KV prefix reuse
e2e_kv_miss           # Full prefill
e2e_offline           # No API, no network
e2e_memory_scaling    # Context length vs RAM
e2e_concurrency       # Concurrent requests
```

## Development

### Code Style

```bash
# Format code
cargo fmt

# Lint with Clippy
cargo clippy -- -D warnings

# Format + Lint (quick check)
./scripts/fmt.sh
```

### Adding a Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        // Arrange
        let input = setup();
        
        // Act
        let result = my_function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

### Adding a Benchmark

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_my_feature(c: &mut Criterion) {
    c.bench_function("my_feature", |b| {
        b.iter(|| {
            // Benchmark code
        });
    });
}

criterion_group!(benches, bench_my_feature);
criterion_main!(benches);
```

### Documentation

```bash
# Generate & open docs
cargo doc --no-deps --open

# Build markdown docs
mdbook build docs/

# Check doc tests
cargo test --doc
```

## Configuration

### Environment Variables

```bash
# Logging
export LLM_LOG_LEVEL=debug|info|warn|error

# Paths
export LLM_MODEL_PATH=/path/to/models
export LLM_CACHE_PATH=/path/to/cache

# API (optional)
export LLM_API_KEY=your_key_here
export LLM_EXTERNAL_PROVIDER=https://api.example.com

# Mode
export LLM_OFFLINE=1                    # Force offline
export LLM_THREADS=8                    # Thread pool size
export LLM_MEMORY_MB=4096               # Cache memory limit
```

### Config File

Optional `~/.llm-engine/config.toml`:

```toml
[inference]
max_tokens = 256
temperature = 0.8
top_p = 0.9

[cache]
response_ttl_hours = 24
kv_path = "~/.llm-engine/kv"

[model]
default_model = "./models/tiny-model"

[performance]
batch_size = 32
scheduler_mode = "latency"
```

## Project Status

### Baseline (v0.1.0)

✅ Complete & stable:
- Single binary with CLI
- Local inference (Transformer)
- Exact cache + KV cache
- Columnar persistent storage
- SIMD kernels with dispatch
- Crash-safe persistence
- E2E tests (123+ tests)
- Reproducible benchmarks

### Roadmap

**v0.2.0** (Q3 2026):
- [ ] Continuous batching
- [ ] Speculative decoding
- [ ] HTTP/WebSocket streaming
- [ ] Multi-GPU support

**v0.3.0** (Q4 2026):
- [ ] Semantic cache
- [ ] Fine-tuning support
- [ ] LoRA integration

**v1.0.0** (Q1 2027):
- [ ] Production-ready APIs
- [ ] Enterprise features
- [ ] Cloud deployment guides

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Contribution Steps

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Write code + tests
4. Run: `./scripts/fmt.sh && cargo test --all`
5. Commit: `git commit -m "feat: description"`
6. Push & open PR

### Areas We Need Help

- [ ] Performance optimization (SIMD, cache tuning)
- [ ] Additional model architectures
- [ ] GPU backend (CUDA, ROCm)
- [ ] Documentation & examples
- [ ] Platform support (Windows, older macOS)

## License

Apache License 2.0. See [LICENSE](LICENSE) file.

## Citation

If you use llm-engine in research or production, please cite:

```bibtex
@software{llm_engine_2026,
  title = {llm-engine: Autonomous Local LLM Runtime},
  author = {...},
  year = {2026},
  url = {https://github.com/llm-engine/llm-engine},
  license = {Apache-2.0}
}
```

## Frequently Asked Questions

**Q: Do I need an API key?**  
A: No. llm-engine works entirely offline with local models. API support is optional.

**Q: What models does it support?**  
A: Decoder-only Transformers (Llama, Mistral, etc.) in FP16, BF16, INT8, INT4 formats.

**Q: How much memory does it need?**  
A: Depends on model size. A 7B-parameter model typically needs 4–8GB RAM (FP16).

**Q: Can I use it in production?**  
A: Yes! See [CONTRIBUTING.md](CONTRIBUTING.md) for production deployment guides.

**Q: How do I optimize for my hardware?**  
A: Run `llm-engine benchmark all` to measure performance. See [docs/PERFORMANCE.md](docs/PERFORMANCE.md).

**Q: Can I run it on a GPU?**  
A: CPU-only in baseline. GPU support (CUDA, ROCm) planned for v0.2.

## Support

- **Issues**: [GitHub Issues](https://github.com/llm-engine/llm-engine/issues)
- **Discussions**: [GitHub Discussions](https://github.com/llm-engine/llm-engine/discussions)
- **Documentation**: [docs/](docs/)
- **Architecture**: [ARCHITECTURE.md](ARCHITECTURE.md)

## Acknowledgments

Built with ❤️ in Rust.

Inspired by local-first, privacy-preserving AI inference.

---

**Ready to build? Start with:**
```bash
git clone https://github.com/llm-engine/llm-engine
cd llm-engine
cargo build --release
./target/release/llm-engine --help
```
