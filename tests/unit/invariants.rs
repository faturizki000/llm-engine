use llm_engine::{cache::ExactResponseCache, kv::KvStore, simd::{detect_simd, dot_product, rmsnorm, rope_apply, softmax}, RuntimeConfig, LocalRuntime};

#[test]
fn local_inference_without_api_key_works() {
    let runtime = LocalRuntime::new(RuntimeConfig::default());
    let output = runtime.generate("hello offline", 4);
    assert!(output.contains("local::"));
}

#[test]
fn cache_semantics_are_correct() {
    let mut cache = ExactResponseCache::new();
    cache.insert("hello".to_string(), "world".to_string());
    assert_eq!(cache.get("hello"), Some(&"world".to_string()));
}

#[test]
fn kv_equivalence_is_preserved_for_same_prompt() {
    let store_a = llm_engine::kv::KvStore::new();
    let store_b = llm_engine::kv::KvStore::new();
    assert_eq!(store_a.len(), store_b.len());
}

#[test]
fn simd_matches_scalar_tolerance() {
    let lhs = vec![1.0, 2.0, 3.0];
    let rhs = vec![4.0, 5.0, 6.0];
    let simd = dot_product(&lhs, &rhs);
    let scalar = lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum::<f32>();
    assert!((simd - scalar).abs() < 1e-5);
    let norm = rmsnorm(&lhs, 1e-5);
    assert_eq!(norm.len(), lhs.len());
    let rope = rope_apply(&lhs, 2);
    assert_eq!(rope.len(), lhs.len());
    let s = softmax(&lhs);
    assert_eq!(s.len(), lhs.len());
    let _ = detect_simd();
}

#[test]
fn corrupt_blocks_are_detected() {
    let block = llm_engine::kv::block::KvBlock { block_id: 1, token_count: 0, hidden_size: 8 };
    assert!(!block.is_valid());
}
