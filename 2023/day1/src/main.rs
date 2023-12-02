use std::time::Instant;

const NUMBER_WORDS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn main() {
    let input = include_str!("../input.txt");

    let part1_start = Instant::now();
    let part1_res = part1(input);
    let part1_stop = part1_start.elapsed();
    println!("Part 1 is '{}' in {:2.?}", part1_res, part1_stop);

    let part2_start = Instant::now();
    let part2_res = part2(input);
    let part2_stop = part2_start.elapsed();
    println!("Part 2 is '{}' in {:2.?}", part2_res, part2_stop);
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

fn part2(input: &str) -> u32 {
    let replaced = input
        .lines()
        .map(|s| {
            let s = s.as_bytes();
            let mut transformed = String::new();
            let mut i = 0;
            while i < s.len() {
                if (s[i] as char).is_ascii_digit() {
                    transformed.push(s[i] as char);
                    i += 1;
                    continue;
                }

                for (word, digit) in NUMBER_WORDS {
                    if s[i..].starts_with(word.as_bytes()) {
                        transformed.push(digit);
                        i += 1;
                        continue;
                    }
                }

                i += 1;
            }

            transformed
        })
        .collect::<Vec<String>>();

    part1(replaced.join("\n").as_str())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_sample() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected = 142;
        let actual = part1(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_sample() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let expected = 281;
        let actual = part2(input);
        assert_eq!(actual, expected);
    }
}
