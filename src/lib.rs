use regex::Regex;
use std::collections::HashSet;

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
pub fn extract_words(input: &str) -> Vec<String> {
    // regex
    let re = Regex::new(r"\b\w+\b").unwrap();

    // Mutable string vector of ALL words
    let mut content_words: Vec<String> = Vec::new();
    content_words.extend(re.find_iter(input).map(|m| m.as_str().to_string()));

    return content_words;
}

/// Returns a vector of content words
pub fn extract_content_words(words: &mut Vec<String>) -> Vec<String> {
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
pub fn extract_content_phrases(words: &mut Vec<String>) -> Vec<Vec<String>> {
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

    return content_phrases;
}
