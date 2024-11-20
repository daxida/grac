use criterion::{black_box, criterion_group, criterion_main, Criterion};
use grac::syllabify;
use grac::syllabify_2;
use grac::syllabify_3;
use greek_syllables::syllables;
use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn benchmark_syllabify_file(c: &mut Criterion) {
    let file_path = "dump.txt";
    let content = read_file(file_path).unwrap();
    let words: Vec<&str> = content.split_whitespace().collect();

    let mut group = c.benchmark_group("group");
    group.sample_size(10);
    group.bench_function("syllabify", |b| {
        b.iter(|| {
            let result: Vec<_> = words.iter().map(|word| syllabify(word)).collect();
            black_box(result);
        })
    });

    group.bench_function("syllabify2", |b| {
        b.iter(|| {
            let result: Vec<_> = words.iter().map(|word| syllabify_2(word)).collect();
            black_box(result);
        })
    });

    group.bench_function("syllabify3", |b| {
        b.iter(|| {
            let result: Vec<_> = words.iter().map(|word| syllabify_3(word)).collect();
            black_box(result);
        })
    });

    group.bench_function("syllables", |b| {
        b.iter(|| {
            let result: Vec<_> = words.iter().map(|word| syllables(word)).collect();
            black_box(result);
        })
    });
}

criterion_group!(benches, benchmark_syllabify_file);
criterion_main!(benches);
