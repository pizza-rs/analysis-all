<div align="center">

# INFINI Pizza Analysis

### The Complete Text Analysis Engine for INFINI Pizza

[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![Plugins](https://img.shields.io/badge/plugins-33-blue)]()
[![Languages](https://img.shields.io/badge/languages-70%2B-brightgreen)]()
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)]()

**26 tokenizers** · **130+ token filters** · **13 normalizers** · **70+ pre-built analyzers** · **21 per-language crates**

*From Arabic to Yiddish — production-grade multilingual text analysis, all in pure Rust.*

</div>

---

## Overview

`pizza-analysis-all` is the unified meta-crate that assembles the entire Pizza analysis pipeline. A single function call registers **33 specialized plugins** covering every major writing system — CJK segmentation, Japanese/Korean morphology, ICU Unicode processing, Snowball stemmers, Pinyin conversion, 21 dedicated per-language analysis crates, and more.

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_all::register_all(&mut factory);
// → 26 tokenizers, 130+ filters, 70+ analyzers ready to use
```

## Quick Start

```toml
[dependencies]
pizza-analysis-all = { path = "../contrib/analysis-all" }
```

### Selective Features

Enable only what you need — each plugin is a Cargo feature:

```toml
[dependencies]
pizza-analysis-all = {
    path = "../contrib/analysis-all",
    default-features = false,
    features = ["core", "jieba", "kuromoji"],
}
```

Feature names correspond to crate names with the `pizza-analysis-` prefix stripped.

---

## Plugin Ecosystem

### Foundation Plugins

| Feature | Crate | Highlights |
|:--------|:------|:-----------|
| **`core`** | [analysis-core](https://github.com/pizza-rs/analysis-core) | Foundation: tokenizers, filters, normalizers, 65+ language analyzers |
| **`stemmers`** | [analysis-stemmers](https://github.com/pizza-rs/analysis-stemmers) | Snowball algorithmic stemmers for 33 languages |
| **`icu`** | [analysis-icu](https://github.com/pizza-rs/analysis-icu) | ICU4X Unicode segmentation, normalization, folding, collation |

### CJK & Asian Language Plugins

| Feature | Crate | Highlights |
|:--------|:------|:-----------|
| **`cjk`** | [analysis-cjk](https://github.com/pizza-rs/analysis-cjk) | CJK bigram, width normalization, stop words (zh/ja/ko) |
| **`ik`** | [analysis-ik](https://github.com/pizza-rs/analysis-ik) | IK Chinese word segmentation (smart + max-word modes) |
| **`jieba`** | [analysis-jieba](https://github.com/pizza-rs/analysis-jieba) | Jieba Chinese segmentation with HMM new-word detection |
| **`kuromoji`** | [analysis-kuromoji](https://github.com/pizza-rs/analysis-kuromoji) | Japanese morphological analysis (IPADIC dictionary) |
| **`nori`** | [analysis-nori](https://github.com/pizza-rs/analysis-nori) | Korean morphological analysis (mecab-ko-dic) |
| **`pinyin`** | [analysis-pinyin](https://github.com/pizza-rs/analysis-pinyin) | Chinese → Pinyin romanization with polyphone support |
| **`smartcn`** | [analysis-smartcn](https://github.com/pizza-rs/analysis-smartcn) | SmartCN Chinese segmentation (Viterbi + DARTS trie) |
| **`stconvert`** | [analysis-stconvert](https://github.com/pizza-rs/analysis-stconvert) | Simplified ↔ Traditional Chinese conversion (CN/TW/HK) |

### Per-Language Analysis Crates (21)

Dedicated crates with language-specific normalization, stemming, and extended stop word corpora:

| Feature | Crate | Components |
|:--------|:------|:-----------|
| **`english`** | [analysis-english](https://github.com/pizza-rs/analysis-english) | KStem stemmer, possessive filter, 245 stop words |
| **`french`** | [analysis-french](https://github.com/pizza-rs/analysis-french) | Elision filter, light stemmer, 321 stop words |
| **`german`** | [analysis-german](https://github.com/pizza-rs/analysis-german) | ß/umlaut normalization, light stemmer, 391 stop words |
| **`spanish`** | [analysis-spanish](https://github.com/pizza-rs/analysis-spanish) | Light stemmer, 325 stop words |
| **`italian`** | [analysis-italian](https://github.com/pizza-rs/analysis-italian) | Elision filter, light stemmer, 328 stop words |
| **`portuguese`** | [analysis-portuguese](https://github.com/pizza-rs/analysis-portuguese) | Light stemmer, 359 stop words |
| **`dutch`** | [analysis-dutch](https://github.com/pizza-rs/analysis-dutch) | Suffix stemmer (plurals/diminutives), 222 stop words |
| **`russian`** | [analysis-russian](https://github.com/pizza-rs/analysis-russian) | ё→е normalization, light stemmer, 301 stop words |
| **`greek`** | [analysis-greek](https://github.com/pizza-rs/analysis-greek) | Accent/tonos removal, Ntais stemmer, stop words |
| **`norwegian`** | [analysis-norwegian](https://github.com/pizza-rs/analysis-norwegian) | Light stemmer (Bokmål + Nynorsk), stop words |
| **`swedish`** | [analysis-swedish](https://github.com/pizza-rs/analysis-swedish) | Snowball-style stemmer, stop words |
| **`finnish`** | [analysis-finnish](https://github.com/pizza-rs/analysis-finnish) | Agglutinative case-ending stripper, stop words |
| **`hungarian`** | [analysis-hungarian](https://github.com/pizza-rs/analysis-hungarian) | Case/plural suffix stemmer, stop words |
| **`turkish`** | [analysis-turkish](https://github.com/pizza-rs/analysis-turkish) | Locale-aware lowercase (dotted/dotless İ/I), 261 stop words |
| **`arabic`** | [analysis-arabic](https://github.com/pizza-rs/analysis-arabic) | ALEF/diacritics normalization, prefix/suffix stem, 249 stop words |
| **`persian`** | [analysis-persian](https://github.com/pizza-rs/analysis-persian) | Farsi char normalization, affix stemmer, stop words |
| **`hindi`** | [analysis-hindi](https://github.com/pizza-rs/analysis-hindi) | Devanagari normalization, suffix stemmer, stop words |
| **`bengali`** | [analysis-bengali](https://github.com/pizza-rs/analysis-bengali) | Script normalization, inflectional stemmer, stop words |
| **`indonesian`** | [analysis-indonesian](https://github.com/pizza-rs/analysis-indonesian) | AFNLP prefix/suffix stemmer, stop words |
| **`brazilian`** | [analysis-brazilian](https://github.com/pizza-rs/analysis-brazilian) | RSLP stemmer (plural/feminine/augmentative), stop words |

### Dictionary-Based Plugins

| Feature | Crate | Highlights |
|:--------|:------|:-----------|
| **`morfologik`** | [analysis-morfologik](https://github.com/pizza-rs/analysis-morfologik) | Polish & Ukrainian dictionary-based lemmatization |
| **`stempel`** | [analysis-stempel](https://github.com/pizza-rs/analysis-stempel) | Polish Stempel stemmer (Egothor multi-trie) |

---

## Component Reference

### Tokenizers (26)

<details>
<summary><strong>General Purpose</strong> — 16 tokenizers from <code>core</code></summary>

| Name | Description |
|:-----|:------------|
| `standard` | Grammar-based tokenizer (UAX#29 word boundaries) |
| `whitespace` | Splits on Unicode whitespace |
| `keyword` | Emits entire input as a single token |
| `letter` | Splits on non-letter characters |
| `lowercase` | Letter tokenizer + lowercasing |
| `classic` | Handles acronyms, emails, hostnames |
| `uax_url_email` | Preserves URLs and emails as single tokens |
| `pattern` | Splits on a configurable regex pattern |
| `simple_pattern` | Matches tokens using a regex |
| `simple_pattern_split` | Splits on regex matches, emits non-matches |
| `char_group` | Splits on configurable character groups |
| `path_hierarchy` | Generates filesystem path prefix tokens |
| `ngram` | Character n-gram tokenizer |
| `edge_ngram` | Edge (prefix) n-gram tokenizer |
| `thai` | Thai script segmentation |
| `burmese` | Burmese script segmentation |

</details>

<details>
<summary><strong>CJK & Asian Languages</strong> — 10 specialized tokenizers</summary>

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `icu_tokenizer` | icu | Unicode UAX#29 segmentation (all scripts) |
| `ik_smart` | ik | Chinese — smart mode (non-overlapping) |
| `ik_max_word` | ik | Chinese — max-word mode (all dictionary hits) |
| `jieba` | jieba | Chinese — Jieba search mode segmentation |
| `kuromoji_tokenizer` | kuromoji | Japanese morphological tokenizer (IPADIC) |
| `nori_tokenizer` | nori | Korean morphological tokenizer (mecab-ko-dic) |
| `pinyin` | pinyin | Chinese → Pinyin romanization |
| `smartcn_tokenizer` | smartcn | Chinese — Viterbi dynamic programming segmenter |
| `stconvert_s2t` | stconvert | Simplified → Traditional Chinese |
| `stconvert_t2s` | stconvert | Traditional → Simplified Chinese |

</details>

---

### Token Filters (130+)

<details>
<summary><strong>Text Transformation</strong> — General-purpose filters</summary>

| Name | Description |
|:-----|:------------|
| `lowercase` | Lowercase all tokens |
| `uppercase` | Uppercase all tokens |
| `trim` | Trim whitespace from tokens |
| `reverse` | Reverse token text |
| `asciifolding` | Fold Unicode to ASCII equivalents |
| `apostrophe` | Strip everything after apostrophe |
| `decimal_digit` | Normalize Unicode digits to 0-9 |
| `classic` | Remove trailing possessives, dots from acronyms |
| `keyword_repeat` | Emit each token twice (original + stemmed) |
| `unique` | Remove duplicate tokens |
| `remove_duplicates` | Remove exact duplicates at same position |
| `flatten_graph` | Flatten token graph for indexing |
| `hyphenated_words` | Rejoin hyphenated words across line breaks |
| `keep_types` | Keep/remove tokens by type |
| `protected_words` | Shield specific words from further filtering |
| `elision` | Remove elisions (l', d', qu', etc.) |
| `pattern_replace` | Regex-based token replacement |
| `fingerprint` | Generate a unique text fingerprint |
| `cjk_bigram` | Generate CJK character bigrams |
| `cjk_width` | Normalize CJK fullwidth ↔ halfwidth characters |

</details>

<details>
<summary><strong>Token Shaping</strong> — Length, n-gram, and boundary controls</summary>

| Name | Description |
|:-----|:------------|
| `length` | Remove tokens outside length bounds |
| `limit` | Cap total number of emitted tokens |
| `truncate` | Truncate tokens to max character length |
| `ngram` | Generate character n-grams from tokens |
| `edge_ngram` | Generate edge (prefix) n-grams |
| `shingle` | Generate word n-grams (shingles) |
| `word_delimiter` | Split on intra-word transitions (camelCase, digits) |
| `word_delimiter_graph` | Graph-aware word delimiter (preserves positions) |

</details>

<details>
<summary><strong>English Stemmers</strong></summary>

| Name | Description |
|:-----|:------------|
| `porter_stem` | Porter English stemmer |
| `kstem` | KStem English stemmer (less aggressive) |
| `stemmer` | Configurable multi-language Snowball stemmer |

</details>

<details>
<summary><strong>Language Normalizations</strong> — Script-specific normalization</summary>

| Name | Description |
|:-----|:------------|
| `arabic_normalization` | Arabic character normalization |
| `bengali_normalization` | Bengali character normalization |
| `german_normalization` | German ä→a, ü→u, ß→ss |
| `hindi_normalization` | Hindi character normalization |
| `indic_normalization` | Indic script family normalization |
| `persian_normalization` | Persian character normalization |
| `romanian_normalization` | Romanian diacritic normalization |
| `scandinavian_normalization` | Scandinavian character normalization |
| `scandinavian_folding` | Scandinavian character folding |
| `serbian_normalization` | Serbian Cyrillic → Latin |
| `sorani_normalization` | Sorani Kurdish normalization |

</details>

<details>
<summary><strong>Language-Specific Lowercase</strong></summary>

| Name | Description |
|:-----|:------------|
| `greek_lowercase` | Greek-aware lowercasing (handles final sigma) |
| `irish_lowercase` | Irish-aware lowercasing (preserves nT, tS prefixes) |
| `turkish_lowercase` | Turkish İ/I-aware lowercasing |

</details>

<details>
<summary><strong>Language-Specific Stemmers</strong> — 27 light/minimal stemmers</summary>

| Name | Description |
|:-----|:------------|
| `arabic_stem` | Arabic light stemmer |
| `bengali_stem` | Bengali stemmer |
| `brazilian_stem` | Brazilian Portuguese stemmer |
| `bulgarian_stem` | Bulgarian stemmer |
| `czech_stem` | Czech stemmer |
| `dutch_stem` | Dutch KP stemmer |
| `french_light_stem` | French light stemmer |
| `french_minimal_stem` | French minimal stemmer |
| `galician_stem` | Galician stemmer |
| `galician_minimal_stem` | Galician minimal stemmer |
| `german_light_stem` | German light stemmer |
| `german_minimal_stem` | German minimal stemmer |
| `greek_stem` | Greek stemmer |
| `hindi_stem` | Hindi stemmer |
| `italian_light_stem` | Italian light stemmer |
| `kannada_stem` | Kannada stemmer |
| `latvian_stem` | Latvian stemmer |
| `norwegian_light_stem` | Norwegian light stemmer |
| `persian_stem` | Persian stemmer |
| `portuguese_light_stem` | Portuguese light stemmer |
| `russian_light_stem` | Russian light stemmer |
| `spanish_light_stem` | Spanish light stemmer |
| `tamil_stem` | Tamil stemmer |
| `telugu_stem` | Telugu stemmer |
| `finnish_light_stem` | Finnish light stemmer |
| `hungarian_light_stem` | Hungarian light stemmer |
| `indonesian_stem` | Indonesian stemmer |

</details>

<details>
<summary><strong>Snowball Stemmers</strong> — 33 algorithmic stemmers from <code>analysis-stemmers</code></summary>

| Name | Language |
|:-----|:---------|
| `snowball_arabic` | Arabic |
| `snowball_armenian` | Armenian |
| `snowball_basque` | Basque |
| `snowball_catalan` | Catalan |
| `snowball_czech` | Czech (aggressive) |
| `snowball_czech_light` | Czech (light) |
| `snowball_danish` | Danish |
| `snowball_dutch` | Dutch |
| `snowball_english` | English (Porter 2) |
| `snowball_english_porter` | English (original Porter) |
| `snowball_english_lovins` | English (Lovins) |
| `snowball_estonian` | Estonian |
| `snowball_finnish` | Finnish |
| `snowball_french` | French |
| `snowball_german` | German |
| `snowball_greek` | Greek |
| `snowball_hindi` | Hindi |
| `snowball_hungarian` | Hungarian |
| `snowball_indonesian` | Indonesian |
| `snowball_irish` | Irish |
| `snowball_italian` | Italian |
| `snowball_lithuanian` | Lithuanian |
| `snowball_nepali` | Nepali |
| `snowball_norwegian` | Norwegian |
| `snowball_polish` | Polish |
| `snowball_polish_unaccented` | Polish (unaccented) |
| `snowball_portuguese` | Portuguese |
| `snowball_romanian` | Romanian |
| `snowball_russian` | Russian |
| `snowball_spanish` | Spanish |
| `snowball_swedish` | Swedish |
| `snowball_turkish` | Turkish |
| `snowball_yiddish` | Yiddish |

</details>

<details>
<summary><strong>ICU Filters</strong> — Unicode-aware processing</summary>

| Name | Description |
|:-----|:------------|
| `icu_folding` | Unicode case folding + accent/diacritic removal |
| `icu_normalizer` | NFKC_Casefold normalization |
| `icu_collation` | Locale-aware sort key generation |

</details>

<details>
<summary><strong>Japanese Filters</strong> — Kuromoji pipeline components</summary>

| Name | Description |
|:-----|:------------|
| `kuromoji_baseform` | Reduce conjugated forms to dictionary form |
| `kuromoji_part_of_speech` | Filter tokens by part-of-speech tags |
| `kuromoji_readingform` | Output katakana/romaji readings |
| `kuromoji_stemmer` | Stem katakana long vowels (ー) |
| `kuromoji_number` | Normalize kanji numerals to Arabic digits |
| `ja_stop` | Japanese stop words |

</details>

<details>
<summary><strong>Korean Filters</strong> — Nori pipeline components</summary>

| Name | Description |
|:-----|:------------|
| `nori_part_of_speech` | Filter tokens by part-of-speech tags |
| `nori_readingform` | Convert Hanja to Hangul reading form |
| `ko_stop` | Korean stop words |

</details>

<details>
<summary><strong>Chinese Filters</strong></summary>

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `smartcn_stop` | smartcn | Chinese/English stop words |
| `stconvert_s2t` | stconvert | Simplified → Traditional Chinese conversion |
| `stconvert_t2s` | stconvert | Traditional → Simplified Chinese conversion |

</details>

<details>
<summary><strong>Polish & Ukrainian Filters</strong></summary>

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `stempel_stem` | stempel | Polish Stempel stemmer (Egothor multi-trie) |
| `polish_stop` | stempel | Polish stop words (186 entries) |
| `morfologik_stem` | morfologik | Polish lemmatizer (dictionary-derived rules) |
| `ukrainian_stem` | morfologik | Ukrainian stemmer (suffix rules) |
| `ukrainian_stop` | morfologik | Ukrainian stop words (1,269 entries) |

</details>

---

### Normalizers (13)

Character-level transformations applied **before** tokenization:

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `html_strip` | core | Strip HTML/XML tags |
| `trim` | core | Trim leading/trailing whitespace |
| `collapse_whitespace` | core | Collapse runs of whitespace to a single space |
| `lowercase` | core | Lowercase the entire input |
| `uppercase` | core | Uppercase the entire input |
| `unicode_nfc` | core | Unicode NFC normalization |
| `unicode_nfd` | core | Unicode NFD normalization |
| `unicode_nfkc` | core | Unicode NFKC normalization |
| `unicode_nfkd` | core | Unicode NFKD normalization |
| `pinyin` | pinyin | Convert Chinese characters to Pinyin |
| `pinyin_first_letter` | pinyin | Extract first letter of each syllable |
| `stconvert_s2t` | stconvert | Simplified → Traditional Chinese |
| `stconvert_t2s` | stconvert | Traditional → Simplified Chinese |

---

### Pre-built Analyzers (70+)

#### Language Analyzers

Full analysis pipelines with stop words and stemming for **65+ languages**:

> `afrikaans` · `amharic` · `arabic` · `armenian`\* · `azerbaijani` · `basque`\* · `bengali` · `brazilian` · `bulgarian` · `catalan`\* · `cjk` · `croatian` · `czech` · `danish` · `dutch` · `english` · `estonian`\* · `filipino` · `finnish` · `french` · `galician` · `georgian` · `german` · `greek` · `hebrew` · `hindi` · `hungarian` · `indonesian` · `irish` · `italian` · `latvian` · `lithuanian`\* · `malay` · `marathi` · `mongolian` · `nepali` · `norwegian` · `persian` · `polish`\* · `portuguese` · `romanian` · `russian` · `serbian` · `slovak` · `slovenian` · `sorani` · `spanish` · `swahili` · `swedish`\* · `tagalog` · `tamil` · `thai` · `turkish`\* · `ukrainian` · `urdu` · `vietnamese`

<sub>\* Enhanced with Snowball stemmer via `analysis-stemmers` (overrides the stop-only version in `analysis-core`).</sub>

<sub>**Bold languages** have dedicated per-language crates with extended stop words and language-specific normalizations: **arabic**, **bengali**, **brazilian**, **dutch**, **english**, **finnish**, **french**, **german**, **greek**, **hindi**, **hungarian**, **indonesian**, **italian**, **norwegian**, **persian**, **portuguese**, **russian**, **spanish**, **swedish**, **turkish**.</sub>

#### Utility Analyzers

| Name | Description |
|:-----|:------------|
| `fingerprint` | Lowercased, sorted, deduplicated — ideal for deduplication |
| `simple` | Letter tokenizer + lowercasing |
| `stop` | Standard tokenizer + lowercasing + English stop words |
| `keyword` | No-op pass-through (entire input as single token) |
| `pattern` | Configurable regex-based tokenization |
| `whitespace` | Whitespace-only tokenization |

#### CJK & Asian Analyzers

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `ik_smart` | ik | Smart segmentation (non-overlapping, best for queries) |
| `ik_max_word` | ik | Maximum segmentation (all hits, best for indexing) |
| `jieba` | jieba | Jieba segmentation with new-word detection |
| `kuromoji` | kuromoji | Full Japanese pipeline: baseform → POS → stop → stemmer |
| `nori` | nori | Full Korean pipeline: POS filtering → Hanja reading |
| `smartcn` | smartcn | SmartCN pipeline: Viterbi segmenter → stop filter |
| `pinyin` | pinyin | Chinese → Pinyin romanization |
| `stconvert_s2t` | stconvert | Simplified → Traditional Chinese |
| `stconvert_t2s` | stconvert | Traditional → Simplified Chinese |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          pizza-analysis-all                                  │
│                    register_all(&mut factory)                                │
├─────────┬──────────┬─────────────────────────────────────────┬──────────────┤
│  core   │ stemmers │             per-language (21)            │   CJK/Asian  │
│         │          │  english · french · german · spanish ... │  ik · jieba  │
│         │          │  arabic · hindi · persian · greek ...    │  kuromoji    │
│         │          │  bengali · indonesian · brazilian ...    │  nori · cjk  │
├─────────┴──────────┴─────────────────────────────────────────┴──────────────┤
│                            pizza-engine                                      │
│                      AnalysisFactory registry                                │
└─────────────────────────────────────────────────────────────────────────────┘
```

Each plugin independently registers its components into the shared `AnalysisFactory`. Plugins are **compile-time optional** via Cargo features — unused plugins are completely eliminated from the binary.

### Registration Order

Per-language crates are registered **after** `analysis-core`, intentionally overriding core's basic analyzers with richer pipelines that include:
- Extended stop word corpora (200–400 words vs. core's minimal lists)
- Language-specific character normalizations
- Dedicated stemming algorithms

### Design: Monolith vs. Modular

| Strategy | Usage |
|:---------|:------|
| **Monolith** | Use `analysis-core` alone — batteries-included with 65+ languages |
| **Modular** | Use individual per-language crates — fine-grained control, smaller binaries |
| **Combined** | `analysis-all` with all features — maximum quality, per-language crates override core |

The `AnalysisFactory` uses `HashMap::insert` semantics — later registrations override earlier ones with the same name. This is safe and intentional.

## Plugin Discovery

A crate is auto-discovered as a plugin when:

1. Its package name starts with `pizza-analysis-`
2. It exports `pub fn register_all(factory: &mut AnalysisFactory)` at the crate root

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs/">INFINI Pizza</a> project.</sub>
</div>
