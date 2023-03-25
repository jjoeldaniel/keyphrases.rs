use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct KeyPhraseExtractor {
    str: String,
    words: Vec<String>,
    content_words: Vec<String>,
    content_phrases: Vec<Vec<String>>,
    word_freq: HashMap<String, usize>,
    word_deg: HashMap<String, usize>,
}

impl KeyPhraseExtractor {
    /// Initializes word frequency and degree maps
    fn initialize_maps(
        content_phrases: Vec<Vec<String>>,
        word_freq: &mut HashMap<String, usize>,
        word_deg: &mut HashMap<String, usize>,
    ) {
        for phrase in content_phrases {
            let mut word_added_len: HashSet<String> = HashSet::new();
            for word in &phrase {
                // Set the word deg value of the word to the length of the phrase
                // Note: This is assigning, so its fine if it runs over duplicates
                if !word_added_len.contains(word) {
                    if word_deg.contains_key(word) {
                        word_deg.insert(word.clone(), phrase.len() + word_deg.get(word).unwrap());
                    } else {
                        word_deg.insert(word.clone(), phrase.len());
                    }
                    word_added_len.insert(word.clone());
                }

                // if word is stored and is some value
                // increment currently stored value by 1
                if word_freq.contains_key(word) {
                    let curr_freq = word_freq.get(word);
                    word_freq.insert(word.clone(), *curr_freq.unwrap_or(&1) + 1);
                } else {
                    word_freq.insert(word.clone(), 1);
                }
            }
        }
    }

    /// Constructor
    pub fn new(str: &str) -> KeyPhraseExtractor {
        let words: Vec<String> = extract_words(&str);
        let str: String = String::from(str);
        let content_words: Vec<String> = extract_content_words(&words);
        let content_phrases: Vec<Vec<String>> = extract_content_phrases(&str);

        // maps
        let mut word_freq: HashMap<String, usize> = HashMap::new();
        let mut word_deg: HashMap<String, usize> = HashMap::new();

        KeyPhraseExtractor::initialize_maps(content_phrases.clone(), &mut word_freq, &mut word_deg);

        return KeyPhraseExtractor {
            str,
            words,
            content_words,
            content_phrases,
            word_freq,
            word_deg,
        };
    }

    /// Returns a map of a degree score and its respective phrase
    pub fn get_phrase_degree_scores(&self) -> HashMap<String, String> {
        // Map of degree scores with their associated phrase
        let mut phrase_degree_score: HashMap<String, String> = HashMap::new();

        // Loop through each phrase
        for phrase in self.get_content_phrases() {
            let mut cum_score: f32 = 0.0;

            // Get cumulative score
            for word in &phrase {
                cum_score += *self.get_word_deg().get(word).unwrap() as f32
                    / *self.get_word_freq().get(word).unwrap() as f32;
            }

            // Insert cumulative score and phrase into list
            phrase_degree_score.insert(cum_score.to_string(), phrase.clone().join(" "));
        }

        return phrase_degree_score;
    }

    // Returns a copy of `word_freq`
    pub fn get_word_freq(&self) -> HashMap<String, usize> {
        let freq = self.word_freq.clone();
        return freq;
    }

    /// Returns a copy of `word_deg`
    pub fn get_word_deg(&self) -> HashMap<String, usize> {
        let deg = self.word_deg.clone();
        return deg;
    }

    /// Returns a copy of `words`
    pub fn get_words(&self) -> Vec<String> {
        let vec = self.words.to_vec();
        return vec;
    }

    /// Returns a copy of `str`
    pub fn get_str(&self) -> String {
        let str = self.str.clone();
        return str;
    }

    /// Returns a copy of `content_words`
    pub fn get_content_words(&self) -> Vec<String> {
        let vec = self.content_words.to_vec();
        return vec;
    }

    /// Returns a copy of `content_phrases`
    pub fn get_content_phrases(&self) -> Vec<Vec<String>> {
        let vec = self.content_phrases.to_vec();
        return vec;
    }
}

/// Reads in stopwords and returns a HashSet of Strings
fn load_stopwords() -> HashSet<String> {
    let stopwords: &str = include_str!("stopwords.txt");
    let mut stop_words: HashSet<String> = HashSet::new();

    // Insert each stopword into struct HashSet
    for line in stopwords.lines() {
        stop_words.insert(String::from(line));
    }

    return stop_words;
}

/// Returns a vector of all words
fn extract_words(input: &str) -> Vec<String> {
    // regex
    let re = Regex::new(r"\b\w+\b").unwrap();

    // Mutable string vector of ALL words
    let mut content_words: Vec<String> = Vec::new();
    content_words.extend(re.find_iter(input).map(|m| m.as_str().to_string()));

    return content_words;
}

/// Returns a vector of content words
fn extract_content_words(words: &Vec<String>) -> Vec<String> {
    // stopwords
    let stopwords: HashSet<String> = load_stopwords();

    let mut content_words: Vec<String> = Vec::new();

    for word in words {
        // Push word to phrases vec if we hit a stop word
        // and clears the phrase vector.
        if stopwords.contains(&word.to_lowercase()) {
            continue;
        } else {
            content_words.push(word.to_string());
        }
    }

    return content_words;
}

/// Returns a vector of all content phrases
fn extract_content_phrases(input: &str) -> Vec<Vec<String>> {
    // stopwords
    let stopwords: HashSet<String> = load_stopwords();

    let mut content_phrases: Vec<Vec<String>> = Vec::new();

    // Loop over sentences and then commas
    for sentence in input.split('.') {
        for phrase in sentence.split(',') {
            let mut content_phrase: Vec<String> = Vec::new();
            let words: Vec<&str> = phrase.split_ascii_whitespace().collect();
            let total_words = words.len();

            for (i, word) in words.iter().enumerate() {
                // If word is a stopword, push the phrase, clear, and
                // move to the next split
                if stopwords.contains(&word.to_ascii_lowercase()) {
                    content_phrases.push(content_phrase.clone());
                    content_phrase.clear();
                    continue;
                } else {
                    // Append the current word to the phrase
                    content_phrase.push(String::from(word.to_owned()).to_ascii_lowercase());

                    // If current word is the last word in the phrase,
                    // push the phrase and clear
                    if i == total_words - 1 {
                        content_phrases.push(content_phrase.clone());
                        content_phrase.clear();
                    }
                }
            }
        }
    }

    // Remove any empty vectors
    content_phrases.retain(|inner_vec| !inner_vec.is_empty());

    return content_phrases;
}
