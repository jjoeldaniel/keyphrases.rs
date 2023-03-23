pub mod map_words {

    use std::collections::HashSet;
    use std::{collections::HashMap, fs};

    pub struct MapwordsFile {
        str: String,
        stop_words: HashSet<String>,
        map: HashMap<String, u16>,
        top_n: u8,
    }

    impl MapwordsFile {
        /// MapwordsString constructor
        pub fn new(str: String, stop_words: HashSet<String>, top_n: u8) -> MapwordsFile {
            MapwordsFile {
                str,
                stop_words,
                map: HashMap::new(),
                top_n,
            }
        }

        /// Collects keywords from passed in file path
        pub fn get_keywords(&mut self) {
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
    }

    /// Reads a .txt file and returns a HashSet of all strings
    pub fn txt_to_set(file_path: String) -> HashSet<String> {
        let content = fs::read_to_string(file_path)
            .expect("Failed to read file path, maybe the file doesn't exist?");

        let mut set: HashSet<String> = HashSet::new();

        for line in content.lines() {
            set.insert(String::from(line));
        }

        return set;
    }
}
fn main() {}
