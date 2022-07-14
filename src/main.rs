use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

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

    let mut bigrams: HashMap<(String, String), f32> = HashMap::new();

    for (words, count) in bigram_counts.iter() {
        let word_count = unigram_counts.get(&words.0).unwrap_or(&0);
        let bigram = *count as f32 / *word_count as f32;
        bigrams.insert((words.0.to_string(), words.1.to_string()), bigram);
    }

    println!("unigrams: {:?}", unigram_counts);
    println!("bigrams: {:?}", bigrams);

    println!("{} unigram count", unigram_counts.len());
    println!("{} bigram count", bigram_counts.len());
}
