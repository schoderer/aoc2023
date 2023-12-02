use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day2::*;
use utils::Part;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../inputs/day2.txt");
    c.bench_function("day2_parser", |b| {
        b.iter(|| {
            let _ = parser::parse_game(input);
        })
    });
    c.bench_function("day2_part1", |b| {
        b.iter(|| {
            let mut part = Part1;
            part.run_part(black_box(input))
        })
    });
    c.bench_function("day2_part2", |b| {
        b.iter(|| {
            let mut part = Part2;
            part.run_part(black_box(input))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
