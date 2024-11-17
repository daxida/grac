#[allow(unused_imports)]
use grac::{syllabify, syllabify_2, syllabify_3};
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn clean_word(word: &str) -> String {
    word.chars().filter(|c| c.is_alphanumeric()).collect()
}

fn main() {
    let file_path = "dump.txt";

    match read_file(file_path) {
        Ok(content) => {
            let words: Vec<&str> = content.split_whitespace().collect();
            let clean_words: Vec<String> = words.iter().map(|&word| clean_word(word)).collect();

            let start_syllabify = Instant::now();
            let _syllabified_words: Vec<Vec<String>> =
                clean_words.iter().map(|cword| syllabify_3(cword)).collect();
            let duration_syllabify = start_syllabify.elapsed();
            println!(
                "Total time for syllabification + allocation: {:?}",
                duration_syllabify
            );
        }
        Err(e) => eprintln!("Error reading file {}: {}", file_path, e),
    }
}
