# keyphrases.rs

[![crates.io link](https://shields.io/crates/v/keyphrases)](https://crates.io/crates/keyphrases) [![documentation](https://img.shields.io/docsrs/keyphrases)](https://docs.rs/keyphrases/) ![https://github.com/jjoeldaniel/keyphrases.rs/actions/workflows/ci.yml](https://github.com/jjoeldaniel/keyphrases.rs/actions/workflows/ci.yml/badge.svg?main) [![downloads](https://img.shields.io/crates/d/keyphrases)](#keyphrasesrs) [![license](https://img.shields.io/crates/l/keyphrases)](https://github.com/jjoeldaniel/keyphrases.rs/blob/main/LICENSE)

keyphrases.rs is a Rapid Automatic Keyword Extraction (RAKE) algorithm implementation in Rust.

- [keyphrases.rs](#keyphrasesrs)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Documentation](#documentation)
  - [License](#license)

## Installation

To use keyphrases.rs in your Rust project, add the following line to your Cargo.toml file:

```toml
[dependencies]
keyphrases = "0.3.1"
```

## Usage

1. Create a new instance of `KeyPhraseExtractor` by passing the string you want to extract key phrases from:

   ```rust
   let text = "This is the text to extract key phrases from.";
   let extractor = KeyPhraseExtractor::new(text);
   ```

2. Call the desired methods on the `extractor` instance to extract the relevant information:

   ```rust
    let keywords = extractor.get_keywords();
    let word_freq = extractor.get_word_freq();
    let word_deg = extractor.get_word_deg();
    let content_words = extractor.get_content_words();
    let content_phrases = extractor.get_content_phrases();
   ```

   Each method returns the relevant information as described in the function docs below.

## [Documentation](https://docs.rs/keyphrases/latest/keyphrases/struct.KeyPhraseExtractor.html)

## License

keyphrases.rs is licensed under the WTFPL License. See the [LICENSE](https://github.com/jjoeldaniel/keyphrases.rs/blob/main/LICENSE) file for more details.
