use regex::Regex;
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
    pub fn collect_keywords(&mut self) {
        self.load_stopwords();
        let re = Regex::new("(\\w+)").unwrap();

        for k in self.str.trim().split_whitespace() {
            // Check if key is a stopword or fails to match regex
            //
            // Continues if true
            if !re.is_match(&k) || self.stop_words.contains(&k.to_lowercase()) {
                continue;
            }

            let str: String = String::from(k);

            if self.map.get(&str).is_some() {
                // Update value
                let stored_value: &u16 = self.map.get(&str).unwrap();
                self.map.insert(str, stored_value + 1);
            } else {
                // Insert value
                self.map.insert(str, 1);
            }
        }
    }

    /// Returns a copy of sorted keywords
    pub fn get_keywords(&self) -> Vec<(&u16, &String)> {
        let mut sorted_vector: Vec<(&u16, &String)> = Vec::new();
        for (k, v) in self.map.iter() {
            sorted_vector.push((v, k));
        }

        sorted_vector.sort_by(|a, b| b.cmp(a));

        let mut sized_vector: Vec<(&u16, &String)> = Vec::new();
        let mut i: u8 = 0;
        for tup in sorted_vector {
            if i == self.top_n {
                break;
            }

            sized_vector.push((tup.0, tup.1));

            i += 1;
        }

        return sized_vector;
    }

    /// Prints top_n keywords
    pub fn print_keywords(&self) {
        let sorted_vector = self.get_keywords();

        for tup in sorted_vector {
            println!("{} : {}", tup.0, tup.1);
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
