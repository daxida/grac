// For profiling and testing.

use grac::syllabify_el_mode;
use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn run(text: &str) {
    let words: Vec<&str> = text.split_whitespace().collect();
    println!(
        "Content size: {}. Number of words: {}",
        text.len(),
        words.len()
    );

    let times = 100;
    for _ in 0..times {
        let _syls: Vec<_> = words
            .iter()
            .map(|cword| {
                let syllables = syllabify_el_mode(cword, grac::Merge::Never);
                syllables
            })
            .collect();
    }
}

fn main() {
    let file_path = "tests/fixtures/dump.txt";
    match read_file(file_path) {
        Ok(content) => run(&content),
        Err(e) => eprintln!("Error reading file {file_path}: {e}"),
    }
}
