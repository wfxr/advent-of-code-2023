use std::collections::HashMap;

use crate::{solution, AocResult};

fn name_to_id(name: &str) -> usize {
    name.bytes().fold(0, |acc, c| acc * 26 + (c - b'A') as usize)
}

fn part1(input: &str) -> AocResult<usize> {
    let (dirs, nodes) = input.split_once("\n\n").ok_or("invalid input")?;

    let network: HashMap<_, _> = nodes
        .lines()
        .map(|line| {
            let mut it = line
                .split(|c: char| !c.is_ascii_uppercase())
                .filter(|s| !s.is_empty())
                .map(name_to_id);
            match (it.next(), it.next(), it.next(), it.next()) {
                (Some(name), Some(left), Some(right), None) => Ok((name, (left, right))),
                _ => Err(format!("invalid node: {}", line)),
            }
        })
        .collect::<Result<_, _>>()?;

    let dest = name_to_id("ZZZ");
    let mut curr = name_to_id("AAA");

    for (i, dir) in dirs.chars().cycle().enumerate() {
        let (left, right) = network[&curr];
        curr = match dir {
            'L' => left,
            'R' => right,
            _ => return Err(format!("invalid direction: {}", dir).into()),
        };

        if curr == dest {
            return Ok(i + 1);
        }
    }

    unreachable!()
}

fn part2(_input: &str) -> AocResult<usize> {
    todo!()
}

solution!(part1 => todo!(), part2 => todo!());

#[cfg(test)]
mod tests {
    const EXAMPLE1: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE2: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    crate::test!(part1,
        t1: EXAMPLE1.trim() => 2,
        t2: EXAMPLE2.trim() => 6,
    );

    crate::test!(part2, t1: EXAMPLE1.trim() => todo!());
}
