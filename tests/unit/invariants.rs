use llm_engine::{
    cache::ExactResponseCache,
    kv::{KvBlock, KvStore},
    simd::{detect_simd, dot_product, rmsnorm, rope_apply, softmax},
    LocalRuntime, RuntimeConfig,
};

#[test]
fn local_inference_without_api_key_works() {
    let runtime = LocalRuntime::new(RuntimeConfig::default());
    let output = runtime.generate("hello offline", 4);
    assert!(output.starts_with("local::"));
    assert!(output.contains("hello offline"));
}

#[test]
fn cache_semantics_are_correct() {
    let mut cache = ExactResponseCache::new();
    cache.insert("hello".to_string(), "world".to_string());
    assert_eq!(cache.get("hello"), Some(&"world".to_string()));
    assert!(cache.contains("hello"));
    assert_eq!(cache.len(), 1);
}

#[test]
fn kv_equivalence_is_preserved_for_same_prompt() {
    let mut store_a = KvStore::new();
    let mut store_b = KvStore::new();
    store_a.insert(KvBlock::new(1, 4, 8, vec![1.0; 32]));
    store_b.insert(KvBlock::new(1, 4, 8, vec![1.0; 32]));
    assert_eq!(store_a.len(), store_b.len());
    assert!(store_a.equivalent_to(&store_b));
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
    let block = KvBlock::new(1, 0, 8, vec![]);
    assert!(!block.is_valid());
    assert!(!block.has_complete_payload());
}

#[test]
fn cache_serialization_roundtrip_is_stable() {
    let mut cache = ExactResponseCache::new();
    cache.insert("alpha".to_string(), "beta".to_string());
    let payload = cache.to_json();
    let decoded = ExactResponseCache::from_json(&payload).unwrap();
    assert_eq!(decoded.get("alpha"), Some(&"beta".to_string()));
}

#[test]
fn persistent_storage_recovers_valid_blocks() {
    let mut storage = llm_engine::columnar::Storage::new();
    storage.write_block(7, b"ok".to_vec());
    assert_eq!(storage.read_block(7), Some(&b"ok".to_vec()));
    assert!(storage.recover(7).is_some());
}
