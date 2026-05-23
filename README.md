<div align="center">

# 🍕 Pizza Analysis

### Multilingual Text Analysis for INFINI Pizza — Pure Rust

[![Rust](https://img.shields.io/badge/Rust-nightly-orange?logo=rust)](https://www.rust-lang.org/)
[![Plugins](https://img.shields.io/badge/plugins-39-blue)]()
[![Languages](https://img.shields.io/badge/languages-70%2B-brightgreen)]()
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

**27 tokenizers** · **140+ token filters** · **13 normalizers** · **70+ pre-built analyzers** · **39 language plugins**

*From Arabic to Vietnamese — the most comprehensive text analysis ecosystem in Rust.*

[Getting Started](#quick-start) · [Plugin Catalog](#plugin-catalog) · [Component Reference](#component-reference) · [Architecture](#architecture)

</div>

---

## Overview

`pizza-analysis-all` is the unified meta-crate for [INFINI Pizza](https://pizza.rs)'s text analysis pipeline. One function call registers **39 specialized plugins** covering every major writing system:

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_all::register_all(&mut factory);
// → 27 tokenizers, 140+ filters, 13 normalizers, 70+ analyzers ready
```

### Key Capabilities

- **🌍 Auto Language Detection** — Automatically detects the language of incoming text and delegates to the best analyzer
- **CJK Segmentation** — IK, Jieba, SmartCN (Chinese), Kuromoji (Japanese), Nori (Korean)
- **Southeast Asian** — Vietnamese compound word tokenizer, Thai Sara Am decomposition
- **South Asian (Indic)** — Hindi, Bengali, Tamil, Telugu, Kannada, Malayalam with dedicated normalization and stemming
- **ICU Unicode** — UAX#29 segmentation, NFKC normalization, case folding, collation
- **33 Snowball Stemmers** — Arabic through Yiddish, algorithmically derived
- **27 Dedicated Language Crates** — Extended stop words, script normalization, specialized stemming
- **Synonym Expansion** — Single-word and graph-aware multi-word synonym support
- **Pinyin & ST Conversion** — Chinese romanization and Simplified/Traditional conversion
- **Dictionary Lemmatization** — Polish (Morfologik + Stempel) and Ukrainian
- **Zero-allocation paths** — `no_std` compatible, `Cow<str>` throughout, arena-friendly

---

## Quick Start

### Full Suite (all 33 plugins)

```toml
[dependencies]
pizza-analysis-all = "0.1"
```

### Selective Features

Enable only what you need — each plugin is a Cargo feature:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", default-features = false, features = ["core", "jieba", "english", "synonym"] }
```

Feature names correspond to crate names with the `pizza-analysis-` prefix stripped.

---

## Plugin Catalog

### Foundation

| Feature | Crate | Description |
|:--------|:------|:------------|
| **`core`** | [analysis-core](https://github.com/pizza-rs/analysis-core) | 16 tokenizers, 60+ filters, 65 built-in language analyzers, HTML/Unicode normalizers |
| **`stemmers`** | [analysis-stemmers](https://github.com/pizza-rs/analysis-stemmers) | Snowball algorithmic stemmers for 33 languages |
| **`icu`** | [analysis-icu](https://github.com/pizza-rs/analysis-icu) | ICU4X Unicode segmentation, NFC/NFKC normalization, case folding, collation sort keys |
| **`synonym`** | [analysis-synonym](https://github.com/pizza-rs/analysis-synonym) | Single-word and graph-aware multi-word synonym expansion/contraction |

### CJK & Asian Languages

| Feature | Crate | Description |
|:--------|:------|:------------|
| **`cjk`** | [analysis-cjk](https://github.com/pizza-rs/analysis-cjk) | CJK bigram tokenizer, fullwidth/halfwidth normalization, CJK stop words |
| **`ik`** | [analysis-ik](https://github.com/pizza-rs/analysis-ik) | IK Chinese segmentation — smart mode (queries) + max-word mode (indexing) |
| **`jieba`** | [analysis-jieba](https://github.com/pizza-rs/analysis-jieba) | Jieba Chinese segmentation with HMM new-word detection |
| **`kuromoji`** | [analysis-kuromoji](https://github.com/pizza-rs/analysis-kuromoji) | Japanese morphological analysis — IPADIC dictionary, baseform, reading, POS filtering |
| **`nori`** | [analysis-nori](https://github.com/pizza-rs/analysis-nori) | Korean morphological analysis — mecab-ko-dic, decompounding, Hanja→Hangul |
| **`pinyin`** | [analysis-pinyin](https://github.com/pizza-rs/analysis-pinyin) | Chinese → Pinyin romanization with polyphone disambiguation |
| **`smartcn`** | [analysis-smartcn](https://github.com/pizza-rs/analysis-smartcn) | SmartCN Chinese segmentation — Viterbi algorithm + DARTS double-array trie |
| **`stconvert`** | [analysis-stconvert](https://github.com/pizza-rs/analysis-stconvert) | Simplified ↔ Traditional Chinese conversion (CN/TW/HK/JP variants) |
| **`vietnamese`** | [analysis-vietnamese](https://github.com/pizza-rs/analysis-vietnamese) | 🇻🇳 Vietnamese compound word tokenizer — forward maximum matching |
| **`thai`** | [analysis-thai](https://github.com/pizza-rs/analysis-thai) | 🇹🇭 Thai Sara Am decomposition, Thai digit normalization, stop words |

### Per-Language Analysis (27 crates)

Each crate provides a complete pipeline: language-specific normalization → extended stop words → dedicated stemmer.

| Feature | Crate | Highlights |
|:--------|:------|:-----------|
| **`arabic`** | [analysis-arabic](https://github.com/pizza-rs/analysis-arabic) | Diacritics removal, ALEF/YEH/TEH normalization, light stemmer, 249 stop words |
| **`bengali`** | [analysis-bengali](https://github.com/pizza-rs/analysis-bengali) | Script normalization, inflectional suffix stemmer, stop words |
| **`brazilian`** | [analysis-brazilian](https://github.com/pizza-rs/analysis-brazilian) | RSLP stemmer (plural/feminine/augmentative/adverb rules), stop words |
| **`dutch`** | [analysis-dutch](https://github.com/pizza-rs/analysis-dutch) | Suffix stemmer (plurals, diminutives), 222 stop words |
| **`english`** | [analysis-english](https://github.com/pizza-rs/analysis-english) | KStem stemmer, possessive filter (`'s` removal), 245 stop words |
| **`finnish`** | [analysis-finnish](https://github.com/pizza-rs/analysis-finnish) | Agglutinative case-ending stripper, vowel harmony handling, stop words |
| **`french`** | [analysis-french](https://github.com/pizza-rs/analysis-french) | Elision filter (l'/d'/qu'), light stemmer, 321 stop words |
| **`german`** | [analysis-german](https://github.com/pizza-rs/analysis-german) | ß→ss, umlaut expansion (ä→a), light stemmer, 391 stop words |
| **`greek`** | [analysis-greek](https://github.com/pizza-rs/analysis-greek) | Accent/tonos removal, Ntais stemmer, stop words |
| **`hindi`** | [analysis-hindi](https://github.com/pizza-rs/analysis-hindi) | Devanagari normalization, Indic base forms, suffix stemmer, stop words |
| **`hungarian`** | [analysis-hungarian](https://github.com/pizza-rs/analysis-hungarian) | Case/plural suffix stemmer, stop words |
| **`indonesian`** | [analysis-indonesian](https://github.com/pizza-rs/analysis-indonesian) | AFNLP prefix/suffix stemmer, stop words |
| **`italian`** | [analysis-italian](https://github.com/pizza-rs/analysis-italian) | Elision filter, light stemmer, 328 stop words |
| **`norwegian`** | [analysis-norwegian](https://github.com/pizza-rs/analysis-norwegian) | Light stemmer (Bokmål + Nynorsk), stop words |
| **`persian`** | [analysis-persian](https://github.com/pizza-rs/analysis-persian) | Farsi character normalization, affix stemmer, stop words |
| **`portuguese`** | [analysis-portuguese](https://github.com/pizza-rs/analysis-portuguese) | Light stemmer, 359 stop words |
| **`russian`** | [analysis-russian](https://github.com/pizza-rs/analysis-russian) | ё→е normalization, light stemmer, 301 stop words |
| **`spanish`** | [analysis-spanish](https://github.com/pizza-rs/analysis-spanish) | Light stemmer, 325 stop words |
| **`swedish`** | [analysis-swedish](https://github.com/pizza-rs/analysis-swedish) | Snowball-style stemmer, stop words |
| **`turkish`** | [analysis-turkish](https://github.com/pizza-rs/analysis-turkish) | Locale-aware lowercase (dotted/dotless İ/I), suffix stemmer, 261 stop words |
| **`tamil`** | [analysis-tamil](https://github.com/pizza-rs/analysis-tamil) | 🇮🇳 Tamil digit normalization, old numeral removal, Indic normalization, stemmer, 100+ stop words |
| **`telugu`** | [analysis-telugu](https://github.com/pizza-rs/analysis-telugu) | 🇮🇳 Telugu digit normalization, Indic normalization, stemmer, 90+ stop words |
| **`kannada`** | [analysis-kannada](https://github.com/pizza-rs/analysis-kannada) | 🇮🇳 Kannada digit normalization, Indic normalization, stemmer, 90+ stop words |
| **`malayalam`** | [analysis-malayalam](https://github.com/pizza-rs/analysis-malayalam) | 🇮🇳 Malayalam digit + chillu normalization, Indic normalization, 90+ stop words |

### Dictionary-Based

| Feature | Crate | Description |
|:--------|:------|:------------|
| **`morfologik`** | [analysis-morfologik](https://github.com/pizza-rs/analysis-morfologik) | Polish & Ukrainian dictionary-based lemmatization (Morfologik FSA) |
| **`stempel`** | [analysis-stempel](https://github.com/pizza-rs/analysis-stempel) | Polish Stempel stemmer — Egothor multi-trie automaton |

### Auto Language Detection

| Feature | Crate | Description |
|:--------|:------|:------------|
| **`auto`** | [analysis-auto](https://github.com/pizza-rs/analysis-auto) | 🔮 Automatic language detection via [whatlang](https://crates.io/crates/whatlang) — routes text to the best analyzer at runtime, supports per-language overrides and configurable confidence threshold |

---

## 🌍 Auto Analyzer — Language Detection at Analysis Time

The `auto` analyzer removes the need to know the language of a document in advance.
It detects the language of incoming text using [whatlang](https://crates.io/crates/whatlang)
and delegates to the matching language-specific analyzer — all transparently.

### How it works

```
Input text  →  whatlang detection  →  language + confidence
                                          │
                     ┌────────────────────┼────────────────────┐
                     ▼                    ▼                    ▼
              confidence ≥ threshold  confidence < threshold  no detection
                     │                use "standard"     use "standard"
                     ▼
              check overrides → use override OR default mapping
```

### Examples

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_all::register_all(&mut factory);

let auto = factory.get_analyzer("auto").unwrap();

// English input → delegates to "english" analyzer
let mut text = "The runners were quickly running".to_string();
let tokens = auto.analyze_and_return_tokens(&mut text);
// → ["runner", "quickly", "run"]  (stop words removed, KStem stemmed)

// French input → delegates to "french" analyzer
let mut text = "Les enfants jouaient dans le jardin".to_string();
let tokens = auto.analyze_and_return_tokens(&mut text);
// → ["enfant", "jouai", "jardin"]  (elision, stop words, light stemmer)

// Chinese input → delegates to "ik" analyzer
let mut text = "全文搜索引擎".to_string();
let tokens = auto.analyze_and_return_tokens(&mut text);
// → ["全文", "搜索引擎"]  (IK smart segmentation)

// Mixed/ambiguous input → falls back to "standard"
let mut text = "12345".to_string();
let tokens = auto.analyze_and_return_tokens(&mut text);
// → ["12345"]  (standard tokenizer)
```

### When to use `auto`

| Scenario | Recommendation |
|:---------|:---------------|
| Multilingual corpus, language unknown | ✅ Use `auto` |
| Single-language index (e.g., all English) | Use the dedicated analyzer for best quality |
| Mixed-language documents | ✅ Use `auto` — each field analyzed independently |
| Short text (1–2 words) | Detection may be uncertain — `auto` falls back to `standard` |

> **Note:** The `auto` analyzer must be registered **last** (after all language analyzers)
> so it can capture them for delegation. `pizza_analysis_all::register_all()` handles
> this automatically.

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
<summary><strong>CJK & Asian</strong> — 10 specialized tokenizers</summary>

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `icu_tokenizer` | icu | Unicode UAX#29 segmentation via ICU4X (all scripts) |
| `ik_smart` | ik | Chinese — smart mode (non-overlapping, best for queries) |
| `ik_max_word` | ik | Chinese — max-word mode (all dictionary hits, best for indexing) |
| `jieba` | jieba | Chinese — Jieba search mode segmentation |
| `kuromoji_tokenizer` | kuromoji | Japanese morphological tokenizer (IPADIC dictionary) |
| `nori_tokenizer` | nori | Korean morphological tokenizer (mecab-ko-dic) |
| `pinyin` | pinyin | Chinese → Pinyin romanization tokenizer |
| `smartcn_tokenizer` | smartcn | Chinese — Viterbi dynamic programming segmenter |
| `stconvert_s2t` | stconvert | Simplified → Traditional Chinese tokenizer |
| `stconvert_t2s` | stconvert | Traditional → Simplified Chinese tokenizer |

</details>

---

### Token Filters (130+)

<details>
<summary><strong>Text Transformation</strong></summary>

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
<summary><strong>Synonym Filters</strong></summary>

| Name | Description |
|:-----|:------------|
| `synonym` | Single-word synonym expansion/contraction |
| `synonym_graph` | Graph-aware multi-word synonym filter (preserves phrase query correctness) |

</details>

<details>
<summary><strong>Stemmers — English</strong></summary>

| Name | Description |
|:-----|:------------|
| `porter_stem` | Porter English stemmer |
| `kstem` | KStem English stemmer (less aggressive) |
| `stemmer` | Configurable multi-language Snowball stemmer |

</details>

<details>
<summary><strong>Stemmers — Language-Specific (27)</strong></summary>

| Name | Description |
|:-----|:------------|
| `arabic_stem` | Arabic light stemmer |
| `bengali_stem` | Bengali stemmer |
| `brazilian_stem` | Brazilian Portuguese RSLP stemmer |
| `bulgarian_stem` | Bulgarian stemmer |
| `czech_stem` | Czech stemmer |
| `dutch_stem` | Dutch KP stemmer |
| `finnish_light_stem` | Finnish light stemmer |
| `french_light_stem` | French light stemmer |
| `french_minimal_stem` | French minimal stemmer |
| `galician_stem` | Galician stemmer |
| `galician_minimal_stem` | Galician minimal stemmer |
| `german_light_stem` | German light stemmer |
| `german_minimal_stem` | German minimal stemmer |
| `greek_stem` | Greek Ntais stemmer |
| `hindi_stem` | Hindi suffix stemmer |
| `hungarian_light_stem` | Hungarian light stemmer |
| `indonesian_stem` | Indonesian AFNLP stemmer |
| `italian_light_stem` | Italian light stemmer |
| `kannada_stem` | Kannada stemmer |
| `latvian_stem` | Latvian stemmer |
| `norwegian_light_stem` | Norwegian light stemmer |
| `persian_stem` | Persian affix stemmer |
| `portuguese_light_stem` | Portuguese light stemmer |
| `russian_light_stem` | Russian light stemmer |
| `spanish_light_stem` | Spanish light stemmer |
| `tamil_stem` | Tamil stemmer |
| `telugu_stem` | Telugu stemmer |

</details>

<details>
<summary><strong>Stemmers — Snowball (33 languages)</strong></summary>

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
<summary><strong>Language Normalizations</strong> — Script-specific filters</summary>

| Name | Description |
|:-----|:------------|
| `arabic_normalization` | Diacritics removal, ALEF/YEH/TEH Marbuta normalization |
| `bengali_normalization` | Bengali script normalization |
| `german_normalization` | ä→a, ü→u, ö→o, ß→ss |
| `hindi_normalization` | Devanagari character normalization |
| `indic_normalization` | Pan-Indic script family normalization |
| `persian_normalization` | Farsi character normalization |
| `tamil_normalization` | Tamil digit (௦-௯→0-9) and old numeral sign removal |
| `telugu_normalization` | Telugu digit (౦-౯→0-9) normalization |
| `kannada_normalization` | Kannada digit (೦-೯→0-9) normalization |
| `malayalam_normalization` | Malayalam digit (൦-൯→0-9) and chillu letter normalization |
| `thai_normalization` | Sara Am decomposition, Thai digit (๐-๙→0-9) normalization |
| `vietnamese_normalization` | Vietnamese Đ/đ→d normalization |
| `romanian_normalization` | Romanian diacritic normalization |
| `scandinavian_normalization` | Scandinavian character equivalence |
| `scandinavian_folding` | Scandinavian character folding |
| `serbian_normalization` | Serbian Cyrillic → Latin transliteration |
| `sorani_normalization` | Sorani Kurdish normalization |

</details>

<details>
<summary><strong>Language-Specific Lowercase</strong></summary>

| Name | Description |
|:-----|:------------|
| `greek_lowercase` | Greek-aware (handles final sigma σ/ς) |
| `irish_lowercase` | Irish-aware (preserves nT, tS prefixes) |
| `turkish_lowercase` | Turkish İ/I-aware (dotted/dotless handling) |

</details>

<details>
<summary><strong>ICU Filters</strong></summary>

| Name | Description |
|:-----|:------------|
| `icu_folding` | Unicode case folding + accent/diacritic removal |
| `icu_normalizer` | NFC/NFKC/NFKC_Casefold normalization per-token |
| `icu_collation` | Locale-aware binary sort key generation |

</details>

<details>
<summary><strong>Japanese (Kuromoji)</strong></summary>

| Name | Description |
|:-----|:------------|
| `kuromoji_baseform` | Reduce conjugated verbs/adjectives to dictionary form |
| `kuromoji_part_of_speech` | Remove tokens by configurable POS tags |
| `kuromoji_readingform` | Output katakana or romaji readings |
| `kuromoji_stemmer` | Stem katakana long vowels (ー) |
| `kuromoji_number` | Normalize kanji numerals to Arabic digits |
| `ja_stop` | Japanese stop words |

</details>

<details>
<summary><strong>Korean (Nori)</strong></summary>

| Name | Description |
|:-----|:------------|
| `nori_part_of_speech` | Remove tokens by POS tags (particles, suffixes, etc.) |
| `nori_readingform` | Convert Hanja (漢字) to Hangul reading form |
| `ko_stop` | Korean stop words |

</details>

<details>
<summary><strong>Chinese</strong></summary>

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `smartcn_stop` | smartcn | Chinese + English stop words |
| `stconvert_s2t` | stconvert | Simplified → Traditional Chinese token filter |
| `stconvert_t2s` | stconvert | Traditional → Simplified Chinese token filter |

</details>

<details>
<summary><strong>Polish & Ukrainian</strong></summary>

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `stempel_stem` | stempel | Polish Stempel stemmer (Egothor multi-trie automaton) |
| `polish_stop` | stempel | Polish stop words (186 entries) |
| `morfologik_stem` | morfologik | Polish dictionary-based lemmatizer |
| `ukrainian_stem` | morfologik | Ukrainian suffix-rule stemmer |
| `ukrainian_stop` | morfologik | Ukrainian stop words (1,269 entries) |

</details>

<details>
<summary><strong>Per-Language Stop Filters (21)</strong></summary>

Each per-language crate registers its own stop filter with extended corpora:

| Name | Words | Source |
|:-----|:------|:-------|
| `arabic_stop` | 249 | Lucene/Snowball |
| `bengali_stop` | — | Common Bengali function words |
| `brazilian_stop` | — | Brazilian Portuguese stop words |
| `dutch_stop` | 222 | Snowball Dutch |
| `english_stop` | 245 | Lucene default English |
| `finnish_stop` | — | Finnish function words |
| `french_stop` | 321 | Snowball French |
| `german_stop` | 391 | Snowball German |
| `greek_stop` | — | Greek function words |
| `hindi_stop` | — | Hindi function words |
| `hungarian_stop` | — | Hungarian function words |
| `indonesian_stop` | — | Indonesian function words |
| `italian_stop` | 328 | Snowball Italian |
| `norwegian_stop` | — | Norwegian function words |
| `persian_stop` | — | Farsi function words |
| `portuguese_stop` | 359 | Snowball Portuguese |
| `russian_stop` | 301 | Snowball Russian |
| `spanish_stop` | 325 | Snowball Spanish |
| `swedish_stop` | — | Swedish function words |
| `turkish_stop` | 261 | Turkish function words |
| `tamil_stop` | 100+ | Tamil function words |
| `telugu_stop` | 90+ | Telugu function words |
| `kannada_stop` | 90+ | Kannada function words |
| `malayalam_stop` | 90+ | Malayalam function words |
| `vietnamese_stop` | 200+ | Vietnamese function words |
| `thai_stop` | 112 | Thai function words |

</details>

---

### Normalizers (13)

Character-level transformations applied **before** tokenization on the raw input text:

| Name | Plugin | Description |
|:-----|:-------|:------------|
| `html_strip` | core | Strip HTML/XML tags, decode entities |
| `trim` | core | Trim leading/trailing whitespace |
| `collapse_whitespace` | core | Collapse runs of whitespace to a single space |
| `lowercase` | core | Lowercase the entire input |
| `uppercase` | core | Uppercase the entire input |
| `unicode_nfc` | core | Unicode NFC normalization |
| `unicode_nfd` | core | Unicode NFD normalization |
| `unicode_nfkc` | core | Unicode NFKC normalization |
| `unicode_nfkd` | core | Unicode NFKD normalization |
| `pinyin` | pinyin | Convert Chinese characters to Pinyin |
| `pinyin_first_letter` | pinyin | Extract first letter of each Pinyin syllable |
| `stconvert_s2t` | stconvert | Simplified → Traditional Chinese |
| `stconvert_t2s` | stconvert | Traditional → Simplified Chinese |

---

### Pre-built Analyzers (70+)

#### Language Analyzers

Full analysis pipelines with stop words and stemming:

| Language | Analyzer Name | Pipeline |
|:---------|:--------------|:---------|
| Afrikaans | `afrikaans` | standard → lowercase → stop |
| Amharic | `amharic` | standard → lowercase → stop |
| Arabic | `arabic` | standard → lowercase → normalization → stop → stem |
| Armenian | `armenian` | standard → lowercase → stop → snowball |
| Azerbaijani | `azerbaijani` | standard → lowercase → stop |
| Basque | `basque` | standard → lowercase → stop → snowball |
| Bengali | `bengali` | standard → lowercase → indic_normalization → bengali_normalization → stop → stem |
| Brazilian | `brazilian` | standard → lowercase → stop → brazilian_stem |
| Bulgarian | `bulgarian` | standard → lowercase → stop → snowball |
| Catalan | `catalan` | standard → lowercase → elision → stop → snowball |
| CJK | `cjk` | standard → cjk_width → lowercase → cjk_bigram → stop |
| Croatian | `croatian` | standard → lowercase → stop |
| Czech | `czech` | standard → lowercase → stop → snowball |
| Danish | `danish` | standard → lowercase → stop → snowball |
| Dutch | `dutch` | standard → lowercase → stop → dutch_stem |
| English | `english` | standard → lowercase → possessive → stop → kstem |
| Estonian | `estonian` | standard → lowercase → stop → snowball |
| Filipino | `filipino` | standard → lowercase → stop |
| Finnish | `finnish` | standard → lowercase → stop → finnish_light_stem |
| French | `french` | standard → lowercase → elision → stop → french_light_stem |
| Galician | `galician` | standard → lowercase → stop → snowball |
| Georgian | `georgian` | standard → lowercase → stop |
| German | `german` | standard → lowercase → german_normalization → stop → german_light_stem |
| Greek | `greek` | standard → greek_lowercase → stop → greek_stem |
| Hebrew | `hebrew` | standard → lowercase → stop |
| Hindi | `hindi` | standard → lowercase → indic_normalization → hindi_normalization → stop → hindi_stem |
| Hungarian | `hungarian` | standard → lowercase → stop → hungarian_light_stem |
| Indonesian | `indonesian` | standard → lowercase → stop → indonesian_stem |
| Irish | `irish` | standard → irish_lowercase → stop → snowball |
| Italian | `italian` | standard → lowercase → elision → stop → italian_light_stem |
| Latvian | `latvian` | standard → lowercase → stop → snowball |
| Lithuanian | `lithuanian` | standard → lowercase → stop → snowball |
| Malay | `malay` | standard → lowercase → stop |
| Marathi | `marathi` | standard → lowercase → indic_normalization → stop |
| Mongolian | `mongolian` | standard → lowercase → stop |
| Nepali | `nepali` | standard → lowercase → stop → snowball |
| Norwegian | `norwegian` | standard → lowercase → stop → norwegian_light_stem |
| Persian | `persian` | standard → lowercase → persian_normalization → stop |
| Polish | `polish` | standard → lowercase → stop → stempel_stem |
| Portuguese | `portuguese` | standard → lowercase → stop → portuguese_light_stem |
| Romanian | `romanian` | standard → lowercase → stop → snowball |
| Russian | `russian` | standard → lowercase → stop → russian_light_stem |
| Serbian | `serbian` | standard → lowercase → serbian_normalization → stop |
| Slovak | `slovak` | standard → lowercase → stop |
| Slovenian | `slovenian` | standard → lowercase → stop |
| Sorani | `sorani` | standard → lowercase → sorani_normalization → stop |
| Spanish | `spanish` | standard → lowercase → stop → spanish_light_stem |
| Swahili | `swahili` | standard → lowercase → stop |
| Swedish | `swedish` | standard → lowercase → stop → snowball |
| Tagalog | `tagalog` | standard → lowercase → stop |
| Tamil | `tamil` | standard → indic_normalization → tamil_normalization → lowercase → decimal_digit → stop → tamil_stem |
| Telugu | `telugu` | standard → indic_normalization → telugu_normalization → lowercase → decimal_digit → stop → telugu_stem |
| Thai | `thai` | thai → normalize → lowercase → stop |
| Turkish | `turkish` | standard → turkish_lowercase → stop → snowball |
| Ukrainian | `ukrainian` | standard → lowercase → stop → ukrainian_stem |
| Urdu | `urdu` | standard → lowercase → stop |
| Vietnamese | `vietnamese` | vietnamese → normalize → stop |
| Kannada | `kannada` | standard → indic_normalization → kannada_normalization → lowercase → decimal_digit → stop → kannada_stem |
| Malayalam | `malayalam` | standard → indic_normalization → malayalam_normalization → lowercase → decimal_digit → stop |

#### CJK & Asian Analyzers

| Name | Plugin | Pipeline |
|:-----|:-------|:---------|
| `ik_smart` | ik | IK smart segmentation → lowercase |
| `ik_max_word` | ik | IK max-word segmentation → lowercase |
| `jieba` | jieba | Jieba segmentation → lowercase |
| `kuromoji` | kuromoji | kuromoji_tokenizer → baseform → POS filter → stop → stemmer |
| `nori` | nori | nori_tokenizer → POS filter → readingform |
| `smartcn` | smartcn | smartcn_tokenizer → smartcn_stop |
| `pinyin` | pinyin | pinyin_tokenizer → lowercase |
| `stconvert_s2t` | stconvert | stconvert_s2t_tokenizer |
| `stconvert_t2s` | stconvert | stconvert_t2s_tokenizer |

#### Utility Analyzers

| Name | Description |
|:-----|:------------|
| `standard` | Standard tokenizer + lowercase (no stop words) |
| `simple` | Letter tokenizer + lowercase |
| `stop` | Standard tokenizer + lowercase + English stop words |
| `keyword` | No-op — entire input as single token |
| `pattern` | Configurable regex-based tokenization + lowercase |
| `whitespace` | Whitespace-only tokenization |
| `fingerprint` | Lowercase, sorted, deduplicated — ideal for record deduplication |

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                             pizza-analysis-all                                    │
│                       register_all(&mut AnalysisFactory)                          │
├──────────┬───────────┬─────────────────────────────────────────┬─────────────────┤
│  core    │ stemmers  │          per-language (27)               │   CJK / Asian   │
│  60+ flt │ 33 langs  │  english · french · german · spanish    │  ik · jieba     │
│  16 tok  │           │  arabic · hindi · tamil · telugu ...    │  kuromoji · nori│
│  65 anlz │           │  bengali · vietnamese · thai · ...      │  smartcn · cjk  │
├──────────┼───────────┼─────────────────────────────────────────┼─────────────────┤
│   icu    │  synonym  │           morfologik · stempel           │ pinyin·stconvert│
├──────────┴───────────┴─────────────────────────────────────────┴─────────────────┤
│                    🌍 auto — language detection (registered last)                 │
├──────────────────────────────────────────────────────────────────────────────────┤
│                              pizza-engine                                         │
│                  AnalysisFactory · Token · Tokenizer · TokenFilter                │
└──────────────────────────────────────────────────────────────────────────────────┘
```

### Design Principles

- **Compile-time modularity** — Each plugin is a Cargo feature. Unused plugins are completely eliminated from the binary.
- **Override semantics** — Per-language crates register *after* `core`, intentionally overriding basic analyzers with richer pipelines (extended stop words, language-specific normalization, dedicated stemmers).
- **`no_std` compatible** — All crates work without the standard library (`alloc` only), enabling embedded and WASM targets.
- **Zero-copy where possible** — `Cow<'_, str>` token terms avoid allocation when the term is unchanged.

### Registration Order

1. **Foundation**: `core` → `stemmers` → `icu`
2. **CJK & Asian**: `cjk` → `ik` → `jieba` → `kuromoji` → `nori` → `pinyin` → `smartcn` → `stconvert`
3. **Per-Language** (27 crates): Each overrides core's basic analyzer with full pipeline
4. **Dictionary**: `morfologik` → `stempel`
5. **Cross-cutting**: `synonym`
6. **Auto detection**: `auto` *(must be last — captures all analyzers above)*

---

## Feature Matrix

| Feature | Default | Description |
|:--------|:-------:|:------------|
| `std` | ✅ | Enable standard library support |
| `core` | ✅ | Foundation tokenizers, filters, analyzers |
| `stemmers` | ✅ | 33 Snowball algorithmic stemmers |
| `icu` | ✅ | ICU4X Unicode processing |
| `synonym` | ✅ | Synonym expansion/contraction |
| `cjk` | ✅ | CJK bigram and width normalization |
| `ik` | ✅ | IK Chinese segmentation |
| `jieba` | ✅ | Jieba Chinese segmentation |
| `kuromoji` | ✅ | Japanese morphological analysis |
| `nori` | ✅ | Korean morphological analysis |
| `pinyin` | ✅ | Chinese Pinyin conversion |
| `smartcn` | ✅ | SmartCN Chinese segmentation |
| `stconvert` | ✅ | Simplified/Traditional Chinese |
| `english` | ✅ | English analysis |
| `french` | ✅ | French analysis |
| `german` | ✅ | German analysis |
| `spanish` | ✅ | Spanish analysis |
| `italian` | ✅ | Italian analysis |
| `portuguese` | ✅ | Portuguese analysis |
| `dutch` | ✅ | Dutch analysis |
| `russian` | ✅ | Russian analysis |
| `greek` | ✅ | Greek analysis |
| `norwegian` | ✅ | Norwegian analysis |
| `swedish` | ✅ | Swedish analysis |
| `finnish` | ✅ | Finnish analysis |
| `hungarian` | ✅ | Hungarian analysis |
| `turkish` | ✅ | Turkish analysis |
| `arabic` | ✅ | Arabic analysis |
| `persian` | ✅ | Persian analysis |
| `hindi` | ✅ | Hindi analysis |
| `bengali` | ✅ | Bengali analysis |
| `indonesian` | ✅ | Indonesian analysis |
| `vietnamese` | ✅ | Vietnamese analysis |
| `thai` | ✅ | Thai analysis |
| `tamil` | ✅ | Tamil analysis |
| `telugu` | ✅ | Telugu analysis |
| `kannada` | ✅ | Kannada analysis |
| `malayalam` | ✅ | Malayalam analysis |
| `brazilian` | ✅ | Brazilian Portuguese analysis |
| `morfologik` | ✅ | Polish/Ukrainian lemmatization |
| `stempel` | ✅ | Polish Stempel stemmer |
| `auto` | ✅ | Auto language detection via whatlang |

---

## License

MIT — see [LICENSE](LICENSE).

---

<div align="center">

**[pizza.rs](https://pizza.rs)** — INFINI Pizza — The Rust Search Engine

</div>
