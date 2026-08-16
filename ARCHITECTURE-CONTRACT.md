# ARCHITECTURE CONTRACT — llm-engine

**Status:** FINAL BASELINE END-TO-END  
**Version:** 1.0 READY-STANDARD  
**Last Updated:** 2026-08-16

---

## TABLE OF CONTENTS

1. [Fundamental Contract](#1-fundamental-contract)
2. [Technical Identity](#2-technical-identity)
3. [Core Invariants](#3-core-invariants)
4. [System Architecture](#4-system-architecture)
5. [Component Specifications](#5-component-specifications)
6. [Tensor & Memory Layer](#6-tensor--memory-layer)
7. [SIMD Acceleration](#7-simd-acceleration)
8. [Cache Hierarchy](#8-cache-hierarchy)
9. [KV Cache Specification](#9-kv-cache-specification)
10. [Columnar Storage](#10-columnar-storage)
11. [Local LLM Runtime](#11-local-llm-runtime)
12. [Cognitive Orchestrator](#12-cognitive-orchestrator)
13. [Reasoning Engine](#13-reasoning-engine)
14. [Scheduler & Execution](#14-scheduler--execution)
15. [External Provider Integration](#15-external-provider-integration)
16. [CLI & Deployment](#16-cli--deployment)
17. [Testing & CI Specification](#17-testing--ci-specification)
18. [Benchmark Specification](#18-benchmark-specification)
19. [Product Matrix](#19-product-matrix)
20. [Operational Workflows](#20-operational-workflows)
21. [Quality Gates & Exit Criteria](#21-quality-gates--exit-criteria)

---

## 1. FUNDAMENTAL CONTRACT

### 1.1 Definition

```
llm-engine is a Rust-based single-binary autonomous LLM runtime 
that executes decoder-only Transformers locally, using persistent 
hybrid-columnar KV storage, hierarchical caching, block-based KV 
reuse, memory-mapped model weights, adaptive reasoning, and 
SIMD-accelerated tensor kernels.

External LLM API is optional and never required for inference 
or reasoning.
```

### 1.2 Architectural Axiom

```
API unavailable ≠ LLM unavailable
Cache miss ≠ inference failure  
Cache ≠ model
Model = primary reasoning engine

Local model always available → inference always possible
```

### 1.3 Core Thesis

The system is **not** a cache layer for an external API.  
The system **is** a complete local LLM runtime that uses caching 
to minimize computation and SIMD to maximize speed of required computation.

---

## 2. TECHNICAL IDENTITY

### 2.1 System Properties

| Property | Value |
|----------|-------|
| **Name** | llm-engine |
| **Category** | Autonomous Local LLM Runtime |
| **Architecture** | Cache-Native Autonomous Inference |
| **Language** | Rust (stable) |
| **Build** | Cargo + LLVM |
| **Deployment** | Single binary |
| **Primary Inference** | Local model |
| **External API** | Optional |
| **Persistent Memory** | Hybrid columnar storage |
| **Acceleration** | SIMD + memory locality + operator optimization |
| **Operating Modes** | Offline, Online, Hybrid |

### 2.2 Compilation Profile

```toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
strip = true
```

**Key:** `target-cpu=native` NOT required; SIMD dispatch at runtime.

---

## 3. CORE INVARIANTS

These invariants must NEVER be violated. Any violation is an architectural bug.

### 3.1 Invariant #1: Local Model Always Available

```
Condition: LLM_API_KEY absent, network unavailable, cache empty
Result: Local inference succeeds ✓
Failure: Application unable to run = ARCHITECTURAL BUG
```

### 3.2 Invariant #2: Cache Does Not Change Semantics

```
Path A: Full recomputation (cache disabled)
Path B: Cache enabled

Input: same prompt, same config, deterministic generation
Result: output_A == output_B

Cache is optimization layer, not semantic modifier.
```

### 3.3 Invariant #3: KV Cache Correctness

```
Path A: Recomputation of prefix tokens
Path B: KV reuse of prefix tokens

Result: logits ≈ identical (within dtype tolerance)
        tokens_A == tokens_B (deterministic sampling)

KV is optimization, not approximation.
```

### 3.4 Invariant #4: Single Binary Completeness

```
Binary: ./llm-engine

Contains:
  ✓ CLI
  ✓ Local API
  ✓ Orchestrator
  ✓ Tokenizer
  ✓ Model Loader
  ✓ Transformer Runtime
  ✓ Tensor Runtime
  ✓ SIMD Dispatcher
  ✓ KV Cache Manager
  ✓ Columnar Storage
  ✓ Response Cache
  ✓ Scheduler
  ✓ Sampling
  ✓ Optional Provider Integration
  ✓ Benchmark suite
  ✓ Diagnostics

No external services required.
```

### 3.5 Invariant #5: Offline Capability

```
llm-engine --offline

Removes:
  API calls
  Network requests
  External provider

Preserves:
  Local inference ✓
  Reasoning ✓
  KV cache ✓
  Response cache ✓
  Persistence ✓
```

### 3.6 Invariant #6: Crash-Safe Storage

```
Process crash during persistence:

Partial block → detected → discarded
Incomplete write → detected → recovered
Valid cache → preserved

Restart: runtime starts, valid cache available for reuse
```

---

## 4. SYSTEM ARCHITECTURE

### 4.1 Request Pipeline

```
┌──────────────────────────────────────────────────────────┐
│                       REQUEST                            │
└──────────────────────┬───────────────────────────────────┘
                       │
                       ▼
        ┌──────────────────────────────┐
        │    COGNITIVE ORCHESTRATOR    │
        │  (ExecutionPlan generator)   │
        └──────────────┬───────────────┘
                       │
       ┌───────────────┼───────────────┐
       ▼               ▼               ▼
┌─────────────┐ ┌─────────────┐ ┌──────────────┐
│Response     │ │KV Cache     │ │Local LLM     │
│Cache        │ │Columnar     │ │Runtime       │
└──────┬──────┘ └──────┬──────┘ └──────┬───────┘
       │               │              │
       │               │              ▼
       │               │        ┌──────────────┐
       │               │        │  SIMD Core   │
       │               │        │ (Kernels)    │
       │               │        └──────┬───────┘
       │               │               │
       └───────────────┴───────────────┘
                       │
                       ▼
            CACHE WRITEBACK & RESPONSE
                       │
                       ▼
                    OUTPUT

OPTIONAL: External Provider integration (via Orchestrator)
```

### 4.2 Execution Phases

```
REQUEST NORMALIZATION
         ↓
    ORCHESTRATOR
         ↓
    EXACT CACHE?
    ├─ HIT → RETURN
    └─ MISS → KV CACHE?
              ├─ HIT → REUSE PREFIX
              └─ MISS → LOCAL PREFILL
                        ↓
                   INCREMENTAL DECODE
                   (token-by-token)
                        ↓
                   REASONING ENGINE
                        ↓
                   VERIFY (optional)
                        ↓
                   KV WRITEBACK
                        ↓
              RESPONSE CACHE WRITE
                        ↓
                     OUTPUT
```

---

## 5. COMPONENT SPECIFICATIONS

### 5.1 Orchestrator (ExecutionPlan Generator)

**Input:**
- Cache state
- KV state
- Model availability
- RAM availability
- CPU SIMD capability
- Context length
- Generation parameters
- API availability

**Output:**
```rust
ExecutionPlan {
    cache_strategy: CacheStrategy,      // Exact/Miss
    kv_strategy: KVStrategy,            // Hit/Miss/Partial
    inference_strategy: InferenceType,  // Prefill/Decode/Both
    simd_backend: SIMDBackend,          // AVX512/AVX2/NEON/Scalar
    provider: ExecutionProvider,        // Local/External
    max_tokens: u32,
    reasoning_budget: ReasoningLevel,   // Simple/Medium/Complex
    scheduler_mode: SchedulerMode,      // Latency/Throughput
}
```

**Responsibility:**
- Find minimum-compute execution path
- Ensure correctness throughout
- Route to appropriate provider (always local as fallback)

---

### 5.2 Tokenizer

**Requirements:**
- Deterministic encoding/decoding
- Support for special tokens
- Vocabulary consistency with model
- Independent of external API

**Output:**
- Token IDs
- Position information
- Attention mask (if applicable)

---

### 5.3 Model Loader

**Capabilities:**
- Load model from disk
- Memory-map weights
- Validate model signature
- Load tokenizer
- Support format:
  - FP32 (reference)
  - FP16 (primary)
  - BF16 (primary)
  - INT8 (optimization)
  - INT4 (optimization)

**Invariant:**
Model loading failure → fatal (no local model = cannot execute)

---

### 5.4 Scheduler

**Two Modes:**

| Mode | Optimization | Use Case |
|------|--------------|----------|
| **Latency** | TTFT, p95 latency | Interactive, single request |
| **Throughput** | Batching, sustained tokens/sec | Batch processing, concurrent |

**Responsibilities:**
- Queue requests
- Classify by complexity
- Allocate compute resources
- Prevent oversubscription
- Manage prefill/decode separation

---

### 5.5 Sampling

**Algorithms Supported:**
- Greedy (argmax)
- Temperature scaling
- Top-K filtering
- Top-P (nucleus) sampling
- Repetition penalty
- Seed-based determinism

**Determinism Contract:**

```
temperature = 0 AND seed = fixed
         ↓
    output identical across runs
```

---

### 5.6 Memory Manager

**Strategies:**

```
Model → mmap
         ↓
    Tensor View
         ↓
   Compute Buffer
         ↓
   SIMD Kernel
```

**Principles:**
- Allocate once, reuse
- Zero-copy where possible
- Aligned allocations
- Buffer pooling
- KV block reuse

---

## 6. TENSOR & MEMORY LAYER

### 6.1 Tensor Runtime API

**Core Types:**

```rust
Tensor {
    shape: Shape,
    stride: Stride,
    dtype: DType,
    device: Device,        // CPU-only for baseline
    buffer: MemoryView,
}

MemoryView {
    data: *const u8,
    size: usize,
    alignment: usize,
}
```

**Memory Principle:**
Not CPU-locked; architecture extensible for GPU/TPU future.

### 6.2 Layout Strategy

**Storage Layout:** Columnar (for persistence)

**Compute Layout:** Contiguous, packed, tiled, SIMD-aligned

**Transform Pipeline:**

```
Columnar Storage
       ↓
Block Selection
       ↓
Prefetch
       ↓
Compute Packing
       ↓
SIMD-aligned Buffer
       ↓
Kernel Execution
```

**Key Invariant:**
Do NOT perform intensive tensor ops directly on persistent columnar layout.

### 6.3 Quantization Support

**Priority Implementation Order:**

1. FP16/BF16 (primary inference)
2. INT8 (compute reduction)
3. INT4 (memory reduction)

**KV Quantization Note:**
Lower precision ≠ automatically faster on decode.  
Dequantization cost must be benchmarked with memory savings.

---

## 7. SIMD ACCELERATION

### 7.1 Runtime CPU Detection

```
CPU Detection
     │
     ├─ x86-64
     │  ├─ AVX-512 → AVX-512 kernels
     │  └─ AVX2    → AVX2 kernels
     │
     ├─ ARM64
     │  └─ NEON    → NEON kernels
     │
     └─ Fallback → Scalar kernels
```

### 7.2 Dispatcher Pattern

```rust
fn execute_kernel(input: &Tensor, simd: SIMDCapability) -> Tensor {
    match simd {
        AVX512 => kernel_avx512(input),
        AVX2   => kernel_avx2(input),
        NEON   => kernel_neon(input),
        Scalar => kernel_scalar(input),  // Always available
    }
}
```

### 7.3 Kernel Baseline (Minimal Coverage)

| Kernel | Status |
|--------|--------|
| Dot product | REQUIRED |
| Vector add | REQUIRED |
| Vector multiply | REQUIRED |
| Matrix-vector | REQUIRED |
| Matrix-matrix | REQUIRED |
| RMSNorm | REQUIRED |
| RoPE | REQUIRED |
| Softmax | REQUIRED |
| Activation (GELU, etc.) | REQUIRED |
| Quantized dequantization | REQUIRED |

### 7.4 Correctness Contract

```
SIMD(input) ≈ Scalar(input)

Within dtype tolerance:
  FP32/FP16: 1e-5
  BF16:      1e-2
  INT8:      1e-3
  INT4:      1e-2
```

**Process:**
```
Every SIMD kernel implementation:
  1. Code complete
  2. Scalar reference written
  3. Property test: SIMD ≈ Scalar
  4. Only then used in production path
```

---

## 8. CACHE HIERARCHY

### 8.1 Three-Level Cache

| Level | Purpose | Storage | TTL | Priority |
|-------|---------|---------|-----|----------|
| **L0 (Hot KV)** | Active context | RAM | Session | P0 |
| **L1 (Persistent KV)** | Prefix reuse | Disk (columnar) | Persistent | P0 |
| **L2 (Response)** | Exact result | Disk/RAM | Configurable | P0 |

**Phase Baseline:**

```
Phase 1 → Exact + KV caches (both operational)
Phase 2 → Batch + speculative decoding
Phase 3 → Semantic cache
```

### 8.2 Cache Lookup Semantics

```
Request
    │
    ▼
Exact Cache (L2)
    │
    ├─ HIT → Return cached response (no computation)
    │
    └─ MISS → Continue to KV
              │
              ▼
         KV Cache (L0/L1)
              │
              ├─ HIT → Reuse prefix KV
              │        Compute new suffix
              │
              └─ MISS → Full prefill
                        Compute prefix
                        Compute suffix
```

### 8.3 Cache Invalidation

Cache entry INVALID if any of:
- model_hash changed
- model_revision changed
- tokenizer_revision changed
- architecture changed
- RoPE config changed
- dtype changed
- quantization changed

**Principle:** Silent reuse of incompatible KV = BUG.

---

## 9. KV CACHE SPECIFICATION

### 9.1 Block-Based Storage

```
KV divided into blocks.

KV_BLOCK_TOKENS = 64 (baseline)

Example:
  Block 0: tokens 0–63
  Block 1: tokens 64–127
  Block 2: tokens 128–191
  Block 3: tokens 192–255
  ...

Request context 0–220:
  Required blocks: 0, 1, 2, 3 (partial)
```

**Benefit:** Partial block reuse without loading entire cache.

### 9.2 Metadata Schema

**Minimal Required Metadata per KV block:**

```
record_id
model_id
model_revision
tokenizer_revision
layer_id
head_id
position_start
position_end
dtype
quantization
block_id
offset (in file)
length (bytes)
checksum (CRC32 or SHA256)
timestamp
```

### 9.3 Compatibility Check

```rust
fn is_kv_compatible(
    kv_record: &KVMetadata,
    current_model: &Model,
    current_tokenizer: &Tokenizer,
) -> bool {
    kv_record.model_hash == current_model.hash() &&
    kv_record.model_revision == current_model.revision() &&
    kv_record.tokenizer_revision == current_tokenizer.revision() &&
    kv_record.architecture == current_model.architecture() &&
    kv_record.layer_count == current_model.layer_count() &&
    kv_record.head_count == current_model.head_count() &&
    kv_record.head_dimension == current_model.head_dimension() &&
    kv_record.position_encoding == current_model.rope_config() &&
    kv_record.dtype == current_dtype &&
    kv_record.quantization == current_quantization
}
```

---

## 10. COLUMNAR STORAGE

### 10.1 Persistent Storage Layout

```
cache/
├── manifest                 # Version, schema, integrity
├── index/                   # Record → block lookup
├── metadata/                # Per-record metadata
├── keys/                    # K tensors (columnar blocks)
├── values/                  # V tensors (columnar blocks)
└── responses/               # Serialized response cache
```

### 10.2 Write Protocol (Crash-Safe)

```
Process:
  1. Generate block in memory
  2. Compute checksum
  3. Write to disk
  4. fsync() (or platform equivalent)
  5. Write commit marker
  6. Update index entry
  7. COMMITTED

If crash between step 4–6:
  Restart detects incomplete block → discard → rebuild on demand
```

### 10.3 Startup Recovery

```
Startup:
  1. Scan storage
  2. Validate checksum of each block
  3. Detect incomplete blocks (no commit marker)
  4. Remove incomplete blocks
  5. Rebuild index from valid blocks
  6. READY
```

### 10.4 Selective Read

**Key Optimization:**

If requesting only:
- layer = 4
- head = 2
- block = 10

Storage must NOT read entire dataset.

**Instrumentation:**

```
blocks_requested: N
blocks_read: N (not total blocks)
bytes_read: M (not entire storage)
```

---

## 11. LOCAL LLM RUNTIME

### 11.1 Supported Architecture (Baseline)

- **Type:** Decoder-only Transformer
- **Attention:** Multi-head or grouped-query
- **Normalization:** RMSNorm
- **Position Encoding:** RoPE
- **FFN:** SwiGLU/GLU-style

### 11.2 Inference Pipeline

```
Prompt
  ↓
Tokenizer → Token IDs
  ↓
Embedding Layer
  ↓
Transformer Layers (for each layer):
  ├─ Self-Attention
  │  ├─ Q, K, V projection
  │  ├─ RoPE application
  │  ├─ KV cache update
  │  └─ Attention computation
  ├─ Residual connection
  ├─ RMSNorm
  ├─ FFN (SwiGLU)
  └─ Residual connection
  ↓
LM Head
  ↓
Logits
  ↓
Sampler
  ↓
Next Token
  ↓
KV Update
  ↓
Repeat (decode loop)
  ↓
EOS or max_tokens
  ↓
RESPONSE
```

### 11.3 Prefill vs Decode Separation

**Prefill Phase:**
```
Input tokens: [t0, t1, t2, ..., tN]
         ↓
Bulk computation (all positions in parallel)
         ↓
KV generation for all positions
         ↓
Initial logits
```

**Decode Phase:**
```
Current state + one new token
         ↓
KV lookup (reuse previous KV)
         ↓
Incremental attention (only new token position)
         ↓
Logits
         ↓
Sample next token
         ↓
Repeat
```

**Why Separate:**
- Prefill: compute-bound (matrix-matrix multiplications)
- Decode: memory-bound (matrix-vector multiplications)
- Different optimization strategies

### 11.4 Attention Implementation

```
For each head:
  Q = tokens @ W_q
  K = tokens @ W_k
  V = tokens @ W_v
  
  Scores = (Q @ K^T) / sqrt(d_head)
  Probs = softmax(Scores)
  Output = Probs @ V
```

With RoPE applied to Q, K before projection.

---

## 12. COGNITIVE ORCHESTRATOR

### 12.1 Responsibilities

1. **Request Analysis**
   - Tokenize request
   - Extract context
   - Identify prefix (for KV reuse)

2. **State Inspection**
   - Check exact cache
   - Check KV availability
   - Assess memory

3. **Plan Generation**
   - Select execution provider (always Local as fallback)
   - Choose SIMD backend
   - Allocate reasoning budget
   - Determine scheduler mode

4. **Execution Delegation**
   - Route to local model
   - Collect results
   - Verify output
   - Trigger persistence

### 12.2 ExecutionPlan Structure

```rust
pub struct ExecutionPlan {
    pub cache: CacheStrategy,
    pub kv_blocks: Vec<u32>,
    pub provider: ExecutionProvider,
    pub simd: SIMDBackend,
    pub max_tokens: u32,
    pub reasoning_budget: ReasoningLevel,
    pub scheduler_mode: SchedulerMode,
    pub is_deterministic: bool,
}
```

---

## 13. REASONING ENGINE

### 13.1 Adaptive Compute Budget

```
Request
  ↓
Task Classification
  ├─ Simple (factual, direct)
  │  └─ Direct generation
  │     Budget: minimal
  │
  ├─ Medium (reasoning, explanation)
  │  └─ Structured reasoning
  │     Budget: moderate
  │     Steps: outline → generate
  │
  └─ Complex (multi-step, verification)
     └─ Deep reasoning
        Budget: high
        Steps: reasoning → verification → regenerate if needed
```

### 13.2 Verification Pattern

```
Generate candidate response
         ↓
Constraint check
  ├─ Passes → Return
  └─ Fails  → Regenerate (max 3 attempts)
```

**Purpose:** Maintain quality without unlimited compute.

### 13.3 Reasoning Budget NOT Reasoning Token Limit

```
Wrong approach:
  "Always generate 10K reasoning tokens"
  → wastes compute on simple tasks

Right approach:
  "Allocate compute to achieve quality"
  → simple tasks: minimal compute
  → complex tasks: more compute
```

---

## 14. SCHEDULER & EXECUTION

### 14.1 Scheduler Modes

| Mode | Optimization | Batching | Context |
|------|--------------|----------|---------|
| **Latency** | TTFT | No (single request priority) | Interactive |
| **Throughput** | Sustained tok/s | Yes (batch) | Batch inference |

### 14.2 Request Queuing

```
Request
  ↓
Classify (latency/throughput mode)
  ↓
Cache lookup
  ↓
KV lookup
  ↓
Add to inference queue
  ↓
Scheduler assigns compute
  ↓
Execute
  ↓
Return
```

### 14.3 Continuous Batching (Future)

```
Request A ──┐
Request B ──┼─→ Scheduler ─→ Batch ─→ SIMD
Request C ──┤
Request D ──┘
```

**Constraint:** Each request maintains independent KV.

---

## 15. EXTERNAL PROVIDER INTEGRATION

### 15.1 Optional Integration

```rust
pub enum ExecutionProvider {
    Local,      // Always available
    External,   // Conditional
}
```

**Lifecycle:**

```
Is external API available?
  ├─ YES, key present, network OK
  │  └─ Orchestrator may choose External
  │
  └─ NO (any reason)
     └─ LocalProvider active
        No alternative, no fallback needed
```

### 15.2 Invariant: API Not Required

```
API_KEY = absent
NETWORK = down
CACHE = empty
         ↓
LOCAL INFERENCE
         ↓
SUCCESS ✓

Any other outcome = architectural failure
```

### 15.3 Provider Interface

```rust
pub trait InferenceProvider {
    fn infer(&self, request: &InferenceRequest) 
        -> Result<InferenceResponse>;
}

pub struct LocalProvider {
    model: Model,
    runtime: TransformerRuntime,
}

pub struct ExternalProvider {
    api_key: String,
    endpoint: String,
}
```

---

## 16. CLI & DEPLOYMENT

### 16.1 CLI Commands

```bash
# Chat mode
llm-engine chat

# Generate with options
llm-engine generate \
  --model model_path \
  --prompt "..." \
  --max-tokens 256 \
  --temperature 0.8 \
  --seed 42

# Server mode
llm-engine serve [--port 8080]

# Model information
llm-engine model info [--model path]

# Cache diagnostics
llm-engine cache stats
llm-engine cache verify
llm-engine cache compact

# Benchmarking
llm-engine benchmark all
llm-engine benchmark inference
llm-engine benchmark cache

# Offline mode
llm-engine --offline generate ...
```

### 16.2 Single Binary Output

```
Build:
  cargo build --release

Output:
  target/release/llm-engine

Deployment:
  ./llm-engine
  
  Models and cache are external files, loaded at runtime.
```

### 16.3 Environment Variables

```bash
# Optional
LLM_MODEL_PATH          # Default: ./models
LLM_CACHE_PATH          # Default: ./cache
LLM_API_KEY             # Optional; no API = OK
LLM_EXTERNAL_PROVIDER   # Optional URL
LLM_OFFLINE             # Force offline: 1/0
LLM_LOG_LEVEL           # debug/info/warn/error
LLM_THREADS             # Thread pool size
LLM_MEMORY_MB           # Cache memory limit
```

---

## 17. TESTING & CI SPECIFICATION

### 17.1 Test Scope

CI validates 6 layers:

```
STATIC & BUILD
      ↓
UNIT TESTS
      ↓
INTEGRATION TESTS
      ↓
CACHE/KV TESTS
      ↓
OFFLINE E2E TESTS
      ↓
PERFORMANCE GATE
```

### 17.2 Test Model

**Use tiny deterministic model for CI:**

```
tiny-model/
├── manifest
├── config
├── tokenizer
└── weights

Properties:
  - Small vocab
  - Small hidden dimension
  - Few layers
  - Deterministic output
  - CPU runnable
  - Fast test cycles
```

### 17.3 Mandatory Tests

| Test | Purpose | Gate |
|------|---------|------|
| **T01** Build | Compilation | Hard |
| **T02** Single Binary | No external services | Hard |
| **T03** Offline Startup | Network unavailable | Hard |
| **T04** Local Inference | No API key | Hard |
| **T05** Cache Disabled | No cache crash | Hard |
| **T06** Cache Miss | Cold path works | Hard |
| **T07** Cache Hit | Exact cache works | Hard |
| **T08** Cache Semantics | output A == output B | Hard |
| **T09** KV Miss | Recompute path works | Hard |
| **T10** KV Hit | Prefix reuse works | Hard |
| **T11** KV Equivalence | logits ≈ identical | Hard |
| **T12** KV Invalidation | Incompatible KV rejected | Hard |
| **T13** Persistent KV | Survives restart | Hard |
| **T14** Crash Recovery | Incomplete blocks handled | Hard |
| **T15** SIMD Correctness | SIMD ≈ Scalar | Hard |
| **T16** SIMD Dispatch | Correct kernel selected | Hard |
| **T17** SIMD Fallback | Scalar available | Hard |
| **T18** Deterministic | Fixed seed reproducible | Hard |
| **T19** Context Growth | No memory corruption | Hard |
| **T20** API Disabled | No API key = OK | Hard |
| **T21** Network Down | No network = OK | Hard |
| **T22** Concurrent Requests | No data race/deadlock | Hard |
| **T23** Full E2E Cold | Empty cache → inference | Hard |
| **T24** Full E2E Warm | Restart → cache reuse | Hard |

### 17.4 CI Matrix

```
Platform    cargo check  cargo test  Clippy  Offline E2E  SIMD  Release
────────────────────────────────────────────────────────────────────
Linux x86   ✓            ✓           ✓       ✓            ✓     ✓ (gate)
Linux ARM   ✓            ✓           ✓       ✓            ✓     -
macOS ARM   ✓            ✓           ✓       opt          ✓     -
Windows     ✓            ✓           ✓       opt          opt   -
```

**Release Gate:** Linux x86-64 only.

### 17.5 Test Contract

```
All tests must run:

cargo test                  # Unit + integration
cargo test --test e2e      # E2E
cargo test --offline       # Offline only

No test may require:
  - API key
  - Network
  - External service
  - Redis/database
  - Cloud inference
```

---

## 18. BENCHMARK SPECIFICATION

### 18.1 Benchmark Purpose

Answer these questions:

1. How fast is cold inference without cache?
2. How much speedup from exact cache hit?
3. How much compute saved by KV prefix reuse?
4. How much SIMD accelerates local inference?
5. Does all optimization preserve correctness?

### 18.2 Benchmark Matrix

| Scenario | Cache | KV | SIMD | API | Purpose |
|----------|-------|-----|------|-----|---------|
| E2E-COLD | OFF | OFF | ON | OFF | Baseline inference |
| E2E-SCALAR | OFF | OFF | OFF | OFF | SIMD reference |
| E2E-SIMD | OFF | OFF | ON | OFF | SIMD speedup |
| E2E-CACHE-HIT | HIT | N/A | OFF | OFF | Cache latency |
| E2E-KV-HIT | MISS | HIT | ON | OFF | Prefix reuse |
| E2E-KV-MISS | MISS | MISS | ON | OFF | Recomputation |
| E2E-OFFLINE | VAR | VAR | ON | OFF | Autonomous mode |
| E2E-CONCURRENT | VAR | VAR | ON | OFF | Concurrency |

### 18.3 Mandatory Metrics

**Latency:**
- Startup latency
- Model load latency
- TTFT (Time To First Token)
- Prefill latency
- Decode latency per token
- p50, p95, p99 latencies

**Throughput:**
- Prefill tokens/sec
- Decode tokens/sec

**Efficiency:**
- KV Reuse Ratio = reused_tokens / total_prefix_tokens
- Compute Avoidance = 1 - (recomputed_tokens / baseline_tokens)
- Read Amplification = bytes_read / bytes_requested

**Memory:**
- Model RSS
- KV RSS
- Cache RSS
- Peak RSS
- Bytes per token

**Correctness:**
- Offline success rate = 100%
- Deterministic consistency = 100%
- SIMD ≈ Scalar = within tolerance

### 18.4 Hardware Metadata (Required)

Every benchmark must record:

```json
{
  "cpu_model": "...",
  "cpu_cores": 8,
  "cpu_threads": 16,
  "simd_capability": "AVX2",
  "ram_gb": 32,
  "os": "Linux",
  "rust_version": "...",
  "model": "tiny-model",
  "dtype": "BF16",
  "timestamp": "...",
  "git_commit": "..."
}
```

**Key:** Never compare absolute numbers across different hardware without normalization.

### 18.5 Performance Relationships (More Important Than Absolute Numbers)

```
Exact Cache Hit < KV Hit < KV Miss < Cold Inference
              ↓           ↓
          Response only   Compute prefill
          (no compute)    + reuse
```

**SIMD Speedup:**
```
SIMD Speedup = SIMD throughput / Scalar throughput

Example: 1.8x (30 tok/s SIMD vs 16 tok/s Scalar)
```

### 18.6 Benchmark Tiers

| Tier | Model | Dataset | Iterations | Purpose |
|------|-------|---------|-----------|---------|
| **T0 (CI)** | tiny | 1–100 | 1 | Regression |
| **T1 (Dev)** | small/medium | various | 5 | Profiling |
| **T2 (Release)** | representative | fixed | 30+ | Release gate |
| **T3 (Stress)** | large | varied | 100+ | Production |

### 18.7 Regression Gate

```
Critical regression (>15%)  → FAIL release
Warning (5–15%)             → Review
Normal (<5%)                → PASS

Correctness regression      → FAIL immediately
```

---

## 19. PRODUCT MATRIX

### 19.1 Component Status

| ID | Component | Capability | Status |
|----|-----------|-----------|--------|
| **P01** | Single Binary | Monolithic executable | READY |
| **P02** | Local LLM | Decoder-only Transformer | READY |
| **P03** | Offline Inference | No API/network required | READY |
| **P04** | Reasoning | Adaptive compute budget | READY |
| **P05** | Exact Cache | Response deduplication | READY |
| **P06** | KV Cache | Block-based persistent | READY |
| **P07** | Columnar Storage | Hybrid schema | READY |
| **P08** | SIMD | Runtime dispatch | READY |
| **P09** | Quantization | FP16/BF16→INT8→INT4 | READY |
| **P10** | Memory Manager | Pool + mmap | READY |
| **P11** | Prefill | Optimized path | READY |
| **P12** | Decode | Incremental KV | READY |
| **P13** | Scheduler | Latency/throughput | READY |
| **P14** | Concurrency | Controlled parallelism | READY |
| **P15** | Persistence | Crash-safe | READY |
| **P16** | External API | Optional provider | READY |
| **P17** | API-less Fallback | Always local available | **CRITICAL** |
| **P18** | E2E Test | Cold→warm→KV reuse | READY |
| **P19** | Benchmark | Reproducible suite | READY |
| **P20** | Observability | Metrics + tracing | READY |

### 19.2 Operating Modes

| Mode | API | Cache | KV | Local Model | Status |
|------|-----|-------|-----|-------------|--------|
| **Offline** | ❌ | ✓ | ✓ | ✓ | Primary |
| **Local** | ❌ | ✓ | ✓ | ✓ | Primary |
| **Hybrid** | ✓ opt | ✓ | ✓ | ✓ | Optional |

### 19.3 KPI Targets

**Correctness:**
```
Inference correctness       = 100%
KV equivalence              = 100%
Cache semantic correctness  = 100%
SIMD correctness            = 100%
Offline execution           = 100%
```

**Reliability:**
```
Crash recovery              = PASS
Corrupt cache detection     = PASS
Restart persistence         = PASS
API-independent inference   = PASS
```

**Performance (Relative):**
```
Exact Cache Hit < KV Hit < KV Miss < Cold
SIMD throughput > Scalar throughput
KV reuse ratio > 0 (for prefix-shared requests)
Compute avoidance > 0 (where applicable)
```

---

## 20. OPERATIONAL WORKFLOWS

### 20.1 Startup Workflow

```
Binary execution
      ↓
Initialize runtime
      ↓
Detect CPU/SIMD
      ↓
Initialize memory manager
      ↓
Load configuration
      ↓
Open cache storage
      ↓
Validate manifest
      ↓
Load model
      ↓
Load tokenizer
      ↓
Initialize scheduler
      ↓
Initialize orchestrator
      ↓
READY (accept requests)
```

### 20.2 Request Lifecycle

```
REQUEST
       ↓
VALIDATE
       ↓
NORMALIZE
       ↓
HASH (RequestSignature)
       ↓
EXACT CACHE LOOKUP
   ├─ HIT → RETURN (no computation)
   └─ MISS → KV LOOKUP
             ├─ HIT → REUSE PREFIX
             └─ MISS → LOCAL PREFILL
                       ↓
                       KV GENERATION
       ↓
INCREMENTAL DECODE
       ↓
REASONING ENGINE
       ↓
VERIFY (optional)
       ↓
KV WRITEBACK
       ↓
RESPONSE CACHE WRITE
       ↓
OUTPUT
```

### 20.3 Cold-to-Warm Lifecycle

**Request #1 (Cold):**
```
Cache: EMPTY
KV: EMPTY
     ↓
Local inference
     ↓
Full computation
     ↓
KV persist
     ↓
Response persist
     ↓
OUTPUT
```

**Request #2 (Exact Repeat):**
```
Cache: HIT
     ↓
Return cached response
     ↓
No computation
     ↓
OUTPUT
```

**Request #3 (Same Prefix):**
```
Cache: MISS
KV: PARTIAL HIT (prefix)
     ↓
Reuse prefix KV
     ↓
Compute suffix
     ↓
KV writeback
     ↓
OUTPUT
```

### 20.4 Restart with Persistence

**Process #1:**
```
Startup
  ↓
Cache miss
  ↓
Compute
  ↓
Persist KV + response
  ↓
Exit
```

**Process #2:**
```
Startup
  ↓
Load persistent KV
  ↓
KV hit (if same prefix)
  ↓
Reuse
  ↓
OUTPUT
```

### 20.5 Error Recovery

| Error | Recovery |
|-------|----------|
| Cache checksum fail | Discard record, recompute |
| KV checksum fail | Discard, recompute prefix |
| API unavailable | Use local inference |
| SIMD unavailable | Fallback to scalar |
| Storage write fail | Retry, then fail gracefully |
| Model unavailable | Fatal (no local execution possible) |
| Memory exhausted | Evict cache, continue with smaller working set |

### 20.6 Concurrent Request Handling

```
Request A ──┐
Request B ──┼──→ Request Queue
Request C ──┤
Request D ──┘
              │
              ▼
          Scheduler
              │
         ┌────┴────┐
         ▼         ▼
     Prefill     Decode
     Pool        Pool
         │         │
         └────┬────┘
              ▼
          Output

Invariants:
  - No KV collision
  - No data race
  - No deadlock
  - Controlled parallelism (no CPU oversubscription)
```

---

## 21. QUALITY GATES & EXIT CRITERIA

### 21.1 Build Gate

```bash
cargo build --release
```

**Acceptance:**
- Exit code = 0
- Binary exists: `target/release/llm-engine`
- No warnings (ideal; at minimum, no errors)

---

### 21.2 Unit Test Gate

```bash
cargo test --lib
```

**Acceptance:**
- All unit tests PASS
- Tensor operations correct
- Tokenizer deterministic
- Sampling algorithms correct
- Cache logic correct
- KV correctness verified

---

### 21.3 Integration Test Gate

```bash
cargo test --test '*'
```

**Acceptance:**
- Model loads
- Inference executes
- Cache pipeline works
- KV pipeline works
- Persistence works

---

### 21.4 Single Binary Test

```bash
./target/release/llm-engine --version
./target/release/llm-engine --help
./target/release/llm-engine model info
```

**Acceptance:**
- Binary starts
- Binary exits normally
- No runtime service required
- All subcommands available in one executable

---

### 21.5 Offline E2E Test

```bash
# Environment
LLM_API_KEY = unset
NETWORK = unavailable
LLM_OFFLINE = 1

./target/release/llm-engine generate \
  --model tiny-model \
  --prompt "Hello"
```

**Acceptance:**
- Process startup = success
- Model load = success
- Tokenization = success
- Inference = success
- Response generated = success
- Exit code = 0

---

### 21.6 No API Key Test

```bash
# Ensure no API key
unset LLM_API_KEY

./target/release/llm-engine generate --offline \
  --model tiny-model \
  --prompt "test"
```

**Acceptance:**
- LocalProvider active
- ExternalProvider disabled
- Inference success = YES
- Exit code = 0

**Failure Condition:**
- Missing API key → application failure = BUG

---

### 21.7 Cache Correctness Test

**Test A: Cache Disabled**
```
Run with cache OFF
Response: R1
```

**Test B: Cache Enabled**
```
Run with cache ON
Response: R2
```

**Acceptance:**
```
R1 == R2 (deterministic generation)
```

**Purpose:** Cache is optimization, not semantic modifier.

---

### 21.8 KV Equivalence Test

**Path A: Full Recomputation**
```
Prefix + suffix
All tokens recomputed
Logits: L_A
```

**Path B: KV Reuse**
```
Prefix (reused from KV)
Suffix (computed)
Logits: L_B
```

**Acceptance:**
```
L_A ≈ L_B (within dtype tolerance)
tokens_A == tokens_B (deterministic sampling)
```

---

### 21.9 KV Invalidation Test

**Scenario:**
```
Generate KV with model_v1
Attempt to reuse with model_v2
```

**Acceptance:**
```
KV reuse = REJECTED
Reason: model_hash mismatch
Fallback: recompute
```

**Failure Condition:**
Silent KV reuse across incompatible models = BUG

---

### 21.10 Persistent KV Test

**Process #1:**
```
Inference → KV generated → Persist → Exit
```

**Process #2:**
```
Startup → KV loaded → Same request → Reuse
```

**Acceptance:**
```
KV survives restart
Reuse successful
Response equivalent
```

---

### 21.11 Crash Recovery Test

**Simulate incomplete write:**
```
Process generating block
Crash before commit marker
Restart system
```

**Acceptance:**
```
Incomplete block detected
Block marked invalid
Discarded or recovered
Cache remains usable
No data corruption
```

---

### 21.12 SIMD Correctness Test

For each SIMD kernel:
```
result_scalar = kernel_scalar(input)
result_simd = kernel_simd(input)

Tolerance by dtype:
  FP32/FP16: 1e-5
  BF16: 1e-2
  INT8: 1e-3
  INT4: 1e-2
```

**Acceptance:**
```
result_simd ≈ result_scalar (within tolerance)
```

---

### 21.13 Deterministic Generation Test

```bash
# Configuration
temperature = 0
seed = 42

# Run 10 times
llm-engine generate --seed 42 --temperature 0 --prompt "test"
```

**Acceptance:**
```
output_1 == output_2 == ... == output_10
```

---

### 21.14 Full E2E Cold Start

```
EMPTY CACHE
NO API KEY
NO NETWORK
FRESH PROCESS
```

**Pipeline:**
```
Start binary
  ↓
Load model
  ↓
Load tokenizer
  ↓
Cache miss
  ↓
KV miss
  ↓
Local inference
  ↓
SIMD kernels
  ↓
Generate tokens
  ↓
Persist KV
  ↓
Persist response
  ↓
Return response
```

**Acceptance:**
```
exit code = 0
response != empty
cache created = YES
KV created = YES
```

---

### 21.15 Full E2E Warm Start

**Process #2 (after #1 completed):**
```
Start binary
  ↓
Load persistent cache
  ↓
Exact/KV lookup
  ↓
Reuse or compute
  ↓
Return response
```

**Acceptance:**
```
cold_run succeeds = YES
warm_run succeeds = YES
warm_run faster = YES
```

---

### 21.16 Benchmark Gate

Run full benchmark suite:
```bash
cargo run --release --bin benchmark -- all
```

**Acceptance Criteria:**

| Metric | Gate |
|--------|------|
| Compile time | < 5 min |
| Total E2E latency | Measure baseline |
| Cold TTFT | Measure baseline |
| Warm cache latency | < cold latency |
| KV reuse ratio | > 0 for prefix-shared |
| Compute avoidance | > 0 where applicable |
| SIMD speedup | > 1.0x |
| Offline success | 100% |
| Deterministic | 100% |
| Crash recovery | PASS |

---

## 22. FINAL EXIT CRITERIA

### 22.1 Baseline Release Checklist

```
COMPILE
  [✓] cargo build --release
  [✓] No errors
  [✓] Single binary produced

LINT
  [✓] cargo fmt --check
  [✓] cargo clippy -- -D warnings

UNIT TESTS
  [✓] All pass
  [✓] No flakes

INTEGRATION TESTS
  [✓] Model loading
  [✓] Inference
  [✓] Cache pipeline
  [✓] KV pipeline
  [✓] Persistence

SIMD TESTS
  [✓] Scalar kernel tests
  [✓] SIMD kernel tests
  [✓] Dispatch tests
  [✓] Fallback tests
  [✓] Correctness: SIMD ≈ Scalar

CACHE TESTS
  [✓] Exact cache hit
  [✓] Exact cache miss
  [✓] Cache semantic correctness
  [✓] Cache corruption detection

KV TESTS
  [✓] KV hit
  [✓] KV miss
  [✓] KV equivalence
  [✓] KV invalidation
  [✓] KV persistence

OFFLINE E2E
  [✓] No API key
  [✓] No network
  [✓] Inference succeeds

PERSISTENCE E2E
  [✓] Write KV
  [✓] Restart
  [✓] Read KV
  [✓] Reuse successful

SINGLE BINARY E2E
  [✓] Binary starts
  [✓] No external services needed
  [✓] All features in one exe

BENCHMARK
  [✓] Cold inference: measure
  [✓] Warm cache: measure
  [✓] KV reuse: measure
  [✓] SIMD speedup: measure
  [✓] No regression > threshold
```

### 22.2 Architectural Invariants Verification

```
INVARIANT 1: Local Model Always Available
  [✓] API absent → inference succeeds
  [✓] Network down → inference succeeds
  [✓] Cache empty → inference succeeds

INVARIANT 2: Cache Does Not Change Semantics
  [✓] output(cache=OFF) == output(cache=ON)

INVARIANT 3: KV Cache Correctness
  [✓] KV reuse path == recomputation path (logits ≈)
  [✓] Deterministic: tokens identical

INVARIANT 4: Single Binary Completeness
  [✓] All features in one executable
  [✓] No external service required

INVARIANT 5: Offline Capability
  [✓] --offline flag disables network
  [✓] All features remain functional

INVARIANT 6: Crash-Safe Storage
  [✓] Incomplete writes detected
  [✓] Recovery on startup
  [✓] Valid data preserved
```

---

### 22.3 Definition of Done

The baseline is **COMPLETE** and **RELEASE-READY** only when:

```
A single Rust binary can be started on a machine 
without API key and without network connectivity, 
can load a local model, perform inference and reasoning, 
generate tokens, build KV cache, persist KV to 
persistent columnar storage, reload KV after restart, 
reuse prefix on subsequent inference, and generate 
output equivalent to full recomputation path.

All paths and invariants must be proven via 
automated CI/E2E tests.
```

---

### 22.4 Most Critical Test

The single most important test in the entire project:

```
┌─────────────────────────────────────┐
│   FINAL BASELINE E2E TEST           │
├─────────────────────────────────────┤
│                                     │
│  API KEY       = NONE               │
│  NETWORK       = OFF                │
│  CACHE         = EMPTY              │
│  MODEL         = LOCAL              │
│                                     │
│       ↓                             │
│  Local Transformer                  │
│       ↓                             │
│  SIMD Inference                     │
│       ↓                             │
│  Reasoning / Generation             │
│       ↓                             │
│  KV Cache                           │
│       ↓                             │
│  Columnar Persistence               │
│                                     │
│       → SUCCESS ✓                   │
│                                     │
└─────────────────────────────────────┘

This test proves the project is truly autonomous local LLM,
not just a cache layer for an external API.
```

---

## 23. OPTIMIZATION SEQUENCE (PRIORITY ORDER)

```
1. VALID INFERENCE
   └─ Correct results, any speed

2. KV REUSE
   └─ Recomputation avoidance

3. MEMORY LOCALITY
   └─ Cache-conscious access patterns

4. SIMD
   └─ Accelerate required computation

5. QUANTIZATION
   └─ Reduce memory footprint

6. FUSION / TILING
   └─ Kernel-level optimization

7. BATCHING
   └─ Concurrent request efficiency

8. SPECULATIVE DECODING
   └─ Optional multi-token acceleration

9. ADAPTIVE REASONING
   └─ Quality-aware compute allocation
```

---

## 24. SUMMARY: WHAT THIS CONTRACT DEFINES

### 24.1 What IS Specified

✓ **Monolithic local LLM execution engine**
✓ **Offline-first architecture**
✓ **Cache as optimization, not requirement**
✓ **KV cache for prefix reuse**
✓ **Persistent columnar storage**
✓ **SIMD-accelerated kernels**
✓ **Adaptive reasoning with compute budgets**
✓ **Crash-safe persistence**
✓ **Complete E2E testing**
✓ **Reproducible benchmarking**

### 24.2 What Is NOT Specified

✗ Absolute performance numbers (hardware-dependent)
✗ Semantic cache (Phase 2+)
✗ Speculative decoding (Phase 2+)
✗ Multi-GPU/TPU distribution (future)
✗ Streaming response details
✗ Fine-tuning infrastructure
✗ Quantization training

### 24.3 Core Principle

```
Local model → Always primary execution engine
Cache/KV → Optimization layer (minimize compute)
SIMD → Speedup layer (maximize speed of required compute)
API → Optional provider (never dependency)
```

---

## 25. ENFORCEMENT

Any implementation claiming to fulfill this contract must:

1. **Pass all mandatory tests** (Section 17.3, T01–T24)
2. **Demonstrate all invariants** (Section 3)
3. **Support all operating modes** (Offline, Local, Hybrid)
4. **Provide E2E benchmark results** with metadata
5. **Maintain SIMD correctness** (Section 7.4)
6. **Ensure crash safety** (Section 10.2)
7. **Document deviations** if any from this contract

---

**This Architecture Contract is the source of truth for llm-engine baseline implementation and acceptance.**
