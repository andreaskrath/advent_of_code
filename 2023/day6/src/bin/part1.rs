use std::time::Instant;

use day6::part1;

fn main() {
    let input = include_str!("../../input.txt");

    let part1_start = Instant::now();
    let part1_res = part1(input);
    let part1_stop = part1_start.elapsed();
    println!("Part 1 is '{}' in {:2.?}", part1_res, part1_stop);
}
