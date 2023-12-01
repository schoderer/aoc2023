use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01;



fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../inputs/day01.txt");
    c.bench_function("part1", |b| b.iter(|| day01::part1::process(black_box(input))));
    c.bench_function("part2", |b| b.iter(|| day01::part2::process(black_box(input))));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);