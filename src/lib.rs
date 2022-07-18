use itertools::Itertools;
use std::collections::HashMap;
use utils::{count_nested, lines_to_word_lists, merge_hashmaps_with, to_hashmap_keys};

mod utils;

pub struct Options {
    smoothing: Option<u32>,
}

// TODO: add optional debug param
pub fn unigrams(
    corpus: &Vec<String>,     // lines
    vocabulary: &Vec<String>, // optional extra vocabulary to compute n-gram probabilities for
    options: Options,
) -> HashMap<String, f32> {
    let smoothing = options.smoothing.unwrap_or(0);

    let word_lists = lines_to_word_lists(corpus);

    // create initial 0 counts of extra vocabulary
    // since there's no guarantee that the extra vocabulary
    // is present in the corpus
    let initial_word_counts: HashMap<String, u32> = to_hashmap_keys(vocabulary, |_| 0);

    // add the actual counts from corpus
    let word_counts = count_nested(&word_lists);

    let total_counts = merge_hashmaps_with(word_counts, initial_word_counts, |l, r| l + r);

    let total_words: u32 = total_counts.values().sum::<u32>();
    let vocabulary_size = total_counts.keys().count() as u32;

    let probabilities = total_counts
        .iter()
        .map(|(word, &count)| {
            let gram = word.clone();

            let unrounded_probability =
                (count + smoothing) as f32 / (total_words + (smoothing * vocabulary_size)) as f32;
            let rounded_probability = (unrounded_probability * 100.0).round() / 100.0;

            (gram, rounded_probability)
        })
        .collect();

    probabilities
}

pub fn bigrams(
    corpus: &Vec<String>,               // lines
    vocabulary: &Vec<(String, String)>, // optional extra vocabulary to compute n-gram probabilities for
    options: Options,
) -> HashMap<(String, String), f32> {
    let smoothing = options.smoothing.unwrap_or(0);

    let word_lists: Vec<Vec<String>> = lines_to_word_lists(corpus);

    let initial_biword_counts = to_hashmap_keys(vocabulary, |_| 0);

    let word_counts = count_nested(&word_lists);

    let (mut vocabulary_as_list, right): (Vec<_>, Vec<_>) = vocabulary.iter().cloned().unzip();

    vocabulary_as_list.extend(right);

    let deduped_voc = vocabulary_as_list
        .iter()
        .filter(|word| !word_counts.contains_key(&word.to_string()))
        .collect_vec();

    let vocabulary_size = word_counts.keys().count() as u32 + deduped_voc.len() as u32;

    let biword_lists: Vec<Vec<(String, String)>> = word_lists
        .iter()
        .map(|words| {
            words
                .iter()
                .tuple_windows()
                .map(|(first_word, second_word)| (first_word.to_string(), second_word.to_string()))
                .collect_vec()
        })
        .collect_vec();

    let biword_counts = count_nested(&biword_lists);

    let total_counts = merge_hashmaps_with(biword_counts, initial_biword_counts, |l, r| l + r);

    let probabilities = total_counts
        .iter()
        .map(|(words, &biword_count)| {
            let gram = words.clone();
            let probability: f32 = {
                let first_word = words.0.clone();
                let first_word_count = *word_counts
                    .get(&first_word)
                    .expect("Should not reach this state");

                let unrounded_probability =
                    (biword_count + smoothing) as f32 / (first_word_count + vocabulary_size) as f32;
                let rounded_probability = (unrounded_probability * 100.0).round() / 100.0;

                rounded_probability
            };

            (gram, probability)
        })
        .collect();

    probabilities
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::*;

    fn get_test_corpus_1() -> Vec<String> {
        let corpus: Vec<&str> = vec![
            "chicago is",
            "chicago is",
            "is cold",
            "is cold",
            "is cold",
            "is cold",
            "chicago",
            "chicago",
            "is",
            "is",
            "cold",
            "cold",
        ];

        return corpus.iter().map(|s| s.to_string()).collect_vec();
    }

    #[test]
    fn test_unigrams() {
        let corpus: Vec<String> = get_test_corpus_1();

        let vocabulary = vec!["hot".to_string()];

        let mut expected_unigrams: HashMap<String, f32> = HashMap::new();
        expected_unigrams.insert("chicago".to_string(), 0.22);
        expected_unigrams.insert("is".to_string(), 0.44);
        expected_unigrams.insert("cold".to_string(), 0.33);
        expected_unigrams.insert("hot".to_string(), 0.00);

        let actual_unigrams = crate::unigrams(&corpus, &vocabulary, Options { smoothing: None });

        assert_eq!(expected_unigrams, actual_unigrams);
    }

    #[test]
    fn test_unigrams_with_laplace_smoothing() {
        let corpus: Vec<String> = get_test_corpus_1();

        let vocabulary = vec!["hot".to_string()];

        let mut expected_unigrams: HashMap<String, f32> = HashMap::new();
        expected_unigrams.insert("chicago".to_string(), 0.23);
        expected_unigrams.insert("is".to_string(), 0.41);
        expected_unigrams.insert("cold".to_string(), 0.32);
        expected_unigrams.insert("hot".to_string(), 0.05);

        let actual_unigrams = crate::unigrams(&corpus, &vocabulary, Options { smoothing: Some(1) });

        assert_eq!(expected_unigrams, actual_unigrams);
    }

    #[test]
    fn test_bigrams() {
        let corpus: Vec<String> = get_test_corpus_1();

        let vocabulary = vec![("is".to_string(), "hot".to_string())];

        let mut expected_bigrams: HashMap<(String, String), f32> = HashMap::new();
        expected_bigrams.insert(("chicago".to_string(), "is".to_string()), 0.50);
        expected_bigrams.insert(("is".to_string(), "cold".to_string()), 0.50);
        expected_bigrams.insert(("is".to_string(), "hot".to_string()), 0.00);

        let actual_bigrams = bigrams(&corpus, &vocabulary, Options { smoothing: None });

        assert_eq!(expected_bigrams, actual_bigrams);
    }

    #[test]
    fn test_bigrams_with_laplace_smoothing() {
        let corpus: Vec<String> = get_test_corpus_1();

        let vocabulary = vec![("is".to_string(), "hot".to_string())];

        let mut expected_bigrams: HashMap<(String, String), f32> = HashMap::new();
        expected_bigrams.insert(("chicago".to_string(), "is".to_string()), 0.38);
        expected_bigrams.insert(("is".to_string(), "cold".to_string()), 0.42);
        expected_bigrams.insert(("is".to_string(), "hot".to_string()), 0.08);

        let actual_bigrams = bigrams(&corpus, &vocabulary, Options { smoothing: Some(1) });

        assert_eq!(expected_bigrams, actual_bigrams);
    }
}
