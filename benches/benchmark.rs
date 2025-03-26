use criterion::{black_box, criterion_group, criterion_main, Criterion};
use grac::{is_greek_word, syllabify_el, syllabify_gr, syllabify_gr_ref, to_monotonic};
use std::fs::File;
use std::io::Read;
use std::path::Path;

macro_rules! bench_words {
    ($group:expr, $words:expr, $name:expr, $( $fn:ident ),* ) => {
        $(
            $group.bench_with_input(format!("{}@{}", stringify!($fn), $name), &$words, |b, i| {
                b.iter(|| {
                    let result: Vec<_> = i.iter().map(|word| $fn(word)).collect();
                    black_box(result);
                });
            });
        )*
    };
}

fn read_file(file_path: &str) -> (String, String) {
    let mut content = String::new();

    let _ = File::open(file_path).and_then(|mut file| file.read_to_string(&mut content));

    let stem = Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let name = format!("{stem}.txt");

    (content, name)
}

// const DUMP_PATH: &str = "tests/fixtures/dump.txt";
const MONO_PATH: &str = "tests/fixtures/monotonic.txt";
const PATHS: [&str; 3] = [
    MONO_PATH,
    "tests/fixtures/polytonic.txt",
    "tests/fixtures/english.txt",
];

fn benchmark_syllabify(c: &mut Criterion) {
    let mut group = c.benchmark_group("syllabify");
    group
        .measurement_time(std::time::Duration::new(3, 0))
        .warm_up_time(std::time::Duration::new(2, 0));

    for file_path in PATHS {
        let (content, stem) = read_file(file_path);
        let words: Vec<_> = content.split_whitespace().collect();

        bench_words!(group, words, stem, syllabify_gr);
        bench_words!(group, words, stem, syllabify_gr_ref);
        bench_words!(group, words, stem, syllabify_el);
    }
}

fn benchmark_to_monotonic(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_monotonic");
    group
        .measurement_time(std::time::Duration::new(3, 0))
        .warm_up_time(std::time::Duration::new(2, 0));

    for file_path in PATHS {
        let (content, stem) = read_file(file_path);
        // let words: Vec<_> = content.split_whitespace().collect();
        // bench_words!(group, words, split_punctuation);
        group.bench_with_input(stem, &content, |b, i| {
            b.iter(|| {
                let result = to_monotonic(i);
                black_box(result);
            });
        });
    }
}

fn benchmark_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_greek_word");
    group
        .measurement_time(std::time::Duration::new(3, 0))
        .warm_up_time(std::time::Duration::new(2, 0));

    for file_path in PATHS {
        let (content, stem) = read_file(file_path);
        let words: Vec<_> = content.split_whitespace().collect();
        group.bench_with_input(stem, &words, |b, i| {
            b.iter(|| {
                let result: Vec<_> = i.iter().map(|word| is_greek_word(word)).collect();
                black_box(result);
            });
        });
    }
}

criterion_group!(
    benches,
    benchmark_syllabify,
    benchmark_to_monotonic,
    benchmark_char
);
criterion_main!(benches);
