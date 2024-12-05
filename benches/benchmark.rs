use criterion::{black_box, criterion_group, criterion_main, Criterion};
use grac::{syllabify_gr, syllabify_gr_ref, to_mono};
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
    group.sample_size(40);
    group.bench_function("syllabify_gr", |b| {
        b.iter(|| {
            let result: Vec<_> = words.iter().map(|word| syllabify_gr(word)).collect();
            black_box(result);
        })
    });

    group.bench_function("syllabify_gr_ref", |b| {
        b.iter(|| {
            let result: Vec<_> = words.iter().map(|word| syllabify_gr_ref(word)).collect();
            black_box(result);
        })
    });

    group.bench_function("to_mono", |b| {
        b.iter(|| {
            let result = to_mono(&content);
            black_box(result);
        })
    });
}

criterion_group!(benches, benchmark_syllabify_file);
criterion_main!(benches);
