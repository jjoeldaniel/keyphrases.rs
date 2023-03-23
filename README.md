# mapwords.rs

mapwords.rs is a Rust library that provides functionality for collecting keywords from a given string or file and then sorts and prints them based on their frequency. It ignores stop words and only returns the most frequently occurring keywords.

## Installation

To use mapwords.rs in your Rust project, add the following line to your Cargo.toml file:

```toml
[dependencies]
mapwords = "0.1.3"
```

## Usage

First, import the `MapWordsString` or `MapWordsFile` struct from the mapwords crate:

```rust
use mapwords::MapWordsString;
use mapwords::MapWordsFile;
```

Then, create a new instance of either struct by calling the new function with either a string or file path and a number representing the number of top keywords to return:

```rust
let mut map_words = MapWordsString::new("This is a test string".to_string(), 2);
```

You can then get the keywords by calling the `get_keywords()` function.
This returns a `Vec<(&u16, &String)>` if you would like to print it out yourself

```rust
let keywords = map_words.get_keywords();
```

Finally, you can print the top keywords by calling the `print_keywords()` function:

```rust
map_words.print_keywords();
```

---

### License

mapwords.rs is licensed under the WTFPL License. See the LICENSE file for more details.
