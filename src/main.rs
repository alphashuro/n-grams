use itertools::Itertools;
use n_gram::{bigrams, unigrams, utils::line_to_words, Options};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, LineWriter, Write},
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

    let lines = reader
        .lines()
        .filter_map(|l| match l {
            Ok(line) => Some(line),
            _ => None,
        })
        .collect_vec();

    let p_unigrams = unigrams(&lines, &Vec::new(), Options::new());
    write_ps_to_csv::<String>(path, "unigrams", p_unigrams);

    let p_bigrams = bigrams(&lines, &Vec::new(), Options::new());
    write_ps_to_csv::<(String, String)>(path, "bigrams", p_bigrams);

    let p_unigrams_with_laplacian =
        unigrams(&lines, &Vec::new(), Options::new().with_add_k_smoothing(1));
    write_ps_to_csv::<String>(path, "unigrams.laplacian", p_unigrams_with_laplacian);

    let p_bigrams_with_laplacian =
        bigrams(&lines, &Vec::new(), Options::new().with_add_k_smoothing(1));
    write_ps_to_csv::<(String, String)>(path, "bigrams.laplacian", p_bigrams_with_laplacian);

    let p_unigrams_with_good_turing =
        unigrams(&lines, &Vec::new(), Options::new().with_good_turing(true));
    write_ps_to_csv::<String>(path, "unigrams.good_turing", p_unigrams_with_good_turing);

    let p_bigrams_with_good_turing =
        bigrams(&lines, &Vec::new(), Options::new().with_good_turing(true));
    write_ps_to_csv::<(String, String)>(path, "bigrams.good_turing", p_bigrams_with_good_turing);
}

fn write_ps_to_csv<K>(path: &str, prefix: &str, hashmap: HashMap<K, f32>)
where
    K: std::fmt::Debug,
{
    let target_file_path = path.to_owned() + "." + prefix + ".csv";
    let target_file = File::create(&target_file_path).expect("Failed to open target file");
    let mut target_file = LineWriter::new(target_file);

    target_file
        .write_all("w,p(w)\n".as_bytes())
        .expect("Failed to write to target file");

    for (gram, p) in hashmap {
        target_file
            .write_all(format!("{:?},{}\n", gram, p).as_bytes())
            .expect("Failed to write to target file");
    }
}
