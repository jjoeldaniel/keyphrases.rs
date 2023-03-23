use std::collections::HashMap;
use std::collections::HashSet;

/// Collects keywords from String by frequency, ignoring stopwords
pub struct MapWordsString {
    str: String,
    stop_words: HashSet<String>,
    map: HashMap<String, u16>,
    top_n: u8,
}

impl MapWordsString {
    /// MapwordsString constructor
    pub fn new(str: String, top_n: u8) -> MapWordsString {
        MapWordsString {
            str,
            stop_words: HashSet::new(),
            map: HashMap::new(),
            top_n,
        }
    }

    /// Collects keywords from passed in file path
    pub fn get_keywords(&mut self) {
        self.load_stopwords();
        for k in self.str.split_whitespace() {
            // Check if key is a stopword
            //
            // Continues if true
            if self.stop_words.contains(&k.to_lowercase()) {
                continue;
            }

            // Check if key exists in map and
            // appropriately increments value
            //
            // Otherwise, insert default value of 1
            *self.map.entry(String::from(k)).or_insert(1) += 1;
        }
    }

    /// Prints top_n keywords
    pub fn print_keywords(&self) {
        let mut i: u8 = 0;
        for (k, v) in &self.map {
            if i == self.top_n {
                return;
            }

            println!("{} : {}", k, v);

            i += 1;
        }
    }

    /// Loads stopwords
    fn load_stopwords(&mut self) {
        let stopwords: &str = include_str!("stopwords.txt");

        for line in stopwords.lines() {
            self.stop_words.insert(String::from(line));
        }
    }
}
