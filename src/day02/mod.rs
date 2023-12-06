use std::str::FromStr;

use crate::{solution, AocResult};

struct Cubes {
    r: usize,
    g: usize,
    b: usize,
}

struct Game {
    round: usize,
    sets:  Vec<Cubes>,
}

impl FromStr for Cubes {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for s in value.split(',') {
            let mut it = s.trim().split(' ');
            let (n, color) = match (it.next(), it.next(), it.next()) {
                (Some(n), Some(color), None) => (n.parse::<usize>()?, color),
                _ => return Err(format!("Invalid cube: {s}").into()),
            };
            match color {
                "red" => r += n,
                "green" => g += n,
                "blue" => b += n,
                _ => return Err(format!("Invalid cube: {s}").into()),
            }
        }
        Ok(Self { r, g, b })
    }
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (round, sets) = value.split_once(": ").ok_or_else(|| format!("Invalid game: {value}"))?;
        let round = round
            .rsplit(' ')
            .next()
            .ok_or_else(|| format!("Invalid game: {value}"))?
            .parse()?;
        let sets = sets.split(';').map(|s| s.parse()).collect::<Result<_, _>>()?;
        Ok(Self { round, sets })
    }
}

fn part1(input: &str) -> AocResult<usize> {
    input.lines().try_fold(0, |mut acc, line| {
        Game::from_str(line).map(|game| {
            if game.sets.iter().all(|&Cubes { r, g, b }| r <= 12 && g <= 13 && b <= 14) {
                acc += game.round;
            }
            acc
        })
    })
}

fn part2(input: &str) -> AocResult<usize> {
    input
        .lines()
        .map(|line| {
            Game::from_str(line).map(|game| {
                game.sets
                    .iter()
                    .fold([0, 0, 0], |[r, g, b], s| [r.max(s.r), g.max(s.g), b.max(s.b)])
                    .iter()
                    .product::<usize>()
            })
        })
        .sum()
}

solution!(part1 => 2204, part2 => 71036);

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    crate::test!(part1, t1: EXAMPLE.trim() => 8);

    crate::test!(part2, t1: EXAMPLE.trim() => 2286);
}
