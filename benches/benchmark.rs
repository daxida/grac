use criterion::{black_box, criterion_group, criterion_main, Criterion};
use grac::*;
use std::fs::File;
use std::io::{self, Read};

macro_rules! benchmark_syllabify {
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

    benchmark_syllabify!(group, words, syllabify_gr);
    benchmark_syllabify!(group, words, syllabify_gr_ref);
    benchmark_syllabify!(group, words, syllabify_el);
}

fn benchmark_monotonic(c: &mut Criterion) {
    let file_path = "dump.txt";
    let content = read_file(file_path).unwrap();

    let mut group = c.benchmark_group("group");
    group.sample_size(10);

    group.bench_function("to_mono", |b| {
        b.iter(|| {
            let result = to_mono(&content);
            black_box(result);
        })
    });
}

criterion_group!(benches, benchmark_syllabify, benchmark_monotonic);
criterion_main!(benches);
