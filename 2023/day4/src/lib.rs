pub fn part1(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let line = &line[9..];
        let (winners, numbers) = line
            .trim()
            .split_once('|')
            .map(|(w, n)| {
                let w: Vec<u8> = w
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                let n: Vec<u8> = n
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();

                (w, n)
            })
            .unwrap();

        let mut temp_sum = 0;
        for num in numbers {
            if winners.contains(&num) {
                if temp_sum == 0 {
                    temp_sum = 1;
                } else {
                    temp_sum *= 2;
                }
            }
        }
        sum += temp_sum;
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let scratchcards: Vec<(Vec<u8>, Vec<u8>)> = input
        .lines()
        .map(|l| {
            let (w, n) = l.to_string()[9..]
                .trim()
                .split_once('|')
                .map(|(w, n)| {
                    let w: Vec<u8> = w
                        .split_ascii_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    let n: Vec<u8> = n
                        .split_ascii_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();

                    (w, n)
                })
                .unwrap();

            (w, n)
        })
        .collect();

    let mut sum = scratchcards.len();
    let mut queue: Vec<usize> = (0..scratchcards.len()).collect();

    while let Some(index) = queue.pop() {
        let mut offset = 1;
        let (winners, numbers) = scratchcards.get(index).unwrap();
        for num in numbers {
            if winners.contains(num) {
                queue.push(index + offset);
                offset += 1;
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_sample() {
        let input = [
            "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card   2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card   3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card   4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card   5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card   6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let expected = 13;
        let actual = part1(input.join("\n").as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_sample() {
        let input = [
            "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card   2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card   3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card   4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card   5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card   6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let expected = 30;
        let actual = part2(input.join("\n").as_str());
        assert_eq!(actual, expected);
    }
}
