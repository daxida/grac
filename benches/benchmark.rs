use criterion::{black_box, criterion_group, criterion_main, Criterion};
use grac::*;
use std::fs::File;
use std::io::{self, Read};

macro_rules! bench_words {
    ($group:expr, $words:expr, $( $fn:ident ),* ) => {
        $(
            $group.bench_function(stringify!($fn), |b| {
                b.iter(|| {
                    let result: Vec<_> = $words.iter().map(|word| $fn(word)).collect();
                    black_box(result);
                });
            });
        )*
    };
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn benchmark_syllabify(c: &mut Criterion) {
    let file_path = "dump.txt";
    let content = read_file(file_path).unwrap();
    let words: Vec<&str> = content.split_whitespace().collect();

    let mut group = c.benchmark_group("group");
    group.sample_size(20);

    bench_words!(group, words, syllabify_gr);
    bench_words!(group, words, syllabify_gr_ref);
    bench_words!(group, words, syllabify_el);
}

fn benchmark_monotonic(c: &mut Criterion) {
    let file_path = "dump.txt";
    let content = read_file(file_path).unwrap();
    let words: Vec<&str> = content.split_whitespace().collect();

    let mut group = c.benchmark_group("group");
    group.sample_size(10);

    bench_words!(group, words, split_word_punctuation);
    group.bench_function("to_mono", |b| {
        b.iter(|| {
            let result = to_mono(&content);
            black_box(result);
        })
    });
}

fn benchmark_char(c: &mut Criterion) {
    let file_path = "dump.txt";
    let content = read_file(file_path).unwrap();
    let words: Vec<&str> = content.split_whitespace().collect();

    let mut group = c.benchmark_group("group");
    group.sample_size(10);

    bench_words!(group, words, is_greek_word);
}

criterion_group!(
    benches,
    benchmark_syllabify,
    benchmark_monotonic,
    benchmark_char
);
criterion_main!(benches);
