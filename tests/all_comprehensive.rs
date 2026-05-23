//! Comprehensive tests for pizza-analysis-all (meta-crate registering all analysis plugins).

use pizza_analysis_all::{enabled_plugins, register_all};
use pizza_engine::analysis::AnalysisFactory;

// ═══════════════════════════════════════════════════════════════════════════════
// register_all — registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_does_not_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_twice_does_not_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    // Registering again should overwrite, not panic
    register_all(&mut factory);
}

// ═══════════════════════════════════════════════════════════════════════════════
// enabled_plugins — feature-flag introspection
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn enabled_plugins_not_empty() {
    let plugins = enabled_plugins();
    assert!(!plugins.is_empty(), "at least some plugins should be enabled with default features");
}

#[test]
fn enabled_plugins_contains_core() {
    let plugins = enabled_plugins();
    assert!(
        plugins.contains(&"pizza-analysis-core"),
        "core plugin should be enabled by default"
    );
}

#[test]
fn enabled_plugins_contains_stemmers() {
    let plugins = enabled_plugins();
    assert!(
        plugins.contains(&"pizza-analysis-stemmers"),
        "stemmers plugin should be enabled by default"
    );
}

#[test]
fn enabled_plugins_contains_ik() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-ik"));
}

#[test]
fn enabled_plugins_contains_jieba() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-jieba"));
}

#[test]
fn enabled_plugins_contains_pinyin() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-pinyin"));
}

#[test]
fn enabled_plugins_contains_smartcn() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-smartcn"));
}

#[test]
fn enabled_plugins_contains_stconvert() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-stconvert"));
}

#[test]
fn enabled_plugins_contains_english() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-english"));
}

#[test]
fn enabled_plugins_contains_french() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-french"));
}

#[test]
fn enabled_plugins_contains_german() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-german"));
}

#[test]
fn enabled_plugins_contains_morfologik() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-morfologik"));
}

#[test]
fn enabled_plugins_contains_stempel() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-stempel"));
}

#[test]
fn enabled_plugins_contains_synonym() {
    let plugins = enabled_plugins();
    assert!(plugins.contains(&"pizza-analysis-synonym"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Factory state after registration — tokenizers
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn factory_has_ik_tokenizers() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_tokenizer("ik_smart").is_some());
    assert!(factory.get_tokenizer("ik_max_word").is_some());
}

#[test]
fn factory_has_jieba_tokenizer() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_tokenizer("jieba").is_some());
}

#[test]
fn factory_has_smartcn_tokenizer() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_tokenizer("smartcn_tokenizer").is_some());
}

#[test]
fn factory_has_pinyin_tokenizer() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_tokenizer("pinyin").is_some());
}

#[test]
fn factory_has_stconvert_tokenizers() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_tokenizer("stconvert_s2t").is_some());
    assert!(factory.get_tokenizer("stconvert_t2s").is_some());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Factory state after registration — token filters
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn factory_has_stempel_filters() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("stempel_stem").is_some());
    assert!(factory.get_token_filter("polish_stop").is_some());
}

#[test]
fn factory_has_morfologik_filters() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("morfologik_stem").is_some());
    assert!(factory.get_token_filter("ukrainian_stem").is_some());
    assert!(factory.get_token_filter("ukrainian_stop").is_some());
}

#[test]
fn factory_has_synonym_filters() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("synonym").is_some());
    assert!(factory.get_token_filter("synonym_graph").is_some());
}

#[test]
fn factory_has_smartcn_stop() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("smartcn_stop").is_some());
}

#[test]
fn factory_has_stconvert_filters() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("stconvert_s2t").is_some());
    assert!(factory.get_token_filter("stconvert_t2s").is_some());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Factory state after registration — analyzers
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn factory_has_ik_analyzers() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("ik_smart").is_some());
    assert!(factory.get_analyzer("ik_max_word").is_some());
}

#[test]
fn factory_has_jieba_analyzer() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("jieba").is_some());
}

#[test]
fn factory_has_smartcn_analyzer() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("smartcn").is_some());
}

#[test]
fn factory_has_pinyin_analyzer() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("pinyin").is_some());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Factory state after registration — normalizers
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn factory_has_pinyin_normalizer() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_normalizer("pinyin").is_some());
    assert!(factory.get_normalizer("pinyin_first_letter").is_some());
}

#[test]
fn factory_has_stconvert_normalizers() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_normalizer("stconvert_s2t").is_some());
    assert!(factory.get_normalizer("stconvert_t2s").is_some());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Plugin count sanity
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn enabled_plugins_count_with_default_features() {
    let plugins = enabled_plugins();
    // With default features, we expect 33 plugins
    // (core, stemmers, icu, cjk, ik, jieba, kuromoji, nori, pinyin, smartcn,
    //  stconvert, english, french, german, spanish, italian, portuguese, dutch,
    //  russian, greek, norwegian, swedish, finnish, hungarian, turkish,
    //  arabic, persian, hindi, bengali, indonesian, brazilian,
    //  morfologik, stempel, synonym)
    assert!(
        plugins.len() >= 30,
        "expected at least 30 plugins with default features, got {}",
        plugins.len()
    );
}

#[test]
fn all_plugin_names_are_prefixed() {
    let plugins = enabled_plugins();
    for &name in plugins {
        assert!(
            name.starts_with("pizza-analysis-"),
            "plugin name '{}' should start with 'pizza-analysis-'",
            name
        );
    }
}
