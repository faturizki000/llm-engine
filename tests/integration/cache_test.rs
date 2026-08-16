use llm_engine::cache::ExactResponseCache;

#[test]
fn cache_can_store_and_retrieve_exact_response() {
    let mut cache = ExactResponseCache::new();
    cache.insert("question".to_string(), "answer".to_string());
    assert_eq!(cache.get("question"), Some(&"answer".to_string()));
}
