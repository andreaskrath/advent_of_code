use std::time::Instant;

fn main() {
    let puzzle_input =
        std::fs::read_to_string("./puzzle_input.txt").expect("should find puzzle input");

    let part_one_start = Instant::now();
    let part_one_res = part_one(puzzle_input.as_str());
    let part_one_end = part_one_start.elapsed();
    println!("Part 1: {}, in {:2.?}", part_one_res, part_one_end);

    let part_two_start = Instant::now();
    let part_two_res = part_two(puzzle_input.as_str());
    let part_two_end = part_two_start.elapsed();
    println!("Part 1: {}, in {:2.?}", part_two_res, part_two_end);
}

fn part_one(input_str: &str) -> u16 {
    input_str
        .lines()
        .map(|line| {
            let (a, b) = line.trim().split_at(line.len() / 2);
            let mut val = 0;
            for ch in a.chars() {
                if b.contains(ch) {
                    val = if ch.is_ascii_lowercase() {
                        ch as u16 - 'a' as u16 + 1
                    } else {
                        ch as u16 - 'A' as u16 + 27
                    };
                    break;
                }
            }

            val
        })
        .sum()
}

fn part_two(input_str: &str) -> u16 {
    // similar approach to part one, the main difference being the collect
    // so that the lines can be chunked in groups of 3
    let lines: Vec<&str> = input_str.lines().collect();
    lines
        .chunks_exact(3)
        .map(|slice| {
            let mut val = 0;
            for ch in slice[0].chars() {
                // because of the chunking the check has to contain both the other chunks
                if slice[1].contains(ch) && slice[2].contains(ch) {
                    val = if ch.is_ascii_lowercase() {
                        ch as u16 - 'a' as u16 + 1
                    } else {
                        ch as u16 - 'A' as u16 + 27
                    };
                    break;
                }
            }

            val
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_sample() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let expected = 157;
        let actual = part_one(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_sample() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let expected = 70;
        let actual = part_two(input);
        assert_eq!(actual, expected);
    }
}
