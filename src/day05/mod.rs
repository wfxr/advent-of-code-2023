use std::ops::Range;

use crate::{solution, AocResult};

struct MapEntry {
    range: Range<usize>,
    delta: isize,
}

impl MapEntry {
    fn new(src: usize, dst: usize, len: usize) -> Self {
        Self { range: src..src + len, delta: dst as isize - src as isize }
    }
    fn get(&self, i: usize) -> Option<usize> {
        self.range.contains(&i).then(|| i.wrapping_add_signed(self.delta))
    }
}

struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn get(&self, i: usize) -> usize {
        self.entries.iter().find_map(|e| e.get(i)).unwrap_or(i)
    }
}

struct MapChain {
    maps: Vec<Map>,
}

impl MapChain {
    fn push(&mut self, map: Map) {
        self.maps.push(map);
    }

    fn get(&self, i: usize) -> usize {
        self.maps.iter().fold(i, |i, map| map.get(i))
    }
}

fn part1(input: &str) -> AocResult<usize> {
    let mut parts = input.split("\n\n");

    let seeds: Vec<usize> = match parts.next() {
        Some(line) if line.starts_with("seeds:") => line[6..]
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?,
        _ => return Err("invalid input".into()),
    };

    let map_chain = parts.try_fold(MapChain { maps: Vec::new() }, |mut map_chain, part| -> AocResult<_> {
        let lines = &mut part.lines();
        match lines.next() {
            Some(line) if line.ends_with("map:") => {
                let entries = lines
                    .map(|line| {
                        let mut parts = line.split_ascii_whitespace();
                        match (parts.next(), parts.next(), parts.next(), parts.next()) {
                            (Some(dst), Some(src), Some(len), None) =>
                                Ok(MapEntry::new(src.parse()?, dst.parse()?, len.parse()?)),
                            _ => Err(format!("invalid line: {}", line))?,
                        }
                    })
                    .collect::<AocResult<_>>()?;
                map_chain.push(Map { entries });
                Ok(map_chain)
            }
            _ => Err(format!("invalid part: {}", part))?,
        }
    })?;

    seeds
        .iter()
        .map(|&seed| map_chain.get(seed))
        .min()
        .ok_or("no solution".into())
}

fn part2(_input: &str) -> AocResult<usize> {
    todo!()
}

solution!(part1 => 535088217, part2 => todo!());

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    crate::test!(part1, t1: EXAMPLE.trim() => 35);
    crate::test!(part2, t1: EXAMPLE.trim() => 42);
}
