# REPOSITORY ARCHITECTURE CONTRACT — llm-engine

**Status:** FINAL BASELINE COMPLETE  
**Version:** 1.0 READY-STANDARD  
**Last Updated:** 2026-08-16

---

## TABLE OF CONTENTS

1. [Repository Identity](#1-repository-identity)
2. [Directory Structure](#2-directory-structure)
3. [Source Code Organization](#3-source-code-organization)
4. [Module Dependencies](#4-module-dependencies)
5. [Build System (Cargo)](#5-build-system-cargo)
6. [Testing Infrastructure](#6-testing-infrastructure)
7. [Benchmarking System](#7-benchmarking-system)
8. [Documentation Structure](#8-documentation-structure)
9. [Configuration & Assets](#9-configuration--assets)
10. [CI/CD Pipeline](#10-cicd-pipeline)
11. [Release Management](#11-release-management)
12. [Development Workflow](#12-development-workflow)
13. [Dependency Management](#13-dependency-management)
14. [Artifact Management](#14-artifact-management)
15. [Versioning Strategy](#15-versioning-strategy)

---

## 1. REPOSITORY IDENTITY

### 1.1 Repository Specification

```
Repository: llm-engine
Type: Rust binary + library
License: Apache 2.0 (or equivalent)
Version Control: Git
Remote: [defined by team]
Default Branch: main
Release Branch: release/*
Dev Branch: dev (optional)
```

### 1.2 Repository Properties

| Property | Value |
|----------|-------|
| **Language** | Rust (100% in src/) |
| **Build System** | Cargo (Cargo.toml) |
| **Binary Name** | llm-engine |
| **Library Name** | llm_engine |
| **MSRV** | Rust 1.70+ (stable) |
| **Edition** | 2021 |
| **Single Binary** | Yes (monolithic) |
| **Publishable** | crates.io (future) |

### 1.3 Repository Visibility

```
Public:
  ✓ Source code
  ✓ Tests
  ✓ Benchmarks
  ✓ Issues
  ✓ CI results
  ✓ Documentation

Private (if applicable):
  ✗ API keys
  ✗ Private models
  ✗ Credentials
  ✗ Sensitive config
```

---

## 2. DIRECTORY STRUCTURE

### 2.1 Root Level

```
llm-engine/
├── Cargo.toml                 # Workspace + binary package
├── Cargo.lock                 # Locked dependencies (committed)
├── rust-toolchain.toml        # Rust version: 1.70+
├── .gitignore                 # Standard Rust excludes
├── .github/
├── src/
├── tests/
├── benches/
├── models/
├── cache/
├── docs/
├── examples/
├── scripts/
├── .cargo/
├── target/                    # Built artifacts (gitignored)
├── .git/
├── README.md
├── LICENSE
├── CHANGELOG.md
├── CONTRIBUTING.md
└── ARCHITECTURE.md            # This contract
```

### 2.2 Source Code Organization (src/)

```
src/
├── main.rs                    # CLI entry point, argument parsing
│
├── lib.rs                     # Library root, public API
│
├── orchestrator/
│   ├── mod.rs                # Orchestrator module root
│   ├── executor.rs           # ExecutionPlan generation
│   ├── router.rs             # Request routing
│   └── metrics.rs            # Execution metrics
│
├── scheduler/
│   ├── mod.rs
│   ├── latency.rs            # Latency mode scheduler
│   ├── throughput.rs         # Throughput mode scheduler
│   ├── queue.rs              # Request queue
│   └── worker.rs             # Worker thread pool
│
├── runtime/
│   ├── mod.rs
│   ├── transformer.rs        # Transformer forward pass
│   ├── attention.rs          # Attention mechanism
│   ├── ffn.rs                # Feed-forward network
│   ├── embedding.rs          # Token embedding
│   └── lm_head.rs            # Language model head
│
├── inference/
│   ├── mod.rs
│   ├── prefill.rs            # Prefill phase
│   ├── decode.rs             # Decode phase
│   ├── router.rs             # Prefill/decode routing
│   └── state.rs              # Inference state machine
│
├── reasoning/
│   ├── mod.rs
│   ├── complexity.rs         # Task complexity classifier
│   ├── budget.rs             # Compute budget allocator
│   ├── verifier.rs           # Constraint/consistency checker
│   └── generator.rs          # Multi-phase generator
│
├── tensor/
│   ├── mod.rs
│   ├── tensor.rs             # Tensor type + API
│   ├── shape.rs              # Shape & stride
│   ├── dtype.rs              # Data type enum + ops
│   ├── memory.rs             # MemoryView, allocation
│   └── ops.rs                # Basic tensor operations
│
├── simd/
│   ├── mod.rs
│   ├── detector.rs           # CPU feature detection
│   ├── dispatcher.rs         # SIMD kernel dispatcher
│   ├── kernels/
│   │   ├── mod.rs
│   │   ├── dot.rs            # Dot product (scalar + SIMD)
│   │   ├── matmul.rs         # Matrix multiply
│   │   ├── normalize.rs      # RMSNorm
│   │   ├── rope.rs           # RoPE
│   │   ├── softmax.rs        # Softmax
│   │   ├── activation.rs     # GELU, etc.
│   │   └── quantized.rs      # Quantized ops
│   └── fallback.rs           # Scalar reference kernels
│
├── tokenizer/
│   ├── mod.rs
│   ├── bpe.rs                # BPE tokenizer (if applicable)
│   ├── vocab.rs              # Vocabulary management
│   └── special.rs            # Special tokens
│
├── model/
│   ├── mod.rs
│   ├── config.rs             # Model config struct
│   ├── loader.rs             # Model weight loading
│   ├── weights.rs            # Weight storage/access
│   └── compat.rs             # Version compatibility
│
├── cache/
│   ├── mod.rs
│   ├── exact.rs              # Exact response cache
│   ├── manager.rs            # Cache lifecycle
│   ├── eviction.rs           # Eviction policy
│   └── serialization.rs      # Serialization format
│
├── kv/
│   ├── mod.rs
│   ├── store.rs              # KV storage interface
│   ├── block.rs              # KV block management
│   ├── index.rs              # Block indexing
│   ├── compat.rs             # Compatibility checking
│   └── metadata.rs           # Block metadata
│
├── columnar/
│   ├── mod.rs
│   ├── storage.rs            # Columnar storage layer
│   ├── manifest.rs           # Storage manifest
│   ├── index.rs              # Selective read indexing
│   ├── write.rs              # Write protocol
│   ├── read.rs               # Read protocol
│   ├── recovery.rs           # Crash recovery
│   └── schema.rs             # Data schema definition
│
├── memory/
│   ├── mod.rs
│   ├── pool.rs               # Buffer pool
│   ├── allocator.rs          # Custom allocator
│   ├── mmap.rs               # Memory mapping
│   └── locality.rs           # Locality optimization
│
├── sampling/
│   ├── mod.rs
│   ├── algorithms.rs         # Sampling algorithms
│   ├── distribution.rs       # Probability distribution
│   └── rng.rs                # Deterministic RNG
│
├── provider/
│   ├── mod.rs
│   ├── trait.rs              # InferenceProvider trait
│   ├── local.rs              # Local provider impl
│   └── external.rs           # External provider impl
│
├── server/
│   ├── mod.rs
│   ├── http.rs               # HTTP server (optional)
│   ├── routes.rs             # HTTP routes
│   └── response.rs           # Response serialization
│
├── cli/
│   ├── mod.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── generate.rs       # llm-engine generate
│   │   ├── chat.rs           # llm-engine chat
│   │   ├── serve.rs          # llm-engine serve
│   │   ├── model.rs          # llm-engine model
│   │   ├── cache.rs          # llm-engine cache
│   │   └── benchmark.rs      # llm-engine benchmark
│   ├── args.rs               # Argument parsing (clap)
│   └── output.rs             # Output formatting
│
├── error/
│   ├── mod.rs
│   ├── kind.rs               # Error enum
│   ├── context.rs            # Error context
│   └── display.rs            # Error formatting
│
├── tracing/
│   ├── mod.rs
│   ├── span.rs               # Trace spans
│   ├── event.rs              # Trace events
│   └── collector.rs          # Span/event collector
│
├── config/
│   ├── mod.rs
│   ├── runtime.rs            # Runtime configuration
│   ├── model.rs              # Model configuration
│   └── loader.rs             # Config file loading
│
├── utils/
│   ├── mod.rs
│   ├── path.rs               # Path utilities
│   ├── io.rs                 # I/O utilities
│   └── math.rs               # Math utilities
│
└── prelude.rs                # Common imports (re-exports)
```

### 2.3 Test Organization (tests/)

```
tests/
├── common/
│   ├── mod.rs
│   ├── fixtures.rs           # Test fixtures
│   ├── tiny_model.rs         # Tiny model for CI
│   └── setup.rs              # Test setup utilities
│
├── unit/
│   ├── tensor.rs             # Tensor tests
│   ├── tokenizer.rs          # Tokenizer tests
│   ├── sampling.rs           # Sampling tests
│   ├── cache.rs              # Cache tests
│   └── kv.rs                 # KV tests
│
├── integration/
│   ├── model_load.rs         # Model loading
│   ├── inference.rs          # End-to-end inference
│   ├── cache_pipeline.rs     # Cache pipeline
│   ├── kv_reuse.rs           # KV reuse
│   └── persistence.rs        # Persistence
│
├── e2e/
│   ├── offline.rs            # Offline E2E
│   ├── cache_hit.rs          # Cache hit flow
│   ├── cache_miss.rs         # Cache miss flow
│   ├── api_disabled.rs       # API unavailable
│   ├── restart.rs            # Restart recovery
│   ├── corruption.rs         # Corruption detection
│   └── full.rs               # Complete E2E
│
├── simd/
│   ├── correctness.rs        # SIMD ≈ Scalar tests
│   ├── dispatch.rs           # Dispatcher tests
│   └── kernels.rs            # Individual kernel tests
│
└── lib.rs or mod.rs          # Test crate root
```

### 2.4 Benchmarks (benches/)

```
benches/
├── lib.rs or main.rs         # Benchmark harness
│
├── inference.rs              # E2E-COLD, E2E-SIMD
│
├── cache.rs                  # E2E-CACHE-HIT
│
├── kv.rs                     # E2E-KV-HIT, E2E-KV-MISS
│
├── simd.rs                   # Kernel benchmarks
│
├── memory.rs                 # Memory scaling
│
├── concurrency.rs            # Concurrent requests
│
├── persistence.rs            # KV persistence
│
├── columnar.rs               # Columnar storage
│
└── reasoning.rs              # Reasoning budget
```

### 2.5 Documentation (docs/)

```
docs/
├── ARCHITECTURE.md           # This contract
├── DESIGN.md                 # Design decisions
├── API.md                    # Public API documentation
├── CLI.md                    # CLI usage guide
├── BUILDING.md               # Build instructions
├── TESTING.md                # Testing guide
├── BENCHMARKING.md           # Benchmark guide
├── PERFORMANCE.md            # Performance tuning
├── TROUBLESHOOTING.md        # Troubleshooting
├── CONTRIBUTING.md           # Contribution guidelines
├── CHANGELOG.md              # Version history
│
├── guides/
│   ├── installation.md
│   ├── quickstart.md
│   ├── offline_mode.md
│   ├── cache_tuning.md
│   ├── kv_management.md
│   └── optimization.md
│
├── technical/
│   ├── tensor_runtime.md
│   ├── simd_kernels.md
│   ├── storage_format.md
│   ├── cache_hierarchy.md
│   └── scheduler.md
│
└── images/
    ├── architecture.png
    ├── pipeline.png
    └── workflows.png
```

### 2.6 Examples (examples/)

```
examples/
├── basic_inference.rs        # Simple generate example
├── cache_reuse.rs            # Cache reuse example
├── kv_persistence.rs         # KV persistence example
├── offline_mode.rs           # Offline mode example
├── reasoning.rs              # Reasoning example
├── concurrent.rs             # Concurrent requests
├── custom_sampler.rs         # Custom sampling
├── memory_tuning.rs          # Memory optimization
└── tracing.rs                # Enable tracing
```

### 2.7 Models & Cache (models/, cache/)

```
models/
├── tiny-model/               # CI test model
│   ├── manifest
│   ├── config.json
│   ├── tokenizer.json
│   └── weights.{fp16|bf16|int8|int4}
│
└── .gitkeep                  # Placeholder for user models

cache/
├── .gitkeep                  # Runtime cache directory
└── README.md                 # Cache documentation
```

### 2.8 Scripts (scripts/)

```
scripts/
├── build.sh                  # Build script
├── test.sh                   # Run all tests
├── bench.sh                  # Run benchmarks
├── ci.sh                     # CI steps
├── fmt.sh                    # Format + lint
├── release.sh                # Release build
├── profile.sh                # CPU profiling
├── download_models.sh        # Download test models
└── generate_docs.sh          # Generate documentation
```

### 2.9 GitHub Actions (.github/)

```
.github/
├── workflows/
│   ├── ci.yml                # Main CI pipeline
│   ├── e2e.yml               # E2E tests
│   ├── benchmark.yml         # Benchmark runs
│   ├── release.yml           # Release workflow
│   └── docs.yml              # Documentation build
│
└── ISSUE_TEMPLATE/
    ├── bug_report.md
    ├── feature_request.md
    └── performance.md
```

---

## 3. SOURCE CODE ORGANIZATION

### 3.1 Module Dependency Graph

```
main.rs
  └─ cli/
      └─ commands/
          ├─ generate, chat, serve, model, cache, benchmark
          └─ All use: orchestrator, scheduler, model, tokenizer

lib.rs (public API)
  ├─ orchestrator/ (core)
  ├─ scheduler/ (coordination)
  ├─ runtime/ (inference)
  ├─ inference/ (prefill/decode)
  ├─ reasoning/ (adaptive)
  ├─ tensor/ (computation)
  ├─ simd/ (acceleration)
  ├─ cache/ (response dedup)
  ├─ kv/ (prefix reuse)
  ├─ columnar/ (persistence)
  ├─ memory/ (allocation)
  ├─ sampling/ (generation)
  ├─ provider/ (execution)
  └─ error/ (error handling)

Dependencies flow:
  CLI → Orchestrator
  Orchestrator → Scheduler, Runtime, Reasoning, Cache, KV
  Runtime → Tensor, SIMD, Inference
  Inference → Runtime, KV, Memory
  Tensor → SIMD, Memory
  KV → Columnar, Cache
  Columnar → Memory
```

### 3.2 Public API (lib.rs)

```rust
// lib.rs: Minimal public surface

pub use orchestrator::ExecutionPlan;
pub use scheduler::SchedulerMode;
pub use runtime::TransformerRuntime;
pub use inference::{InferenceRequest, InferenceResponse};
pub use reasoning::ReasoningLevel;
pub use cache::CacheManager;
pub use kv::KVStore;
pub use tokenizer::Tokenizer;
pub use model::Model;
pub use provider::InferenceProvider;
pub use error::{Error, Result};

// Internal modules: not public (implementation detail)
mod orchestrator;
mod scheduler;
// ... etc
```

### 3.3 Module Visibility

```
PUBLIC (in lib.rs):
  ✓ ExecutionPlan
  ✓ InferenceRequest
  ✓ InferenceResponse
  ✓ Model
  ✓ Tokenizer
  ✓ Error
  ✓ InferenceProvider trait

INTERNAL (not in lib.rs, used only internally):
  ✗ TransformerRuntime details
  ✗ AttentionKernel internals
  ✗ ColumnlarStorage details
  ✗ SIMD dispatcher internals
  ✗ Scheduler queue details

REASON:
  API stability. Users should not depend on internal details.
  Allows refactoring without breaking semver.
```

### 3.4 Code Style & Convention

```
✓ Naming:
  - snake_case for functions, variables
  - CamelCase for types, traits
  - SCREAMING_SNAKE_CASE for constants

✓ Module organization:
  - One public type per module (generally)
  - Private helpers in same file
  - Tests at bottom of file (#[cfg(test)])

✓ Error handling:
  - Result<T> ubiquitously
  - Custom Error enum for context
  - No panics in production code (except startup validation)

✓ Documentation:
  - /// doc comments on public items
  - Examples in doc comments
  - Module-level documentation

✓ Safety:
  - unsafe { } only in SIMD kernels, memory layer
  - Document invariants for unsafe blocks
```

---

## 4. MODULE DEPENDENCIES

### 4.1 Dependency Graph (Text Format)

```
LAYER 0 (No dependencies on other modules)
├─ error/            (std + anyhow/thiserror)
├─ utils/            (std)
├─ config/           (std + serde/toml)
└─ tracing/          (std + tracing crate)

LAYER 1 (Depends on Layer 0)
├─ tensor/           (layer0 + ndarray or similar)
├─ dtype/            (layer0)
└─ memory/           (layer0)

LAYER 2 (Depends on Layer 0–1)
├─ simd/             (layer1 + std_simd)
├─ tokenizer/        (layer0 + vocab)
└─ sampling/         (layer1)

LAYER 3 (Depends on Layer 0–2)
├─ model/            (layer0-2 + config + tokenizer)
├─ kv/               (layer1 + memory)
└─ columnar/         (layer1 + memory)

LAYER 4 (Depends on Layer 0–3)
├─ runtime/          (layer0-3 + simd + tensor)
├─ cache/            (layer0-3 + kv)
└─ inference/        (layer0-4 + runtime)

LAYER 5 (Depends on Layer 0–4)
├─ reasoning/        (layer0-4 + inference)
└─ provider/         (layer0-4 + runtime)

LAYER 6 (Depends on Layer 0–5)
├─ orchestrator/     (layer0-5 + reasoning)
└─ scheduler/        (layer0-5)

LAYER 7 (Depends on Layer 0–6)
├─ server/           (layer0-6 + http framework)
└─ cli/              (layer0-6 + clap)

LAYER 8 (Top-level)
├─ main.rs           (cli)
├─ lib.rs            (public API, re-exports)
└─ prelude.rs        (common imports)
```

### 4.2 Circular Dependency Prevention

**RULE:** No circular dependencies allowed.

Enforcement mechanism:
```bash
# Add to CI:
cargo build --all-targets
# If it compiles, no cycles exist (Rust enforces at compile time)
```

If cycle appears:
1. Identify the circular modules
2. Extract shared logic to new module at lower layer
3. Both modules depend on extracted module

Example:
```
Bad:  A ↔ B

Fix:  A ← Shared → B
```

---

## 5. BUILD SYSTEM (CARGO)

### 5.1 Cargo.toml Structure

```toml
[workspace]
members = [".", "tools"]
resolver = "2"

[package]
name = "llm-engine"
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
license = "Apache-2.0"
authors = ["..."]
repository = "https://github.com/..."
documentation = "https://docs.rs/llm_engine"
readme = "README.md"

[dependencies]
# Core
tokio = { version = "1.35", features = ["full"] }
anyhow = "1.0"
thiserror = "1.0"

# Tensor/Math
ndarray = "0.15"
ndarray-rand = "0.15"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"
toml = "0.8"

# Math
rand = "0.8"

# Tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# CLI
clap = { version = "4.4", features = ["derive"] }

# HTTP (server feature)
axum = { version = "0.7", optional = true }
tower = { version = "0.4", optional = true }

# Performance
parking_lot = "0.12"
crossbeam = "0.8"

# Testing
proptest = "1.4"

[dev-dependencies]
criterion = "0.5"
tempfile = "3.8"
mockito = "1.2"

[features]
default = ["local"]
local = []
server = ["axum", "tower"]
full = ["local", "server"]

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
strip = true

[profile.bench]
opt-level = 3
lto = "thin"
codegen-units = 1
```

### 5.2 Build Targets

```bash
# Binary
cargo build --release
  → target/release/llm-engine

# Library
cargo build --lib
  → target/release/libllm_engine.{a,so,dylib}

# Tests
cargo test

# Benchmarks
cargo bench

# Documentation
cargo doc --open
```

### 5.3 Feature Flags

| Feature | Default | Purpose |
|---------|---------|---------|
| `local` | YES | Local inference (always included) |
| `server` | NO | HTTP server support |
| `full` | NO | All features |

```bash
# Build with features
cargo build --release --features server

# Build without default features
cargo build --release --no-default-features --features local
```

---

## 6. TESTING INFRASTRUCTURE

### 6.1 Test Organization

```
Unit Tests
├─ src/ (inline #[cfg(test)])
├─ tests/unit/ (external)
└─ Run: cargo test --lib

Integration Tests
├─ tests/integration/
└─ Run: cargo test --test '*'

E2E Tests
├─ tests/e2e/
└─ Run: cargo test --test 'e2e_*'

Doctests
├─ src/ (/// examples in doc comments)
└─ Run: cargo test --doc
```

### 6.2 Test Fixtures

```
tests/common/fixtures/
├── tiny-model/              # CI model
│   ├── manifest
│   ├── config.json
│   ├── tokenizer.json
│   └── weights.bf16
│
└── prompts/
    ├── simple.txt
    ├── reasoning.txt
    └── long_context.txt
```

### 6.3 Test Macros & Helpers

```rust
// tests/common/mod.rs

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $tolerance:expr) => {
        assert!(($a - $b).abs() <= $tolerance);
    };
}

pub fn setup_test_model() -> Model { ... }
pub fn setup_test_cache() -> CacheManager { ... }
```

### 6.4 Test Execution Matrix

```
Default (all tests):
  cargo test

By category:
  cargo test --lib              # Unit tests
  cargo test --test '*'         # Integration tests
  cargo test --test 'e2e_*'     # E2E tests

By feature:
  cargo test --features server  # With server feature

Offline only (no network):
  LLM_OFFLINE=1 cargo test

Single test:
  cargo test test_name -- --nocapture
```

### 6.5 CI Test Requirements

| Category | Count | Status |
|----------|-------|--------|
| Unit tests | 50+ | REQUIRED |
| Integration | 20+ | REQUIRED |
| E2E | 15+ | REQUIRED |
| Doc tests | 10+ | REQUIRED |
| SIMD correctness | 8+ | REQUIRED |
| Cache tests | 10+ | REQUIRED |
| KV tests | 10+ | REQUIRED |
| **Total** | **123+** | **ALL PASS** |

---

## 7. BENCHMARKING SYSTEM

### 7.1 Benchmark Organization

```
benches/
├── lib.rs                     # Benchmark harness (Criterion)
├── inference.rs               # E2E inference benchmarks
├── cache.rs                   # Cache benchmarks
├── kv.rs                      # KV benchmarks
├── simd.rs                    # SIMD kernel benchmarks
├── memory.rs                  # Memory scaling
└── ... (others)

Run:
  cargo bench                   # All benchmarks
  cargo bench e2e_             # E2E benchmarks only
```

### 7.2 Benchmark Framework

Using **Criterion.rs**:

```rust
// benches/inference.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn e2e_cold(c: &mut Criterion) {
    c.bench_function("e2e_cold_tiny_model", |b| {
        b.iter(|| {
            // Benchmark code
        });
    });
}

criterion_group!(benches, e2e_cold);
criterion_main!(benches);
```

### 7.3 Benchmark Output

```
Each benchmark produces:
  - CSV results: target/criterion/
  - HTML report: target/criterion/report/index.html
  - Comparison graphs (if baseline exists)
```

### 7.4 Baseline & Regression

```bash
# Establish baseline on reference hardware
cargo bench --bench inference -- --save-baseline baseline_1

# Compare against baseline
cargo bench --bench inference -- --baseline baseline_1
```

---

## 8. DOCUMENTATION STRUCTURE

### 8.1 Documentation Levels

```
1. Code Documentation
   ├─ Module doc comments
   ├─ Function doc comments (///
   ├─ Examples in doc comments
   └─ Run: cargo doc --open

2. Inline Documentation
   ├─ // Comments in code
   └─ Explain complex algorithms

3. Guide Documentation
   ├─ docs/guides/ (user guides)
   ├─ docs/technical/ (technical deep dives)
   └─ Markdown files

4. API Documentation
   ├─ docs/API.md (public API reference)
   ├─ Cargo doc (rustdoc)
   └─ Generated from code

5. Development Documentation
   ├─ CONTRIBUTING.md (contribution guide)
   ├─ docs/TESTING.md (testing guide)
   ├─ docs/BENCHMARKING.md (benchmark guide)
   └─ docs/BUILDING.md (build instructions)
```

### 8.2 Document Ownership

| Document | Owner | Update Frequency |
|----------|-------|------------------|
| README.md | Core team | Per release |
| ARCHITECTURE.md | Core team | Design changes |
| API.md | API reviewer | Per API change |
| CHANGELOG.md | Release manager | Per release |
| docs/ | Contributors | As needed |
| Code comments | Code authors | Per commit |

### 8.3 Documentation Build

```bash
# Generate Rust docs
cargo doc --no-deps --open

# Generate guide (if using mdbook)
mdbook build docs/

# Full documentation
scripts/generate_docs.sh
```

---

## 9. CONFIGURATION & ASSETS

### 9.1 Configuration Files

```
llm-engine/ (root)
├── rust-toolchain.toml        # Rust version requirement
├── .cargo/config.toml         # Cargo settings (optional)
├── clippy.toml                # Clippy lints (optional)
└── .github/workflows/*.yml    # CI/CD configuration
```

### 9.2 Runtime Configuration

```
models/
└── tiny-model/
    ├── manifest               # Version, schema
    ├── config.json            # Model hyperparameters
    ├── tokenizer.json         # Vocab, special tokens
    └── weights.bf16           # Model weights

cache/
└── (empty at start, populated at runtime)

~/.llm-engine/               # User config (optional)
├── config.toml
└── cache/
```

### 9.3 Environment Variables

```
Development:
  LLM_LOG_LEVEL=debug
  LLM_OFFLINE=0
  RUST_BACKTRACE=1

Production:
  LLM_LOG_LEVEL=info
  LLM_OFFLINE=1 (for edge devices)
  LLM_CACHE_PATH=/var/cache/llm-engine
  LLM_MODEL_PATH=/opt/models

CI/Testing:
  LLM_OFFLINE=1
  LLM_CACHE_PATH=/tmp/llm-engine-cache
  LLM_LOG_LEVEL=warn
```

---

## 10. CI/CD PIPELINE

### 10.1 CI Workflow Structure (.github/workflows/ci.yml)

```yaml
name: CI

on:
  push:
    branches: [main, dev]
  pull_request:
    branches: [main]

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings

  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - run: ./target/release/llm-engine --version

  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all

  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: |
          LLM_OFFLINE=1 cargo test --test 'e2e_*'

  benchmark:
    runs-on: ubuntu-latest
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo bench --bench inference -- --verbose

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: |
          cargo install tarpaulin
          cargo tarpaulin --out Xml --all
      - uses: codecov/codecov-action@v3
```

### 10.2 E2E Test Workflow (.github/workflows/e2e.yml)

```yaml
name: E2E Tests

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM

jobs:
  offline-e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: |
          LLM_OFFLINE=1 \
          LLM_API_KEY= \
          cargo test --test 'e2e_*' -- --nocapture

  persistence-e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --test '*persistence*'

  simd-e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --test '*simd*'
```

### 10.3 Benchmark Workflow (.github/workflows/benchmark.yml)

```yaml
name: Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Run benchmarks
        run: cargo bench --bench '*' -- --verbose

      - name: Compare with baseline
        run: |
          # Store results
          mkdir -p benchmark-results
          cp -r target/criterion/* benchmark-results/

      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: benchmark-results/
```

### 10.4 Release Workflow (.github/workflows/release.yml)

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Build release
        run: cargo build --release

      - name: Run tests
        run: cargo test --release

      - name: Create release
        uses: softprops/action-gh-release@v1
        with:
          files: target/release/llm-engine
```

### 10.5 CI Status Requirements

```
Must PASS before merge:
  ✓ Format (cargo fmt --check)
  ✓ Lint (cargo clippy)
  ✓ Build (cargo build --release)
  ✓ Unit tests (cargo test --lib)
  ✓ Integration tests (cargo test --test '*')
  ✓ E2E tests (cargo test --test 'e2e_*')

Should PASS (warning if failure):
  ~ Benchmarks (no regression > 15%)
  ~ Coverage (maintain > 70%)

Can be SKIPPED:
  - Benchmark on external PRs (if resource-limited)
```

---

## 11. RELEASE MANAGEMENT

### 11.1 Versioning Strategy

**Semantic Versioning (semver):** `MAJOR.MINOR.PATCH`

```
0.1.0 - Initial baseline release
0.2.0 - Batching + speculative decoding
0.3.0 - Semantic cache
1.0.0 - Production ready

MAJOR = Breaking changes to public API
MINOR = New features (backward compatible)
PATCH = Bug fixes (backward compatible)
```

### 11.2 Release Branch Strategy

```
main                          # Stable, release-ready
  ├─ v0.1.0 (tag)
  ├─ v0.2.0 (tag)
  └─ v1.0.0 (tag)

dev                           # Development (optional)
  └─ Features in progress

release/v1.0.0                # Release branch
  ├─ Hotfixes
  └─ Backports from main
```

### 11.3 Release Checklist

Before tagging:

```
✓ Update CHANGELOG.md
✓ Update version in Cargo.toml
✓ Run: cargo test --all
✓ Run: cargo doc --no-deps
✓ Review breaking changes
✓ Tag: git tag v1.0.0
✓ Push: git push origin v1.0.0
✓ GitHub Actions release workflow runs
✓ Publish to crates.io (if public)
```

### 11.4 Changelog Format

```markdown
## [1.0.0] - 2026-08-16

### Added
- Single binary with offline inference
- KV cache with persistent columnar storage
- SIMD acceleration
- Adaptive reasoning engine
- Crash-safe persistence

### Changed
- [BREAKING] Model format updated to v2
- Cache API redesigned for clarity

### Fixed
- KV cache corruption on concurrent requests
- Memory leak in tensor arena

### Security
- Validate all user input in CLI

[Unreleased]: https://github.com/.../compare/v1.0.0...HEAD
[1.0.0]: https://github.com/.../releases/v1.0.0
```

---

## 12. DEVELOPMENT WORKFLOW

### 12.1 Development Environment Setup

```bash
# Clone repository
git clone https://github.com/llm-engine/llm-engine
cd llm-engine

# Install Rust (if not already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify Rust version
rustc --version  # Should match rust-toolchain.toml

# Install recommended tools
cargo install clippy
cargo install rustfmt
cargo install cargo-doc

# Build project
cargo build --release

# Run tests
cargo test

# View documentation
cargo doc --open
```

### 12.2 Development Workflow

```
1. Pick an issue/feature

2. Create branch:
   git checkout -b feature/my-feature

3. Implement:
   - Write code in src/
   - Add tests in tests/
   - Update docs/

4. Format & lint:
   ./scripts/fmt.sh

5. Test:
   ./scripts/test.sh

6. Benchmark (if performance-sensitive):
   cargo bench --bench <relevant>

7. Commit:
   git commit -m "feat: description"

8. Push & open PR:
   git push origin feature/my-feature

9. CI runs automatically

10. Review & merge
```

### 12.3 Commit Message Convention

```
<type>(<scope>): <subject>

<body>

<footer>

Types:
  feat:     New feature
  fix:      Bug fix
  docs:     Documentation
  test:     Test
  perf:     Performance improvement
  refactor: Code refactoring
  ci:       CI/CD changes
  chore:    Build/dependency changes

Example:
  feat(cache): implement exact cache with TTL

  Add support for time-based cache expiration.
  Implements LRU eviction when capacity exceeded.

  Fixes #123
```

### 12.4 Code Review Checklist

```
✓ Follows module conventions
✓ No circular dependencies
✓ Tests added (unit + integration)
✓ Documentation updated
✓ No unsafe code (except justified)
✓ Error handling complete
✓ Performance-sensitive code benchmarked
✓ No breaking API changes (or semver bumped)
✓ CI passes
✓ Changelog entry added
```

---

## 13. DEPENDENCY MANAGEMENT

### 13.1 Dependency Policy

**Principles:**
- Minimal dependencies
- Well-maintained crates
- Prefer std library where possible
- Review new dependencies in PR

### 13.2 Core Dependencies

```toml
# Async runtime
tokio = "1.35"              # Multithreading, concurrency

# Error handling
anyhow = "1.0"              # Context-aware errors
thiserror = "1.0"           # Error enum derive

# Math/Tensor
ndarray = "0.15"            # N-dimensional arrays
rand = "0.8"                # Random number generation

# Serialization
serde = "1.0"               # Serialization framework
bincode = "1.3"             # Binary encoding

# Tracing
tracing = "0.1"             # Distributed tracing

# CLI
clap = "4.4"                # Command-line parsing

# Performance
parking_lot = "0.12"        # Faster mutexes
crossbeam = "0.8"           # Concurrency utilities
```

### 13.3 Avoiding Dependency Bloat

```
✓ Only add if solving real problem
✓ Consider std library alternatives
✓ Minimize transitive dependencies
✓ Prefer crates with few dependencies
✓ Audit new crate security/maintenance

Before adding: cargo tree
After adding: review tree growth
```

### 13.4 Updating Dependencies

```bash
# Check for outdated dependencies
cargo outdated

# Update with caution
cargo update
cargo test  # Verify no breakage

# Update single dependency
cargo update -p ndarray
```

---

## 14. ARTIFACT MANAGEMENT

### 14.1 Build Artifacts

```
target/
├── release/
│   ├── llm-engine                    # Binary (main output)
│   ├── libllm_engine.{a,so,dylib}   # Library
│   └── deps/                         # Dependencies
│
├── debug/
│   ├── llm-engine                    # Debug binary
│   └── deps/
│
└── criterion/                        # Benchmark results
    ├── baseline_1/
    ├── benchmark_name/
    └── report/index.html
```

### 14.2 Distribution Artifacts

```
Release (per tag):
  ✓ Source tarball: llm-engine-v1.0.0.tar.gz
  ✓ Linux binary: llm-engine-v1.0.0-linux-x86_64
  ✓ macOS binary: llm-engine-v1.0.0-macos-arm64
  ✓ Windows binary: llm-engine-v1.0.0-windows-x86_64.exe
  ✓ Cargo crate: llm_engine-1.0.0.crate

Created via:
  cargo build --release
  cargo package
  cargo publish  # (if public)
```

### 14.3 Cache & KV Artifacts (at Runtime)

```
~/.llm-engine/
├── cache/
│   ├── manifest
│   ├── index/
│   ├── metadata/
│   ├── responses/
│   ├── keys/
│   └── values/
│
└── models/
    └── tiny-model/
        ├── manifest
        ├── config.json
        ├── tokenizer.json
        └── weights.bf16
```

---

## 15. VERSIONING STRATEGY

### 15.1 File Versions

```
Cargo.toml:         package version   (0.1.0)
Cargo.lock:         snapshot          (always committed)
rust-toolchain.toml: Rust version     (1.70+)
CHANGELOG.md:       release history
.git:               commit history
```

### 15.2 API Versioning

```
Public API (lib.rs):  MUST follow semver
  ✓ orchestrator::ExecutionPlan
  ✓ inference::InferenceRequest/Response
  ✓ Error type

Internal API:  NOT bound by semver
  ✗ TransformerRuntime internals
  ✗ SIMD kernel details
  ✗ Storage layer details
```

### 15.3 Binary Versioning

```
llm-engine --version
  → llm-engine v0.1.0

llm-engine --help
  → Shows version + all commands

Embedded in binary:
  const VERSION: &str = env!("CARGO_PKG_VERSION");
```

---

## SUMMARY: REPOSITORY STRUCTURE STANDARDS

### Enforcement

```
Automated checks in CI:

✓ cargo fmt --check          # Code formatting
✓ cargo clippy               # Linting
✓ cargo test                 # All tests
✓ cargo build --release      # Release build
✓ cargo doc --no-deps        # Documentation builds
✓ Dependency audit           # No unsafe crates
✓ Coverage threshold > 70%   # Code coverage
```

### File Organization Checklist

```
Before accepting PR, verify:

✓ New module in appropriate layer
✓ No circular dependencies
✓ Public API minimal (only in lib.rs)
✓ Tests in tests/ (or #[cfg(test)])
✓ Documentation complete
✓ No breaking changes (or CHANGELOG updated)
✓ Benchmarks if performance-sensitive
✓ CI passes
```

### Directory Ownership

| Directory | Owner | Changes |
|-----------|-------|---------|
| src/ | Core team | Code review required |
| tests/ | QA/Reviewers | Code review required |
| benches/ | Performance team | Code review + benchmark approval |
| docs/ | Documentation lead | Direct commits acceptable |
| .github/workflows/ | DevOps | Code review required |
| scripts/ | DevOps | Code review required |

---

**This Repository Architecture Contract is the source of truth for physical and logical organization of the llm-engine codebase.**
