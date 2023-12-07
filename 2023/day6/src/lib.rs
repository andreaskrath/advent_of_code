pub fn part1(input: &str) -> usize {
    let mut input = input.lines();
    let time = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let distance = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut product = 1;
    for (t, d) in time.into_iter().zip(distance) {
        let mut sum = 0;
        for n in 0..=t {
            let remaining_time = t - n;
            if remaining_time * n > d {
                sum += 1;
            }
        }

        if sum > 0 {
            product *= sum;
        }
    }

    product
}

pub fn part2(input: &str) -> usize {
    let mut input = input.lines();
    let time = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .fold(String::new(), |concat, s| format!("{concat}{s}"))
        .parse::<usize>()
        .unwrap();
    let distance = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .fold(String::new(), |concat, s| format!("{concat}{s}"))
        .parse::<usize>()
        .unwrap();

    let mut i = 0;
    while (time - i) * i <= distance {
        i += 1;
    }

    let mut j = time;
    while (time - j) * j <= distance {
        j -= 1;
    }

    1 + j - i
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_sample() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let expected = 288;
        let actual = part1(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_sample() {
        let input = "Time: 71530\nDistance: 940200";
        let expected = 71503;
        let actual = part2(input);
        assert_eq!(actual, expected);
    }
}
