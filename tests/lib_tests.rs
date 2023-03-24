mod tests {
    use mapwords::MapWordsFile;
    use mapwords::MapWordsString;

    fn compare_vectors(vec1: Vec<(u16, String)>, vec2: Vec<(u16, String)>) -> bool {
        for i in 0..(vec2.len() - 1) {
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
        let mut map = MapWordsString::new(test_data, 4);

        // expected values
        let mut test_vec: Vec<(u16, String)> = Vec::new();
        let str1: String = String::from("judgement");
        let str2: String = String::from("said");
        let str3: String = String::from("might");
        let str4: String = String::from("man");
        test_vec.push((5, str1));
        test_vec.push((2, str2));
        test_vec.push((2, str3));
        test_vec.push((2, str4));

        map.get_keywords();
        map.print_keywords();

        // compare vector equality
        assert!(compare_vectors(map.get_keywords(), test_vec));
    }

    #[test]
    fn test_file_keywords() {
        // collect keywords
        let path: String = String::from("./tests/data.txt");
        let mut map: MapWordsFile = MapWordsFile::new(path, 4);

        // expected values
        let mut test_vec: Vec<(u16, String)> = Vec::new();
        let str1: String = String::from("usurpations");
        let str2: String = String::from("government");
        let str3: String = String::from("right");
        let str4: String = String::from("rights");
        test_vec.push((4, str1));
        test_vec.push((4, str2));
        test_vec.push((3, str3));
        test_vec.push((2, str4));

        map.get_keywords();
        map.print_keywords();

        // compare vector equality
        assert!(compare_vectors(map.get_keywords(), test_vec));
    }
}
