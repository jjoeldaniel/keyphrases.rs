use regex::Regex;
use std::collections::HashSet;

pub struct KeyPhraseExtractor {
    str: String,
    words: Vec<String>,
    content_words: Vec<String>,
    content_phrases: Vec<Vec<String>>,
}

impl KeyPhraseExtractor {
    pub fn new(str: &str) -> KeyPhraseExtractor {
        let words: Vec<String> = extract_words(&str);
        let str: String = String::from(str);
        let content_words: Vec<String> = extract_content_words(&words);
        let content_phrases: Vec<Vec<String>> = extract_content_phrases(&str);

        return KeyPhraseExtractor {
            str,
            words,
            content_words,
            content_phrases,
        };
    }

    // Returns a mutable copy of `words`
    pub fn get_words(&self) -> Vec<String> {
        let mut vec = self.words.to_vec();
        return vec;
    }

    // Returns a mutable copy of `str`
    pub fn get_str(&self) -> String {
        let mut str = self.str.clone();
        return str;
    }

    // Returns a mutable copy of `content_words`
    pub fn get_content_words(&self) -> Vec<String> {
        let mut vec = self.content_words.to_vec();
        return vec;
    }

    // Returns a mutable copy of `content_phrases`
    pub fn get_content_phrases(&self) -> Vec<Vec<String>> {
        let mut vec = self.content_phrases.to_vec();
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

/// Returns map of
// pub fn extract_content_words_frequency(content_words: &mut Vec<String>) {}

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
                    content_phrase.push(String::from(word.to_owned()));

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
