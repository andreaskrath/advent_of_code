use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("puzzle_input.txt").unwrap();
    let vec_input = input.split('\n').collect::<Vec<&str>>();

    let part1_time = Instant::now();
    println!("Part 1 solution: {}", part1(vec_input));
    println!("Completed in {:.2?}", part1_time.elapsed());
}

fn part1(vec: Vec<&str>) -> usize {
    vec.iter()
        .map(|line| {
            let split_line = line.trim().splitn(3, 'x').collect::<Vec<&str>>();
            let length = split_line.first().unwrap().parse::<usize>().unwrap();
            let width = split_line.get(1).unwrap().parse::<usize>().unwrap();
            let height = split_line.get(2).unwrap().parse::<usize>().unwrap();

            let side_a = length * width;
            let side_b = length * height;
            let side_c = width * height;

            let smallest_side = side_a.min(side_b).min(side_c);

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
