use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day4::{part1, part2};

fn part1_bench(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("Part 1 with input.txt", |b| {
        b.iter(|| {
            let _ = part1(black_box(input));
        })
    });
}

fn part2_bench(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("Part 2 with input.txt", |b| {
        b.iter(|| {
            let _ = part2(black_box(input));
        })
    });
}

criterion_group!(benches, part1_bench, part2_bench);
criterion_main!(benches);
