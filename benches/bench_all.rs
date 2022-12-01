use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_22::calculate;

const MAX_DAY: u32 = 1;

fn criterion_benchmark(c: &mut Criterion) {
    for i in 1..=MAX_DAY {
        c.bench_function(format!("d{}",i).as_str(), |b| b.iter(|| calculate(black_box(i))));
    }

    c.bench_function("all days", |b| b.iter(|| {
        for i in 1..=MAX_DAY {
            calculate(black_box(i));
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
