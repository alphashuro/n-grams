#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{count_words, lines_to_word_lists, merge_hashmaps_with, Word, WordLists};

    #[test]
    fn test_lines_to_word_lists() {
        let lines = vec!["chicago is cold", "africa is hot"];

        let expected = vec![vec!["chicago", "is", "cold"], vec!["africa", "is", "hot"]];
        let actual = lines_to_word_lists(&lines);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_count_words() {
        let word_list: Vec<String> = (vec!["chicago", "is", "cold", "africa", "is", "hot"])
            .iter()
            .map(|word| word.to_string())
            .collect();

        let mut expected_counts: HashMap<Word, u32> = HashMap::new();
        expected_counts.insert("chicago".to_string(), 1);
        expected_counts.insert("is".to_string(), 2);
        expected_counts.insert("cold".to_string(), 1);
        expected_counts.insert("africa".to_string(), 1);
        expected_counts.insert("hot".to_string(), 1);

        let actual_counts = count_words(&word_list);

        assert_eq!(expected_counts, actual_counts);
    }

    #[test]
    fn test_merge_hashmaps_with() {
        let mut left = HashMap::new();
        left.insert("one", 1);
        left.insert("two", 1);
        left.insert("three", 2);

        let mut right = HashMap::new();
        right.insert("two", 1);
        right.insert("three", 1);
        right.insert("four", 4);

        let mut expected = HashMap::new();
        expected.insert("one", 1);
        expected.insert("two", 2);
        expected.insert("three", 3);
        expected.insert("four", 4);

        let actual =
            merge_hashmaps_with(left, right, |(key, left), (_, right)| (key, left + right));

        assert_eq!(expected, actual);
    }

    #[test]
    fn uni_grams() {
        let corpus = vec![
            "chicago is cold",
            "chicago is cold",
            "is cold",
            "is cold",
            "is cold",
            "is cold",
            "chicago is",
            "chicago is",
        ];
        let word_lists = lines_to_word_lists(&corpus);

        let mut expected_unigrams: HashMap<Vec<String>, f32> = HashMap::new();
        expected_unigrams.insert(vec!["chicago".to_string()], 0.22);
        expected_unigrams.insert(vec!["is".to_string()], 0.44);
        expected_unigrams.insert(vec!["cold".to_string()], 0.33);
        expected_unigrams.insert(vec!["hot".to_string()], 0.00);

        let actual_unigrams = crate::n_grams(&word_lists, &vec!["hot".to_string()], 1);

        assert_eq!(expected_unigrams, actual_unigrams);
    }

    #[test]
    fn bi_grams() {
        let corpus = vec![
            "chicago is cold",
            "chicago is cold",
            "is cold",
            "is cold",
            "is cold",
            "is cold",
            "chicago is",
            "chicago is",
        ];
        let word_lists = lines_to_word_lists(&corpus);

        let mut expected_bigrams: HashMap<Vec<String>, f32> = HashMap::new();
        expected_bigrams.insert(vec!["chicago".to_string(), "is".to_string()], 0.50);
        expected_bigrams.insert(vec!["is".to_string(), "cold".to_string()], 0.50);
        expected_bigrams.insert(vec!["is".to_string(), "hot".to_string()], 0.00);

        let actual_bigrams =
            crate::n_grams(&word_lists, &vec!["is".to_string(), "hot".to_string()], 2);

        assert_eq!(expected_bigrams, actual_bigrams);
    }
}
