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
    println!("Part 2: {}, in {:2.?}", part_two_res, part_two_end);
}

// rock = A X
// paper = B Y
// scissors = C Z

#[inline]
fn calculate_score(elf_hand: &str, my_hand: &str) -> usize {
    let game_score = match (elf_hand, my_hand) {
        // wins
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        // draws
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        // losses
        ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
        _ => unreachable!("the match clause is guarded"),
    };

    let hand_score = match my_hand {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!("the match clause is guarded"),
    };

    game_score + hand_score
}

#[inline]
fn calculate_my_hand<'a>(elf_hand: &str, expected_result: &str) -> &'a str {
    match (elf_hand, expected_result) {
        ("A", "Y") | ("B", "X") | ("C", "Z") => "X",
        ("A", "Z") | ("B", "Y") | ("C", "X") => "Y",
        ("A", "X") | ("B", "Z") | ("C", "Y") => "Z",
        _ => unreachable!("the match clause is guarded"),
    }
}

fn part_one(input_str: &str) -> usize {
    input_str
        .lines()
        .map(|line| {
            let mut split_hand = line.split_whitespace();
            let (elf_hand, my_hand) = (
                split_hand.next().expect("should have elf hand"),
                split_hand.next().expect("should have my hand"),
            );

            calculate_score(elf_hand, my_hand)
        })
        .sum()
}

fn part_two(input_str: &str) -> usize {
    input_str
        .lines()
        .map(|line| {
            let mut split_hand = line.split_whitespace();
            let (elf_hand, expected_result) = (
                split_hand.next().expect("should have elf hand"),
                split_hand.next().expect("should have my hand"),
            );

            let my_hand = calculate_my_hand(elf_hand, expected_result);
            calculate_score(elf_hand, my_hand)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_sample() {
        let input = "A Y\nB X\nC Z";
        let expected = 15;
        let actual = part_one(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_sample() {
        let input = "A Y\nB X\nC Z";
        let expected = 12;
        let actual = part_two(input);
        assert_eq!(actual, expected);
    }
}
