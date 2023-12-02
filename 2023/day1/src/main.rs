use std::time::Instant;


fn main() {
    let input = include_str!("../input.txt");

    let part1_start = Instant::now();
    let part1_res = part1(input);
    let part1_stop = part1_start.elapsed();
    println!("Part 1 is '{}' in {:2.?}", part1_res, part1_stop);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let digits = s
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();

            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod part1 {
    use crate::part1;

    #[test]
    fn sample() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected = 142;
        let actual = part1(input);
        assert_eq!(actual, expected);
    }
}

