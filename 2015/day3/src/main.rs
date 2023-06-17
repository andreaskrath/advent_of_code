use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn main() {
    let input = read_to_string("puzzle_input.txt").expect("failed to read puzzle input");

    let part_one_start = Instant::now();
    let part_one_res = part_one(input.trim());
    let part_one_end = part_one_start.elapsed();
    println!("Part 1: {}, in {:.2?}", part_one_res, part_one_end);

    let part_two_start = Instant::now();
    let part_two_res = part_two(input.trim());
    let part_two_end = part_two_start.elapsed();
    println!("Part 2: {}, in {:.2?}", part_two_res, part_two_end);
}

#[inline(always)]
fn update_location(x: i16, y: i16, input: char) -> (i16, i16) {
    match input {
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        '<' => (x - 1, y),
        '>' => (x + 1, y),
        _ => unreachable!(),
    }
}

fn part_one(input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut grid: HashMap<(i16, i16), u16> = HashMap::new();

    // inserting gift at initial location
    grid.insert((x, y), 1);

    for c in input.chars() {
        (x, y) = update_location(x, y, c);
        grid.entry((x, y)).and_modify(|n| *n += 1).or_insert(1);
    }

    grid.len()
}

fn part_two(input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    let mut grid: HashMap<(i16, i16), u16> = HashMap::new();

    grid.insert((x, y), 1);

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            (x, y) = update_location(x, y, c);
            grid.entry((x, y)).and_modify(|n| *n += 1).or_insert(1);
        } else {
            (robo_x, robo_y) = update_location(robo_x, robo_y, c);
            grid.entry((robo_x, robo_y))
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
    }

    grid.len()
}

#[cfg(test)]
mod part_one {
    use crate::part_one;

    #[test]
    fn sample_one() {
        let input = ">";
        let expected = 2;
        let actual = part_one(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = "^>v<";
        let expected = 4;
        let actual = part_one(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = "^v^v^v^v^v";
        let expected = 2;
        let actual = part_one(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod part_two {
    use crate::part_two;

    #[test]
    fn sample_one() {
        let input = "^v";
        let expected = 3;
        let actual = part_two(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = "^>v<";
        let expected = 3;
        let actual = part_two(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = "^v^v^v^v^v";
        let expected = 11;
        let actual = part_two(input);
        assert_eq!(actual, expected);
    }
}
