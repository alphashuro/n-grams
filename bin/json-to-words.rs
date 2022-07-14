use std::{
    fs::File,
    io::{LineWriter, Write},
};

// convert from the JSON format used on https://github.com/nlp-compromise/nlp-corpus
// to line separated strings
fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tcargo run -- [path]");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let text = std::fs::read_to_string(file_path).expect("Failed to read input file");
    let lines: Vec<String> = serde_json::from_str(&text).expect("Failed to read file as json");

    let target_file_path = "words2.txt";
    let target_file = File::create(&target_file_path).expect("Failed to open target file");
    let mut target_file = LineWriter::new(target_file);

    for line in lines {
        target_file
            .write(line.as_bytes())
            .expect("Failed to write to target file");
    }
}
