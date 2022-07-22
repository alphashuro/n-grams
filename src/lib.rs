use itertools::Itertools;
use std::collections::HashMap;
use utils::{count_nested, lines_to_word_lists, merge_hashmaps_with, to_hashmap_keys};

mod utils;

#[derive(Default)]
pub struct Options {
    add_k_smoothing: u32,
    good_turing: bool,
}

impl Options {
    pub fn new() -> Options {
        Default::default()
    }

    pub fn with_add_k_smoothing(mut self: Self, k: u32) -> Self {
        self.add_k_smoothing = k;

        self
    }

    pub fn with_good_turing(mut self: Self, on: bool) -> Self {
        self.good_turing = on;

        self
    }
}

// TODO: add optional debug param
pub fn unigrams(
    corpus: &Vec<String>,     // lines
    vocabulary: &Vec<String>, // optional extra vocabulary to compute n-gram probabilities for
    options: Options,
) -> HashMap<String, f32> {
    let smoothing = options.add_k_smoothing;

    let word_lists = lines_to_word_lists(corpus);

    // create initial 0 counts of extra vocabulary
    // since there's no guarantee that the extra vocabulary
    // is present in the corpus
    let initial_word_counts: HashMap<String, u32> = to_hashmap_keys(vocabulary, |_| 0);

    // add the actual counts from corpus
    let word_counts = count_nested(&word_lists);
    // TODO: only calculate this when good turing is on

    let total_counts = merge_hashmaps_with(word_counts, initial_word_counts, |l, r| l + r);

    let counts_for_words =
        total_counts
            .iter()
            .fold(HashMap::<u32, u32>::new(), |mut counts, (_word, count)| {
                *counts.entry(*count).or_default() += 1;

                counts
            });
    let total_words: u32 = total_counts.values().sum::<u32>();
    let vocabulary_size = total_counts.keys().count() as u32;

    let probabilities = total_counts
        .iter()
        .map(|(word, &count)| {
            let gram = word.clone();

            // smooth count if good turing is enabled
            let c: f32 = match options.good_turing {
                false => count as f32,
                true => {
                    let c_1 = count + 1;
                    let n_1: f32 = *counts_for_words.get(&count).unwrap_or(&0) as f32;
                    let n_2 = *counts_for_words.get(&(c_1)).unwrap_or(&0) as f32;

                    match count {
                        0 => n_2,
                        _ => c_1 as f32 * (n_2 / n_1),
                    }
                }
            };

            let unrounded_probability =
                (c + smoothing as f32) / (total_words + (smoothing * vocabulary_size)) as f32;
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
    let smoothing = options.add_k_smoothing;

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
    let counts_for_bigrams =
        total_counts
            .iter()
            .fold(HashMap::<u32, u32>::new(), |mut counts, (_words, count)| {
                *counts.entry(*count).or_default() += 1;

                counts
            });

    let probabilities = total_counts
        .iter()
        .map(|(words, &count)| {
            let gram = words.clone();
            let probability: f32 = {
                let first_word = words.0.clone();
                let first_word_count = *word_counts
                    .get(&first_word)
                    .expect("Should not reach this state");

                // smooth count if good turing is enabled
                let c: f32 = match options.good_turing {
                    false => count as f32,
                    true => {
                        let c_1 = count + 1;
                        let n_1: f32 = *counts_for_bigrams.get(&count).unwrap_or(&0) as f32;
                        let n_2 = *counts_for_bigrams.get(&(c_1)).unwrap_or(&0) as f32;

                        match count {
                            0 => n_2,
                            _ => c_1 as f32 * (n_2 / n_1),
                        }
                    }
                };

                let unrounded_probability = (c + smoothing as f32) as f32
                    / (first_word_count + (vocabulary_size * smoothing)) as f32;
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

    fn to_vec_of_string(list: Vec<&str>) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect_vec()
    }

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

        to_vec_of_string(corpus)
    }

    fn get_test_corpus_2_species() -> Vec<String> {
        let corpus: Vec<&str> = vec![
            "carp",
            "carp",
            "carp",
            "carp",
            "carp",
            "carp",
            "carp",
            "carp",
            "carp",
            "carp",
            "perch",
            "perch",
            "perch",
            "whitefish",
            "whitefish",
            "trout",
            "salmon",
            "eel",
        ];

        to_vec_of_string(corpus)
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

        let actual_unigrams = crate::unigrams(&corpus, &vocabulary, Options::new());

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

        let actual_unigrams =
            crate::unigrams(&corpus, &vocabulary, Options::new().with_add_k_smoothing(1));

        assert_eq!(expected_unigrams, actual_unigrams);
    }

    #[test]
    fn test_unigrams_with_good_turing() {
        let corpus: Vec<String> = get_test_corpus_2_species();

        let vocabulary = to_vec_of_string(vec!["catfish", "bass"]);

        let actual = crate::unigrams(&corpus, &vocabulary, Options::new().with_good_turing(true));

        assert_eq!(0.04, actual["trout"]);
        assert_eq!(0.17, actual["bass"]);
    }

    #[test]
    fn test_bigrams() {
        let corpus: Vec<String> = get_test_corpus_1();

        let vocabulary = vec![("is".to_string(), "hot".to_string())];

        let mut expected_bigrams: HashMap<(String, String), f32> = HashMap::new();
        expected_bigrams.insert(("chicago".to_string(), "is".to_string()), 0.50);
        expected_bigrams.insert(("is".to_string(), "cold".to_string()), 0.50);
        expected_bigrams.insert(("is".to_string(), "hot".to_string()), 0.00);

        let actual_bigrams = bigrams(&corpus, &vocabulary, Options::new());

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

        let actual_bigrams = bigrams(&corpus, &vocabulary, Options::new().with_add_k_smoothing(1));

        assert_eq!(expected_bigrams, actual_bigrams);
    }
}
