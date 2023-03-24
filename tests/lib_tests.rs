mod tests {
    use keyphrases::KeywordExtractor;

    #[test]
    fn test_extract() {
        let test_string: &str = "Feature extraction is not that complex. There are many algorithms available that can help you with feature extraction. Rapid Automatic Keyword Extraction is one of those.";
        let kp = KeywordExtractor::new(test_string);

        assert!(!kp.get_words().is_empty());
        assert!(!kp.get_content_words().is_empty());
        assert!(!kp.get_content_phrases().is_empty());
        assert!(!kp.get_str().is_empty());
    }
}
