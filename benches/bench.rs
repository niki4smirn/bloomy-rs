use bloomy_rs::BloomFilter;
use criterion::{criterion_group, criterion_main, Criterion};
use fastbloom_rs::FilterBuilder as FastBloomFilter;

fn read_input() -> Vec<String> {
    const FILENAME: &str = "input.txt";
    let mut input = Vec::new();
    let mut file = File::open(FILENAME).expect("Run cargo test first, to generate input.txt");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        input.push(line.to_string());
    }
    input
}

fn false_pos_prob(filter_sz: usize, inserts_num: usize) -> f64 {
    let e = std::f64::consts::E;
    let ln2sq = std::f64::consts::LN_2.powi(2);
    let f_sz = filter_sz as f64;
    let ins_cnt = inserts_num as f64;
    let power = -(f_sz / ins_cnt as f64 * ln2sq);
    e.powf(power)
}

use fastbloom_rs::Membership;
use std::fs::File;
use std::io::Read;

fn bench_bfs_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BloomFilter");
    const FILTER_SIZE: usize = 1024 * 1024 * 8;
    let input = read_input();
    let input = input.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
    group.bench_function("bloomy/insert", |b| {
        // iter_batched here was not working, so I had to do this
        // to be more precise, the process of building was killed after some amount of time
        // I think the problem was because of stack overflow or something like that
        //
        // so the benchmark is a little unfair to my implementation
        // but it's still faster
        b.iter(|| {
            let mut bf = BloomFilter::<FILTER_SIZE>::new(input.len());
            for word in &input {
                bf.insert(word);
            }
        })
    });
    group.bench_function("fastbloom/insert", |b| {
        b.iter_batched(
            || {
                FastBloomFilter::new(input.len() as u64, false_pos_prob(FILTER_SIZE, input.len()))
                    .build_bloom_filter()
            },
            |mut fbf| {
                for word in &input {
                    fbf.add(word);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_bfs_contains_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("BloomFilter");
    const FILTER_SIZE: usize = 1024 * 1024 * 8;
    let input = read_input();
    let input = input.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
    let mut bf = BloomFilter::<FILTER_SIZE>::new(input.len());
    for word in &input {
        bf.insert(word);
    }
    let mut fbf =
        FastBloomFilter::new(input.len() as u64, false_pos_prob(FILTER_SIZE, input.len()))
            .build_bloom_filter();
    for word in &input {
        fbf.add(word);
    }
    group.bench_function("bloomy/contains_existing", |b| {
        b.iter(|| {
            for word in &input {
                bf.contains(word);
            }
        })
    });
    group.bench_function("fastbloom/contains_existing", |b| {
        b.iter(|| {
            for word in &input {
                fbf.contains(word);
            }
        })
    });
    group.finish();
}

fn bench_bfs_contains_non_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("BloomFilter");
    const FILTER_SIZE: usize = 1024 * 32;
    let input = read_input();
    let mut input = input.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
    let to_check = input.split_off(input.len() / 2);
    let to_check = to_check
        .iter()
        .filter(|val| !input.contains(val))
        .collect::<Vec<&&[u8]>>();
    let mut bf = BloomFilter::<FILTER_SIZE>::new(input.len());
    for word in &input {
        bf.insert(word);
    }
    let mut fbf =
        FastBloomFilter::new(input.len() as u64, false_pos_prob(FILTER_SIZE, input.len()))
            .build_bloom_filter();
    for word in &input {
        fbf.add(word);
    }
    group.bench_function("bloomy/contains_non_existing", |b| {
        b.iter(|| {
            for word in &to_check {
                bf.contains(word);
            }
        })
    });
    group.bench_function("fastbloom/contains_non_existing", |b| {
        b.iter(|| {
            for word in &to_check {
                fbf.contains(word);
            }
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_bfs_insert,
    bench_bfs_contains_existing,
    bench_bfs_contains_non_existing
);
criterion_main!(benches);
