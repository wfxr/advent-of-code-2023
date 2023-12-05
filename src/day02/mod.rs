use std::str::FromStr;

use crate::{solution, AocResult};

struct Cubes {
    r: usize,
    g: usize,
    b: usize,
}

impl FromStr for Cubes {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut cubes = Self { r: 0, g: 0, b: 0 };
        for s in value.split(',') {
            let mut it = s.trim().split(' ');
            let (n, color) = match (it.next(), it.next(), it.next()) {
                (Some(n), Some(color), None) => (n.parse::<usize>()?, color),
                _ => return Err(format!("Invalid cube: {s}").into()),
            };
            match color {
                "red" => cubes.r += n,
                "green" => cubes.g += n,
                "blue" => cubes.b += n,
                _ => return Err(format!("Invalid cube: {s}").into()),
            }
        }
        Ok(cubes)
    }
}

struct Game {
    round: usize,
    sets:  Vec<Cubes>,
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (round, sets) = value.split_once(": ").ok_or_else(|| format!("Invalid game: {value}"))?;
        let round: usize = round
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
            if game.sets.iter().all(|set| set.r <= 12 && set.g <= 13 && set.b <= 14) {
                acc += game.round;
            }
            acc
        })
    })
}

fn part2(_input: &str) -> AocResult<usize> {
    todo!()
}

solution!(part1 => 2204, part2 => todo!());

#[cfg(test)]
mod tests {
    crate::test!(part1, t1: "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ".trim() => 8);

    crate::test!(part2, t1: "" => 42);
}
