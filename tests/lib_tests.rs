mod tests {
    use keyphrases;

    #[test]
    fn test_keywords() {
        let test_string: &str = "Feature extraction is not that complex. There are many algorithms available that can help you with feature extraction. Rapid Automatic Keyword Extraction is one of those.";
        let mut words: Vec<String> = keyphrases::extract_words(test_string);
        let content_phrases: Vec<Vec<String>> = keyphrases::extract_content_phrases(&mut words);

        for phrase in &content_phrases {
            println!("{:?}", phrase);
        }

        assert!(false);
    }
}
