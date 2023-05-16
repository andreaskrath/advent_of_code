use std::time::Instant;

fn main() {
    let input_text = std::fs::read_to_string("puzzle_input.txt").expect("failed to read file");

    let part1_time = Instant::now();
    println!("Part 1 solution: {}", part1(input_text.clone()));
    println!("Completed in {:.2?}", part1_time.elapsed());

    println!();

    let part2_time = Instant::now();
    println!("Part 2 solution: {}", part2(input_text.clone()));
    println!("Completed in {:.2?}", part2_time.elapsed());

    println!();

    let part2_functional_time = Instant::now();
    println!(
        "Part 2 functional solution: {}",
        part2_functional(input_text)
    );
    println!("Completed in {:.2?}", part2_functional_time.elapsed());
}

fn part1(input: String) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod part1 {
    use crate::part1;

    #[test]
    fn sample_one() {
        let input = String::from("(())");
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn sample_two() {
        let input = String::from("(((");
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn sample_three() {
        let input = String::from(")))");
        assert_eq!(part1(input), -3);
    }

    #[test]
    fn sample_four() {
        let input = String::from("))(((((");
        assert_eq!(part1(input), 3);
    }
}

fn part2(input: String) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        };

        if floor == -1 {
            return i + 1;
        }
    }
    unreachable!()
}

fn part2_functional(input: String) -> usize {
    input
        .chars()
        .scan(0, |floor, c| {
            *floor += match c {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            };

            Some(*floor)
        })
        .enumerate()
        .find(|&(_, floor)| floor < 0)
        .unwrap()
        .0
        + 1
}

#[cfg(test)]
mod part2 {
    use crate::part2;

    #[test]
    fn sample_one() {
        let input = String::from(")");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn sample_two() {
        let input = String::from("()())");
        assert_eq!(part2(input), 5);
    }
}

#[cfg(test)]
mod part2_functional {
    use crate::part2_functional;

    #[test]
    fn sample_one() {
        let input = String::from(")");
        assert_eq!(part2_functional(input), 1);
    }

    #[test]
    fn sample_two() {
        let input = String::from("()())");
        assert_eq!(part2_functional(input), 5);
    }
}
