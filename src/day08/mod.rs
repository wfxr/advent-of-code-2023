use crate::*;
use std::collections::HashMap;

fn name_to_id(name: &str) -> u32 {
    name.bytes().fold(0, |acc, c| acc * 26 + (c - b'A') as u32)
}

type Network = HashMap<u32, (u32, u32)>;

fn parse_input(input: &str) -> Result<(&str, Network)> {
    let (dirs, nodes) = input.split_once("\n\n").ok_or_else(|| anyhow!("invalid input"))?;

    let network: Network = nodes
        .lines()
        .map(|line| {
            let mut it = line
                .split(|c: char| !c.is_ascii_uppercase())
                .filter(|s| !s.is_empty())
                .map(name_to_id);
            match (it.next(), it.next(), it.next(), it.next()) {
                (Some(name), Some(left), Some(right), None) => Ok((name, (left, right))),
                _ => bail!("invalid node: {}", line),
            }
        })
        .collect::<Result<_>>()?;
    Ok((dirs, network))
}

fn part1(input: &str) -> Result<usize> {
    let (dirs, network) = parse_input(input)?;

    let dest = name_to_id("ZZZ");
    let mut curr = name_to_id("AAA");

    for (dir, i) in dirs.chars().cycle().zip(1..) {
        let (left, right) = network[&curr];
        curr = match dir {
            'L' => left,
            'R' => right,
            _ => bail!("invalid direction: {}", dir),
        };

        if curr == dest {
            return Ok(i);
        }
    }

    unreachable!()
}

fn part2(input: &str) -> Result<usize> {
    let (dirs, network) = parse_input(input)?;

    let mut currs: Vec<_> = network.keys().copied().filter(|&id| id % 26 == 0).collect();
    let mut steps = Vec::with_capacity(currs.len());

    for curr in currs.iter_mut() {
        for (dir, i) in dirs.chars().cycle().zip(1..) {
            let (left, right) = network[curr];
            *curr = match dir {
                'L' => left,
                'R' => right,
                _ => bail!("invalid direction: {}", dir),
            };

            if *curr % 26 == 25 {
                steps.push(i);
                break;
            }
        }
    }

    Ok(lcm(&steps))
}

fn lcm(s: &[usize]) -> usize {
    s.iter().fold(1, |lcm, &x| lcm * x / gcd(lcm, x))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (b, a) = (a % b, b);
    }
    a
}

solution!(part1 => 21251, part2 => 11678319315857);

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

    const EXAMPLE3: &str = "
LR

AAA = (AAB, XXX)
AAB = (XXX, AAZ)
AAZ = (AAB, XXX)
BBA = (BBB, XXX)
BBB = (BBC, BBC)
BBC = (BBZ, BBZ)
BBZ = (BBB, BBB)
XXX = (XXX, XXX)
";
    crate::test!(part1,
        t1: EXAMPLE1.trim() => 2,
        t2: EXAMPLE2.trim() => 6,
    );

    crate::test!(part2, t1: EXAMPLE3.trim() => 6);
}
