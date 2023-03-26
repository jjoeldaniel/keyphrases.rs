use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

/// `KeyPhraseExtractor` is a struct that allows for keyphrase and keyword extraction
/// utilizing Rapid Automatic Keyword Extraction (RAKE).
///
/// Properties:
///
/// * `content_words`: A vector of all the words in the text.
/// * `content_phrases`: A vector of vectors of strings. Each vector of strings represents a phrase.
/// * `word_freq`: A hashmap that maps each word to its frequency in the document.
/// * `word_deg`: A HashMap that maps each word to its degree. The degree of a word is the number of
/// words that are one edit distance away from it.
pub struct KeyPhraseExtractor {
    content_words: Vec<String>,
    content_phrases: Vec<Vec<String>>,
    word_freq: HashMap<String, usize>,
    word_deg: HashMap<String, usize>,
}

impl KeyPhraseExtractor {
    /// Initializing the word_freq and word_deg maps.
    fn initialize_maps(
        content_phrases: Vec<Vec<String>>,
        word_freq: &mut HashMap<String, usize>,
        word_deg: &mut HashMap<String, usize>,
    ) {
        for phrase in content_phrases {
            let mut word_added_len: HashSet<String> = HashSet::new();
            for word in &phrase {
                // This is checking if the word has already been added to the word_deg map. If it has,
                // it will add the length of the phrase to the current value. If it hasn't, it will add
                // the length of the phrase to the map.
                if !word_added_len.contains(word) {
                    if word_deg.contains_key(word) {
                        word_deg.insert(word.clone(), phrase.len() + word_deg.get(word).unwrap());
                    } else {
                        word_deg.insert(word.clone(), phrase.len());
                    }
                    word_added_len.insert(word.clone());
                }

                // This is checking if the word has already been added to the word_freq map. If stored,
                // increment the current value. Otherwise, insert the value with a default of 1
                if word_freq.contains_key(word) {
                    let curr_freq = word_freq.get(word);
                    word_freq.insert(word.clone(), *curr_freq.unwrap_or(&1) + 1);
                } else {
                    word_freq.insert(word.clone(), 1);
                }
            }
        }
    }

    /// Constructs a new KeyPhraseExtractor instace
    ///
    /// Arguments:
    ///
    /// * `str`: The string to extract key phrases from.
    ///
    /// Returns:
    ///
    /// A KeyPhraseExtractor struct
    pub fn new(str: &str) -> KeyPhraseExtractor {
        let words: Vec<String> = extract_words(&str);
        let content_words: Vec<String> = extract_content_words(&words);
        let content_phrases: Vec<Vec<String>> = extract_content_phrases(&String::from(str));

        // maps
        let mut word_freq: HashMap<String, usize> = HashMap::new();
        let mut word_deg: HashMap<String, usize> = HashMap::new();

        KeyPhraseExtractor::initialize_maps(content_phrases.clone(), &mut word_freq, &mut word_deg);

        return KeyPhraseExtractor {
            content_words,
            content_phrases,
            word_freq,
            word_deg,
        };
    }

    /// Returns a vector of tuples containing a cumulative degree score
    /// and its respective phrase
    pub fn get_keywords(&self) -> Vec<(f32, String)> {
        let mut phrase_degree_score: Vec<(f32, String)> = Vec::new();

        // Loop through each phrase
        for phrase in self.get_content_phrases() {
            let mut cum_score: f32 = 0.0;

            // Get cumulative score
            for word in &phrase {
                cum_score += *self.get_word_deg().get(word).unwrap() as f32
                    / *self.get_word_freq().get(word).unwrap() as f32;
            }

            // Insert cumulative score and phrase into list
            phrase_degree_score.push((cum_score, phrase.clone().join(" ")));
        }

        phrase_degree_score.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        return phrase_degree_score;
    }

    /// Returns a copy of the word_freq HashMap.
    ///
    /// Returns:
    ///
    /// A clone of the word_freq HashMap.
    pub fn get_word_freq(&self) -> HashMap<String, usize> {
        let freq = self.word_freq.clone();
        return freq;
    }

    /// Returns a copy of the word_deg HashMap.
    ///
    /// Returns:
    ///
    /// A clone of the word_deg HashMap.
    pub fn get_word_deg(&self) -> HashMap<String, usize> {
        let deg = self.word_deg.clone();
        return deg;
    }

    /// Returns a vector of all content words.
    ///
    /// Returns:
    ///
    /// A vector of strings.
    pub fn get_content_words(&self) -> Vec<String> {
        let vec = self.content_words.to_vec();
        return vec;
    }

    /// Returns a vector of of all content phrases
    ///
    /// Content phrases contain content word vectors.
    ///
    /// Returns:
    ///
    /// A vector of vectors of strings.
    pub fn get_content_phrases(&self) -> Vec<Vec<String>> {
        let vec = self.content_phrases.to_vec();
        return vec;
    }
}

/// Reads in the stopwords.txt file, splits it into lines, and inserts each line into a HashSet
///
/// Returns:
///
/// A HashSet of Strings
fn load_stopwords() -> HashSet<String> {
    let stopwords: &str = include_str!("stopwords.txt");
    let mut stop_words: HashSet<String> = HashSet::new();

    // Insert each stopword into struct HashSet
    for line in stopwords.lines() {
        stop_words.insert(String::from(line));
    }

    return stop_words;
}

/// Extracts all individual words of a input &str and returns
/// all words in vector form.
///
/// Arguments:
///
/// * `input`: &str - The input string to extract words from
///
/// Returns:
///
/// A vector of strings
fn extract_words(input: &str) -> Vec<String> {
    // regex
    let re = Regex::new(r"\b\w+\b").unwrap();

    // Mutable string vector of ALL words
    let mut content_words: Vec<String> = Vec::new();
    content_words.extend(re.find_iter(input).map(|m| m.as_str().to_string()));

    return content_words;
}

/// Takes a vector of all words passed into KeyPhraseExtractor constructor,
/// and returns a vector of content words (words that are not stopwords)
///
/// Arguments:
///
/// * `words`: A vector of strings, where each string is a word in the text.
///
/// Returns:
///
/// A vector of strings.
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

/// Splits an input string into sentences, then splits each sentence into phrases, then splits each
/// phrase into words, then checks each word against a list of stopwords, and finally returns a vector
/// of vectors of strings
///
/// This constructs a RAKE keyphrase.
///
/// Arguments:
///
/// * `input`: The input string to extract phrases from
///
/// Returns:
///
/// A vector of vectors of strings.
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
