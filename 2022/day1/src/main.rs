use std::{collections::BinaryHeap, time::Instant};

fn main() {
    let puzzle_input =
        std::fs::read_to_string("./puzzle_input.txt").expect("should find puzzle input");

    let part_one_start = Instant::now();
    let part_one_result = part_one(puzzle_input.as_str());
    let part_one_end = part_one_start.elapsed();
    println!("Part 1: {}, in {:2.?}", part_one_result, part_one_end);

    let part_two_start = Instant::now();
    let part_two_res = part_two(puzzle_input.as_str());
    let part_two_end = part_two_start.elapsed();
    println!("Part 2: {}, in {:2.?}", part_two_res, part_two_end);
}

fn part_one(input_str: &str) -> usize {
    input_str
        // each elfs inventory is split with a double newline
        // 1 newline from the end of last item, and one empty line between inventories
        .split("\n\n")
        .map(|slice| {
            // the slice contains an elfs inventory as a single string
            // which is split, parsed and summed to get each elves total inventory value
            slice
                .lines()
                .map(|line| {
                    line.trim()
                        .parse::<usize>()
                        .expect("should always be a number")
                })
                .sum()
        })
        // taking the max value from this temp collection represents
        // the elf with the most calories in their inventory
        .max()
        .expect("should have at least one value")
}

fn part_two(input_str: &str) -> usize {
    let mut heap = input_str
        .split("\n\n")
        .map(|slice| {
            slice
                .lines()
                .map(|line| {
                    line.trim()
                        .parse::<usize>()
                        .expect("should always be a number")
                })
                .sum()
        })
        .collect::<BinaryHeap<usize>>();

    // process is the exact same, but instead of returning max we collect into a max-binary heap
    // to easily access the three largest elements of the collection
    heap.pop().expect("should have one element")
        + heap.pop().expect("should have a second element")
        + heap.pop().expect("should have a third element")
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_sample() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let expected = 24000;
        let actual = part_one(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_sample() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let expected = 45000;
        let actual = part_two(input);
        assert_eq!(actual, expected);
    }
}
