//! Meta-crate that wires every discovered Pizza analysis plugin into a single
//! [`AnalysisFactory`]. Each plugin is gated behind a Cargo feature so consumers
//! can opt out individually via `default-features = false` + selective features.
//!
//! ## Plugins (33 total)
//!
//! - **Foundation**: `core`, `stemmers`, `icu`
//! - **CJK & Asian**: `cjk`, `ik`, `jieba`, `kuromoji`, `nori`, `pinyin`, `smartcn`, `stconvert`
//! - **European**: `english`, `french`, `german`, `spanish`, `italian`, `portuguese`,
//!   `dutch`, `russian`, `greek`, `norwegian`, `swedish`, `finnish`, `hungarian`, `turkish`
//! - **South/SE Asian & Other**: `arabic`, `persian`, `hindi`, `bengali`, `indonesian`, `brazilian`
//! - **Dictionary-based**: `morfologik`, `stempel`

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use pizza_engine::analysis::AnalysisFactory;

/// Register every enabled plugin into `factory`.
///
/// Call order: foundation first (core, stemmers, icu), then specialized plugins
/// (CJK, per-language, dictionary-based). Plugins registered later may override
/// analyzers/filters of the same name registered earlier — per-language crates
/// intentionally override the basic analyzers from `analysis-core` with richer
/// pipelines including language-specific stop words, normalization, and stemming.
pub fn register_all(factory: &mut AnalysisFactory) {
    // ── Foundation ──────────────────────────────────────────────────────
    #[cfg(feature = "core")]
    pizza_analysis_core::register_all(factory);
    #[cfg(feature = "stemmers")]
    pizza_analysis_stemmers::register_all(factory);
    #[cfg(feature = "icu")]
    pizza_analysis_icu::register_all(factory);

    // ── CJK & Asian Language Plugins ───────────────────────────────────
    #[cfg(feature = "cjk")]
    pizza_analysis_cjk::register_all(factory);
    #[cfg(feature = "ik")]
    pizza_analysis_ik::register_all(factory);
    #[cfg(feature = "jieba")]
    pizza_analysis_jieba::register_all(factory);
    #[cfg(feature = "kuromoji")]
    pizza_analysis_kuromoji::register_all(factory);
    #[cfg(feature = "nori")]
    pizza_analysis_nori::register_all(factory);
    #[cfg(feature = "pinyin")]
    pizza_analysis_pinyin::register_all(factory);
    #[cfg(feature = "smartcn")]
    pizza_analysis_smartcn::register_all(factory);
    #[cfg(feature = "stconvert")]
    pizza_analysis_stconvert::register_all(factory);

    // ── European Language Plugins ──────────────────────────────────────
    #[cfg(feature = "english")]
    pizza_analysis_english::register_all(factory);
    #[cfg(feature = "french")]
    pizza_analysis_french::register_all(factory);
    #[cfg(feature = "german")]
    pizza_analysis_german::register_all(factory);
    #[cfg(feature = "spanish")]
    pizza_analysis_spanish::register_all(factory);
    #[cfg(feature = "italian")]
    pizza_analysis_italian::register_all(factory);
    #[cfg(feature = "portuguese")]
    pizza_analysis_portuguese::register_all(factory);
    #[cfg(feature = "dutch")]
    pizza_analysis_dutch::register_all(factory);
    #[cfg(feature = "russian")]
    pizza_analysis_russian::register_all(factory);
    #[cfg(feature = "greek")]
    pizza_analysis_greek::register_all(factory);
    #[cfg(feature = "norwegian")]
    pizza_analysis_norwegian::register_all(factory);
    #[cfg(feature = "swedish")]
    pizza_analysis_swedish::register_all(factory);
    #[cfg(feature = "finnish")]
    pizza_analysis_finnish::register_all(factory);
    #[cfg(feature = "hungarian")]
    pizza_analysis_hungarian::register_all(factory);
    #[cfg(feature = "turkish")]
    pizza_analysis_turkish::register_all(factory);

    // ── South/Southeast Asian & Other Languages ────────────────────────
    #[cfg(feature = "arabic")]
    pizza_analysis_arabic::register_all(factory);
    #[cfg(feature = "persian")]
    pizza_analysis_persian::register_all(factory);
    #[cfg(feature = "hindi")]
    pizza_analysis_hindi::register_all(factory);
    #[cfg(feature = "bengali")]
    pizza_analysis_bengali::register_all(factory);
    #[cfg(feature = "indonesian")]
    pizza_analysis_indonesian::register_all(factory);
    #[cfg(feature = "brazilian")]
    pizza_analysis_brazilian::register_all(factory);

    // ── Dictionary-based Plugins ───────────────────────────────────────
    #[cfg(feature = "morfologik")]
    pizza_analysis_morfologik::register_all(factory);
    #[cfg(feature = "stempel")]
    pizza_analysis_stempel::register_all(factory);

    // ── Synonym & Specialized Plugins ──────────────────────────────────
    #[cfg(feature = "synonym")]
    pizza_analysis_synonym::register_all(factory);
    // ── Auto Language Detection ────────────────────────────────────────────
    // Must be registered LAST so it can capture all language analyzers above.
    #[cfg(feature = "auto")]
    pizza_analysis_auto::register_all(factory);
}

