use std::time::Instant;

fn main() {
    let input = include_str!("../../input.txt");

    let part2_start = Instant::now();
    let part2_res = part2(input);
    let part2_stop = part2_start.elapsed();
    println!("Part 2 is '{}' in {:2.?}", part2_res, part2_stop);
}
