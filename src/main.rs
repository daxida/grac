#[allow(unused_imports)]
use grac::{syllabify_gr, syllabify_gr_ref, to_mono};
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn simple_benchmark() {
    let file_path = "dump.txt";

    match read_file(file_path) {
        Ok(content) => {
            let words: Vec<&str> = content.split_whitespace().collect();
            println!(
                "Content size: {}. Number of words: {}",
                content.len(),
                words.len()
            );

            let now = Instant::now();
            let _syls: Vec<Vec<_>> = words.iter().map(|cword| syllabify_gr(cword)).collect();
            println!("Syllabification took: {:?}", now.elapsed());

            let now = Instant::now();
            let _mono = to_mono(&content);
            println!("To monotonic took:    {:?}", now.elapsed());
        }
        Err(e) => eprintln!("Error reading file {}: {}", file_path, e),
    }
}

fn main() {
    simple_benchmark();
}
