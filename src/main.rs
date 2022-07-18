use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // convert from xml to line separated
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tcargo run -- [path]");
        std::process::exit(1);
    }

    let path = &args[1];

    let file = File::open(path).expect("Failed to read input file");
    let reader = BufReader::new(file);

    // read line separated file and create index of each word and each two words
    let words_re = Regex::new(r"([\w']+)").unwrap();

    let mut unigram_counts: HashMap<String, u32> = HashMap::new();
    let mut bigram_counts: HashMap<(String, String), u32> = HashMap::new();

    for line in reader.lines() {
        match line {
            Err(error) => {
                println!("Error reading line: {}", error);
                return;
            }
            Ok(text) => {
                // TODO: split across sentences.
                let words = words_re
                    .captures_iter(&text)
                    .map(|capture| capture[1].to_lowercase());

                // calculate unigrams and bigrams
                for (word, next_word) in words.tuple_windows() {
                    let unigram_key = &word;
                    let unigram_count = *unigram_counts.get(unigram_key).unwrap_or(&0);

                    unigram_counts.insert(unigram_key.to_string(), unigram_count + 1);

                    let bigram_key = (word, next_word);
                    let bigram_count = *bigram_counts.get(&bigram_key).unwrap_or(&0);

                    bigram_counts.insert(bigram_key, bigram_count + 1);
                }
            }
        }
    }

    let mut bigram_probabilities: HashMap<(String, String), f32> = HashMap::new();

    for ((first_word, second_word), bigram_count) in bigram_counts.iter() {
        let first_word_count = unigram_counts.get(first_word).unwrap_or(&0);
        let bigram_probability = *bigram_count as f32 / *first_word_count as f32;

        bigram_probabilities.insert(
            (first_word.to_string(), second_word.to_string()),
            bigram_probability,
        );
    }

    println!("unigram probabilities: {:?}", unigram_counts);
    println!("bigrams probabilities: {:?}", bigram_probabilities);

    println!("{} unigram count", unigram_counts.len());
    println!("{} bigram count", bigram_counts.len());

    // add la-placian smoothing

    // add good-turing discounting
}
