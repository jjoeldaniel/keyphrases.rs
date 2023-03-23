mod tests {

    fn compare_vectors(vec1: Vec<(u16, String)>, vec2: Vec<(u16, String)>) -> bool {
        if vec1.len() != vec2.len() {
            return false;
        }

        for i in 0..vec1.len() {
            if vec2[i] != vec1[i] {
                return false;
            }
        }

        return true;
    }

    #[test]
    fn test_keywords() {
        // collect keywords
        let test_data = String::from("For although a judgement judgement judgement man is judged by his actions, by what he has said judgement and done, a man judges himself by what he is willing to do, by what he might have said, or might have done – a judgement that is necessarily hampered, bot only by the scope and limits of his imagination, but by the ever-changing measure of his doubt and self-esteem.");
        let mut map = mapwords::MapWordsString::new(test_data, 5);
        map.collect_keywords();

        // expected values
        let mut test_vec: Vec<(u16, String)> = Vec::new();
        let str1: String = String::from("judgement");
        let str2: String = String::from("might");
        let str3: String = String::from("man");
        let str4: String = String::from("willing");
        let str5: String = String::from("self-esteem.");
        test_vec.push((5, str1));
        test_vec.push((2, str2));
        test_vec.push((2, str3));
        test_vec.push((1, str4));
        test_vec.push((1, str5));

        // compare vector equality
        assert!(compare_vectors(map.get_keywords(), test_vec));
    }
}
