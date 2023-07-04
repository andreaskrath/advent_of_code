use std::time::Instant;

fn main() {
    let puzzle_input =
        std::fs::read_to_string("./puzzle_input.txt").expect("should find puzzle input");

    let part_one_start = Instant::now();
    let part_one_res = part_one(puzzle_input.as_str());
    let part_one_end = part_one_start.elapsed();
    println!("Part 1: {}, in {:2.?}", part_one_res, part_one_end);
}

fn part_one(input_str: &str) -> u16 {
    input_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').expect("should always contain a comma");
            let (a_lower, a_upper) = a.split_once('-').and_then(|(a1, a2)| {
                Some((a1.parse::<u16>().unwrap(), a2.parse::<u16>().unwrap()))
            });
            let (b_lower, b_upper) = b.split_once('-').expect("should always contain hyphen");
            if (a_lower <= b_lower && a_upper >= b_upper)
                || (a_lower >= b_lower && a_upper <= b_upper)
            {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    #[test]
    fn part_one_sample() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let expected = 2;
        let actual = part_one(input);
        assert_eq!(actual, expected);
    }
}
