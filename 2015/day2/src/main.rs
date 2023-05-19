use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("puzzle_input.txt").unwrap();
    let vec_input = input.split('\n').collect::<Vec<&str>>();

    let part1_time = Instant::now();
    println!("Part 1 solution: {}", part1(vec_input.clone()));
    println!("Completed in {:.2?}", part1_time.elapsed());

    println!();

    let part2_time = Instant::now();
    println!("Part 2 solution: {}", part2(vec_input));
    println!("Completed in {:.2?}", part2_time.elapsed());
}

fn compute_input(line: &str) -> (usize, usize, usize) {
    let split_line = line.trim().splitn(3, 'x').collect::<Vec<&str>>();
    let length = split_line.first().unwrap().parse::<usize>().unwrap();
    let width = split_line.get(1).unwrap().parse::<usize>().unwrap();
    let height = split_line.get(2).unwrap().parse::<usize>().unwrap();

    (length, width, height)
}

#[cfg(test)]
mod compute_input {
    use crate::compute_input;

    #[test]
    fn sample_one() {
        let input = "2x3x4";
        assert_eq!(compute_input(input), (2, 3, 4));
    }

    #[test]
    fn sample_two() {
        let input = "1x1x10";
        assert_eq!(compute_input(input), (1, 1, 10));
    }
}

fn part1(vec: Vec<&str>) -> usize {
    vec.iter()
        .map(|line| {
            let (length, width, height) = compute_input(line);
            let smallest_side = (length * width).min(length * height).min(width * height);
            2 * ((length * width) + (width * height) + (height * length)) + smallest_side
        })
        .sum()
}

#[cfg(test)]
mod part1 {
    use crate::part1;

    #[test]
    fn sample_one() {
        let input = vec!["2x3x4"];
        assert_eq!(part1(input), 58);
    }

    #[test]
    fn sample_two() {
        let input = vec!["1x1x10"];
        assert_eq!(part1(input), 43);
    }
}

fn part2(vec: Vec<&str>) -> usize {
    vec.iter()
        .map(|line| {
            let (length, width, height) = compute_input(line);
            let mut sorted_sides = vec![length, width, height];
            sorted_sides.sort_unstable();
            sorted_sides.pop(); // removes the last element of vector, which is largest of the three
            let small_one = sorted_sides.pop().unwrap();
            let small_two = sorted_sides.pop().unwrap();

            (2 * small_one) + (2 * small_two) + (length * width * height)
        })
        .sum()
}

#[cfg(test)]
mod part2 {
    use crate::part2;

    #[test]
    fn sample_one() {
        let input = vec!["2x3x4"];
        assert_eq!(part2(input), 34);
    }

    #[test]
    fn sample_two() {
        let input = vec!["1x1x10"];
        assert_eq!(part2(input), 14);
    }
}
