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
        let content_phrases: Vec<Vec<String>> = extract_content_phrases(&words);

        return KeyPhraseExtractor {
            str,
            words,
            content_words,
            content_phrases,
        };
    }

    // Returns a reference to `words`
    pub fn get_words(&self) -> &Vec<String> {
        return &self.words;
    }

    // Returns a reference to `str`
    pub fn get_str(&self) -> &String {
        return &self.str;
    }

    // Returns a reference to `content_words`
    pub fn get_content_words(&self) -> &Vec<String> {
        return &self.content_words;
    }

    // Returns a reference to `content_phrases`
    pub fn get_content_phrases(&self) -> &Vec<Vec<String>> {
        return &self.content_phrases;
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
fn extract_content_phrases(words: &Vec<String>) -> Vec<Vec<String>> {
    // stopwords
    let stopwords: HashSet<String> = load_stopwords();

    let mut content_phrases: Vec<Vec<String>> = Vec::new();
    let mut content_words: Vec<String> = Vec::new();
    let mut content_phrase: Vec<String> = Vec::new();

    for word in words {
        // Push word to phrases vec if we hit a stop word
        // and clears the phrase vector.
        if stopwords.contains(&word.to_lowercase()) {
            content_phrases.push(content_phrase.clone());
            content_phrase.clear();
            continue;
        } else {
            // Push word to phrase and words vectors
            content_phrase.push(word.to_string());
            content_words.push(word.to_string());
        }
    }

    // Remove any empty vectors
    content_phrases.retain(|inner_vec| !inner_vec.is_empty());

    return content_phrases;
}
