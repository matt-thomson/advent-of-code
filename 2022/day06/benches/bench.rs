use criterion::{criterion_group, criterion_main, Criterion};
use day06::Problem;

fn part_one(c: &mut Criterion) {
    let problem = Problem::new("example.txt").unwrap();
    c.bench_function("example", |b| b.iter(|| problem.part_one()));
}

fn part_two(c: &mut Criterion) {
    let problem = Problem::new("example.txt").unwrap();
    c.bench_function("example", |b| b.iter(|| problem.part_two()));
}

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);
