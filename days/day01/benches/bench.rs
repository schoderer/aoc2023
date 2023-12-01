use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01;
use utils::Part;


fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../inputs/day01.txt");

    c.bench_function("part1", |b| b.iter(|| {
        let mut part = day01::Part1;
        part.run_part(black_box(input))
    }));
    c.bench_function("part2", |b| b.iter(|| {
        let mut part = day01::Part2;
        part.run_part(black_box(input))
    }));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);