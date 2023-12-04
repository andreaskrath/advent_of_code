pub fn part1(input: &str) -> usize {
    let mut game_id = 1;
    let mut sum = 0;

    for line in input.lines() {
        let mut valid_game = true;
        let pattern = format!("Game {}:", game_id);
        let stripped = line.strip_prefix(pattern.as_str()).unwrap();
        let sets = stripped.split(';');

        'target: for set in sets {
            let cubes = set.split(',');
            for cube in cubes {
                let (num, color) = cube.trim().split_once(' ').unwrap();
                let num: usize = num.parse().unwrap();
                match color {
                    "red" => {
                        if num > 12 {
                            valid_game = false;
                            break 'target;
                        }
                    }
                    "green" => {
                        if num > 13 {
                            valid_game = false;
                            break 'target;
                        }
                    }
                    "blue" => {
                        if num > 14 {
                            valid_game = false;
                            break 'target;
                        }
                    }
                    _ => unreachable!("colors are bounded and this arm should not be possible"),
                }
            }
        }

        if valid_game {
            sum += game_id;
        }
        game_id += 1;
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let mut game_id = 1;
    let mut sum = 0;

    for line in input.lines() {
        let pattern = format!("Game {}:", game_id);
        let stripped = line.strip_prefix(pattern.as_str()).unwrap();
        let sets = stripped.split(';');

        let (mut red, mut green, mut blue) = (0, 0, 0);
        for set in sets {
            let cubes = set.split(',');
            for cube in cubes {
                let (num, color) = cube.trim().split_once(' ').unwrap();
                let num: usize = num.parse().unwrap();
                match color {
                    "red" => red = red.max(num),
                    "green" => green = green.max(num),
                    "blue" => blue = blue.max(num),
                    _ => unreachable!("colors are bounded and this arm should not be possible"),
                }
            }
        }

        sum += red * green * blue;
        game_id += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 8;
        let actual = part1(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 2286;
        let actual = part2(input);
        assert_eq!(actual, expected);
    }
}
