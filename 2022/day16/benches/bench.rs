use criterion::{criterion_group, criterion_main, Criterion};
use day16::Problem;

fn part_one(c: &mut Criterion) {
    let problem = Problem::new("example.txt").unwrap();
    c.bench_function("part one", |b| b.iter(|| problem.part_one().unwrap()));
}

criterion_group!(benches, part_one);
criterion_main!(benches);
