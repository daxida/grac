use grac::{syllabify_gr, to_monotonic};
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

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

    let now = Instant::now();
    let _syls: Vec<Vec<_>> = words.iter().map(|cword| syllabify_gr(cword)).collect();
    println!("Syllabification took: {:?}", now.elapsed());

    let now = Instant::now();
    let _mono = to_monotonic(text);
    println!("To monotonic took:    {:?}", now.elapsed());
}

fn main() {
    let file_path = "dump.txt";
    match read_file(file_path) {
        Ok(content) => run(&content),
        Err(e) => eprintln!("Error reading file {file_path}: {e}"),
    }
}
