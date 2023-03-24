mod tests {
    use keyphrases;

    #[test]
    fn test_keywords() {
        let test_string: &str = "Feature extraction is not that complex. There are many algorithms available that can help you with feature extraction. Rapid Automatic Keyword Extraction is one of those.";
        let words: Vec<String> = keyphrases::extract_words(test_string);

        for w in &words {
            println!("{}", w);
        }

        assert!(false);
    }
}
