use std::{cmp::Ordering, ops::Range};

use crate::{solution, AocResult};

#[derive(Debug)]
struct MapEntry {
    range: Range<usize>,
    delta: isize,
}

impl MapEntry {
    fn new(src: usize, dst: usize, len: usize) -> Self {
        Self { range: src..src + len, delta: dst as isize - src as isize }
    }

    // TODO: remove this
    fn mapped(&self, key: usize) -> Option<usize> {
        self.range.contains(&key).then(|| key.wrapping_add_signed(self.delta))
    }
}

struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn new(mut entries: Vec<MapEntry>) -> Self {
        entries.sort_unstable_by_key(|e| e.range.start);
        Self { entries }
    }

    fn mapped(&self, key: usize) -> usize {
        self.entries
            .binary_search_by(|MapEntry { range: Range { start, end }, .. }| match key {
                k if start > &k => Ordering::Greater,
                k if end <= &k => Ordering::Less,
                _ => Ordering::Equal,
            })
            .ok()
            .and_then(|i| self.entries[i].mapped(key))
            .unwrap_or(key)
    }

    fn mapped_ranges(&self, keys: Range<usize>) -> Vec<Range<usize>> {
        let (mut kstart, kend) = (keys.start, keys.end);
        let mut ranges = Vec::new();

        let mut i = match self
            .entries
            .binary_search_by(|MapEntry { range: Range { start, end }, .. }| match kstart {
                kstart if start > &kstart => Ordering::Greater,
                kstart if end <= &kstart => Ordering::Less,
                _ => Ordering::Equal,
            }) {
            Ok(i) => i,
            Err(i) => match self.entries.get(i) {
                Some(entry) => {
                    let len = (entry.range.start).min(kend) - kstart;
                    ranges.push(kstart..kstart + len);
                    kstart += len;
                    i
                }
                None => {
                    ranges.push(kstart..kend);
                    return ranges;
                }
            },
        };

        while i < self.entries.len() && kstart < kend {
            let entry = &self.entries[i];
            let vstart = entry.mapped(kstart).expect("invalid state");
            let len = (entry.range.end).min(kend) - kstart;
            ranges.push(vstart..vstart + len);
            kstart += len;

            i += 1;

            let len = match self.entries.get(i) {
                Some(entry) => (entry.range.start).min(kend) - kstart,
                None => kend - kstart,
            };
            ranges.push(kstart..kstart + len);
            kstart += len;
        }

        ranges.into_iter().filter(|range| !range.is_empty()).collect()
    }
}

struct MapChain {
    maps: Vec<Map>,
}

impl MapChain {
    fn mapped(&self, key: usize) -> usize {
        self.maps.iter().fold(key, |key, map| map.mapped(key))
    }

    fn mapped_ranges(&self, range: Range<usize>) -> Vec<Range<usize>> {
        self.maps.iter().fold(vec![range], |ranges, map| {
            ranges.into_iter().flat_map(|range| map.mapped_ranges(range)).collect()
        })
    }
}

fn parse_map_chain(input: &str) -> AocResult<MapChain> {
    input
        .split("\n\n")
        .try_fold(MapChain { maps: Vec::new() }, |mut map_chain, part| -> AocResult<_> {
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
                    map_chain.maps.push(Map::new(entries));
                    Ok(map_chain)
                }
                _ => Err(format!("invalid part: {}", part))?,
            }
        })
}

fn parse_seeds(input: &str) -> AocResult<impl Iterator<Item = Result<usize, std::num::ParseIntError>> + '_> {
    match input.strip_prefix("seeds:") {
        Some(seeds) => Ok(seeds.split_ascii_whitespace().map(|s| s.parse())),
        None => Err(format!("invalid seeds: {}", input))?,
    }
}

fn part1(input: &str) -> AocResult<usize> {
    let (seeds, map_chain) = input.split_once("\n\n").ok_or("invalid input")?;
    let map_chain = parse_map_chain(map_chain)?;

    let mut min_loc = usize::MAX;
    for seed in parse_seeds(seeds)? {
        min_loc = min_loc.min(map_chain.mapped(seed?));
    }
    Ok(min_loc)
}

fn part2(input: &str) -> AocResult<usize> {
    let (seeds, map_chain) = input.split_once("\n\n").ok_or("invalid input")?;
    let map_chain = parse_map_chain(map_chain)?;

    let mut min_loc = usize::MAX;
    for [start, count] in parse_seeds(seeds)?.array_chunks() {
        let (start, count) = (start?, count?);
        for range in map_chain.mapped_ranges(start..start + count) {
            min_loc = min_loc.min(range.start);
        }
    }
    Ok(min_loc)
}

solution!(part1 => 535088217, part2 => 51399228);

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
    crate::test!(part2, t1: EXAMPLE.trim() => 46);
}
