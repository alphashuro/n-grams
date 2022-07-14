use std::{
    fs::File,
    io::{LineWriter, Write},
};

fn main() {
    // convert from xml to line separated
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage:\n\tcargo run -- [path] [tag_name]");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let tag_name = &args[2];

    let text = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let doc = roxmltree::Document::parse(&text).expect("Failed to parse document");
    let words = doc
        .descendants()
        .filter(|n| n.tag_name().name() == tag_name)
        .filter_map(|n| n.text());

    let target_file_path = "words.txt";
    let target_file = File::create(&target_file_path).expect("Failed to open target file");
    let mut target_file = LineWriter::new(target_file);

    for line in words {
        target_file
            .write(line.as_bytes())
            .expect("Failed to write to target file");
    }
}