/// Names of all plugins compiled into this build.
pub fn enabled_plugins() -> &'static [&'static str] {
    &[
        #[cfg(feature = "core")]
        "pizza-analysis-core",
        #[cfg(feature = "stemmers")]
        "pizza-analysis-stemmers",
        #[cfg(feature = "icu")]
        "pizza-analysis-icu",
        #[cfg(feature = "cjk")]
        "pizza-analysis-cjk",
        #[cfg(feature = "ik")]
        "pizza-analysis-ik",
        #[cfg(feature = "jieba")]
        "pizza-analysis-jieba",
        #[cfg(feature = "kuromoji")]
        "pizza-analysis-kuromoji",
        #[cfg(feature = "nori")]
        "pizza-analysis-nori",
        #[cfg(feature = "pinyin")]
        "pizza-analysis-pinyin",
        #[cfg(feature = "smartcn")]
        "pizza-analysis-smartcn",
        #[cfg(feature = "stconvert")]
        "pizza-analysis-stconvert",
        #[cfg(feature = "english")]
        "pizza-analysis-english",
        #[cfg(feature = "french")]
        "pizza-analysis-french",
        #[cfg(feature = "german")]
        "pizza-analysis-german",
        #[cfg(feature = "spanish")]
        "pizza-analysis-spanish",
        #[cfg(feature = "italian")]
        "pizza-analysis-italian",
        #[cfg(feature = "portuguese")]
        "pizza-analysis-portuguese",
        #[cfg(feature = "dutch")]
        "pizza-analysis-dutch",
        #[cfg(feature = "russian")]
        "pizza-analysis-russian",
        #[cfg(feature = "greek")]
        "pizza-analysis-greek",
        #[cfg(feature = "norwegian")]
        "pizza-analysis-norwegian",
        #[cfg(feature = "swedish")]
        "pizza-analysis-swedish",
        #[cfg(feature = "finnish")]
        "pizza-analysis-finnish",
        #[cfg(feature = "hungarian")]
        "pizza-analysis-hungarian",
        #[cfg(feature = "turkish")]
        "pizza-analysis-turkish",
        #[cfg(feature = "arabic")]
        "pizza-analysis-arabic",
        #[cfg(feature = "persian")]
        "pizza-analysis-persian",
        #[cfg(feature = "hindi")]
        "pizza-analysis-hindi",
        #[cfg(feature = "bengali")]
        "pizza-analysis-bengali",
        #[cfg(feature = "indonesian")]
        "pizza-analysis-indonesian",
        #[cfg(feature = "brazilian")]
        "pizza-analysis-brazilian",
        #[cfg(feature = "morfologik")]
        "pizza-analysis-morfologik",
        #[cfg(feature = "stempel")]
        "pizza-analysis-stempel",
        #[cfg(feature = "synonym")]
        "pizza-analysis-synonym",
        #[cfg(feature = "auto")]
        "pizza-analysis-auto",
    ]
}
