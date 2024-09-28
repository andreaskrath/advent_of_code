use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let result_1 = part1(input);
    println!("part1: {result_1}");

    let result_2 = part2(input);
    println!("part2: {result_2}");
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn new_direction(self, c: char) -> Self {
        match c {
            'L' => match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            'R' => match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            _ => unreachable!("bounded by exercise description"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }

    fn dist(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn part1(s: &str) -> i32 {
    let mut direction = Direction::North;
    let (mut x, mut y) = (0, 0);

    for instruction in s.split(',') {
        let (s_turn, s_steps) = instruction.trim().split_at(1);
        let turn = s_turn.chars().next().expect("failed to get turn char");
        let steps: i32 = s_steps.parse().expect("failed to parse steps");

        direction = direction.new_direction(turn);

        match direction {
            Direction::North => y += steps,
            Direction::East => x += steps,
            Direction::South => y -= steps,
            Direction::West => x -= steps,
        }
    }

    x.abs() + y.abs()
}

fn part2(s: &str) -> i32 {
    let mut direction = Direction::North;
    let mut location = Location::new(0, 0);
    let mut locations = HashSet::new();

    for instruction in s.split(',') {
        let (s_turn, s_steps) = instruction.trim().split_at(1);
        let turn = s_turn.chars().next().expect("failed to get turn char");
        let steps: i32 = s_steps.parse().expect("failed to parse steps");

        direction = direction.new_direction(turn);

        for _ in 0..steps {
            location.step(direction);

            if !locations.insert(location.clone()) {
                return location.dist();
            }
        }
    }

    location.dist()
}

#[cfg(test)]
mod part1 {
    use crate::part1;

    #[test]
    fn example_1() {
        let input = "R2, L3";
        let expected = 5;

        let actual = part1(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let input = "R2, R2, R2";
        let expected = 2;

        let actual = part1(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_3() {
        let input = "R5, L5, R5, R3";
        let expected = 12;

        let actual = part1(input);

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod part2 {
    use crate::part2;

    #[test]
    fn example_1() {
        let input = "R8, R4, R4, R8";
        let expected = 4;

        let actual = part2(input);

        assert_eq!(actual, expected);
    }
}
