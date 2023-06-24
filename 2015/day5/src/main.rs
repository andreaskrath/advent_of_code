use std::{fs, time::Instant};

fn main() {
    let puzzle_input =
        fs::read_to_string("./puzzle_input.txt").expect("failed to get puzzle input");

    let part_one_time = Instant::now();
    let part_one_res = part_one(&puzzle_input);
    let part_one_elapsed = part_one_time.elapsed();
    println!("Part 1: {} in {:2.?}", part_one_res, part_one_elapsed);
}

fn part_one(input: &str) -> usize {
    let split_input: Vec<&str> = input.split_whitespace().collect();

    let mut counter = 0;
    for s in split_input {
        let mut vowels = 0;
        let mut has_double_letters = false;
        let mut no_special_strings = true;

        let mut char_iter = s.chars().peekable();
        while let Some(c) = char_iter.next() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
                vowels += 1;
            }

            if let Some(next_char) = char_iter.peek() {
                if c == *next_char {
                    has_double_letters = true;
                }

                no_special_strings = !matches!(
                    (c, *next_char),
                    ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
                );
                if !no_special_strings {
                    break;
                }
            }
        }

        if (vowels >= 3) && has_double_letters && no_special_strings {
            counter += 1;
        }
    }

    counter
}

#[cfg(test)]
mod part_one {
    use crate::part_one;

    #[test]
    fn sample_one() {
        let input = "ugknbfddgicrmopn";
        let result = part_one(input);
        assert!(result == 1);
    }

    #[test]
    fn sample_two() {
        let input = "aaa";
        let result = part_one(input);
        assert!(result == 1);
    }

    #[test]
    fn sample_three() {
        let input = "jchzalrnumimnmhp";
        let result = part_one(input);
        assert!(result == 0);
    }

    #[test]
    fn sample_four() {
        let input = "haegwjzuvuyypxyu";
        let result = part_one(input);
        assert!(result == 0);
    }

    #[test]
    fn sample_five() {
        let input = "dvszwmarrgswjxmb";
        let result = part_one(input);
        assert!(result == 0);
    }
}
