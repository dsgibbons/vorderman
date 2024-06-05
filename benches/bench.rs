use std::collections::HashSet;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vorderman::search::{search, NumbersRound};

pub fn criterion_benchmark(c: &mut Criterion) {
    let inputs = vec![
        NumbersRound {
            target: 321,
            numbers: HashSet::from_iter(vec![1, 2, 3, 5, 10, 100]),
        },
        NumbersRound {
            target: 120,
            numbers: HashSet::from_iter(vec![8, 3, 7, 2, 5, 4]),
        },
        NumbersRound {
            target: 615,
            numbers: HashSet::from_iter(vec![25, 3, 7, 2, 5, 4]),
        },
        NumbersRound {
            target: 813,
            numbers: HashSet::from_iter(vec![1, 10, 25, 50, 75, 100]),
        },
        NumbersRound {
            target: 952,
            numbers: HashSet::from_iter(vec![3, 6, 25, 50, 75, 100]),
        },
    ];
    let mut group = c.benchmark_group("search");
    group.sample_size(10);
    for input in inputs.iter() {
        group.bench_function(format!("input-{}", input.target.clone()), |b| {
            b.iter(|| search(black_box(input.clone())))
        });
    }
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
