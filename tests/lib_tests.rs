mod tests {
    use keyphrases::KeyPhraseExtractor;

    #[test]
    fn test_extract() {
        let test_string: &str = "Feature extraction is not that complex. There are many algorithms available that can help you with feature extraction. Rapid Automatic Keyword Extraction is one of those.";
        let kp = KeyPhraseExtractor::new(test_string, 3, 2);

        for phrase in kp.get_keywords() {
            println!("{:?}", phrase);
        }
        // assert!(false);

        assert!(!kp.get_keywords().is_empty());
        assert!(!kp.get_content_phrases().is_empty());
        assert!(!kp.get_content_words().is_empty());
        assert!(!kp.get_word_deg().is_empty());
        assert!(!kp.get_word_freq().is_empty());
    }
}
