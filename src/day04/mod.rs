use std::collections::HashSet;

use crate::{solution, AocResult};

fn part1(input: &str) -> AocResult<usize> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(|c| c == ':' || c == '|').skip(1);
            match (parts.next(), parts.next(), parts.next()) {
                (Some(wins), Some(mine), None) => {
                    let wins: HashSet<usize> = wins
                        .split_ascii_whitespace()
                        .map(|n| n.parse::<usize>())
                        .collect::<Result<_, _>>()?;
                    mine.split_ascii_whitespace()
                        .map(|n| n.parse::<usize>())
                        .try_fold(1usize, |mut acc, num| {
                            if wins.contains(&num?) {
                                acc *= 2;
                            }
                            Ok(acc)
                        })
                        .map(|n| n / 2)
                }
                _ => Err(format!("Invalid input: {}", line).into()),
            }
        })
        .sum()
}

fn part2(_input: &str) -> AocResult<usize> {
    todo!()
}

solution!(part1 => 23235, part2 => todo!());

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    crate::test!(part1, t1: EXAMPLE.trim() => 13);
    crate::test!(part2, t1: "" => 42);
}
