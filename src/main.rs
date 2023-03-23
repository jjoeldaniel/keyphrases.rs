use std::collections::HashSet;
use std::{collections::HashMap, fs};

struct MapwordsString {
    str: String,
    stop_words: HashSet<String>,
    map: HashMap<String, u16>,
    top_n: u8,
}

impl MapwordsString {
    fn new(str: String, stop_words: HashSet<String>, top_n: u8) -> MapwordsString {
        MapwordsString {
            str,
            stop_words,
            map: HashMap::new(),
            top_n,
        }
    }
    fn get_keywords(&mut self) {
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

    fn print_keywords(&self) {
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

fn txt_to_set(file_path: String) -> HashSet<String> {
    let content = fs::read_to_string(file_path)
        .expect("Failed to read file path, maybe the file doesn't exist?");

    let mut set: HashSet<String> = HashSet::new();

    for line in content.lines() {
        set.insert(String::from(line));
    }

    return set;
}

fn main() {
    let stopwords = txt_to_set(String::from("stopwords-english.txt"));
    let test_data = String::from("I glance in the mirror. The man I see is not so much me as my father. When did he show up? There is no soap; I rub hand sanitizer into my face–it burns. I nearly drown myself in the sink trying to rinse it off.");
    let mut map = MapwordsString::new(test_data, stopwords, 5);

    map.get_keywords();
    map.print_keywords();
}
