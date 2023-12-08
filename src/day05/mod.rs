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

fn part1(input: &str) -> AocResult<usize> {
    let mut parts = input.split("\n\n");

    let seeds: Vec<usize> = match parts.next() {
        Some(line) if line.starts_with("seeds:") => line[6..]
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?,
        _ => return Err("invalid input".into()),
    };

    let mut seed_to_soil = None;
    let mut soil_to_fertilizer = None;
    let mut fertilizer_to_water = None;
    let mut water_to_light = None;
    let mut light_to_temperature = None;
    let mut temperature_to_humidity = None;
    let mut humidity_to_location = None;

    for part in parts {
        let lines = &mut part.lines();

        let build_map = |lines: &mut std::str::Lines<'_>| -> AocResult<Map> {
            let entries = lines
                .map(|line| {
                    let mut parts = line.split_ascii_whitespace();
                    match (parts.next(), parts.next(), parts.next(), parts.next()) {
                        (Some(dst), Some(src), Some(len), None) =>
                            Ok(MapEntry::new(src.parse()?, dst.parse()?, len.parse()?)),
                        _ => Err(format!("invalid line: {}", line).into()),
                    }
                })
                .collect::<AocResult<_>>()?;
            Ok(Map { entries })
        };

        match lines.next() {
            Some(line) if line.starts_with("seed-to-soil map:") => {
                seed_to_soil = Some(build_map(lines)?);
            }
            Some(line) if line.starts_with("soil-to-fertilizer map:") => {
                soil_to_fertilizer = Some(build_map(lines)?);
            }
            Some(line) if line.starts_with("fertilizer-to-water map:") => {
                fertilizer_to_water = Some(build_map(lines)?);
            }
            Some(line) if line.starts_with("water-to-light map:") => {
                water_to_light = Some(build_map(lines)?);
            }
            Some(line) if line.starts_with("light-to-temperature map:") => {
                light_to_temperature = Some(build_map(lines)?);
            }
            Some(line) if line.starts_with("temperature-to-humidity map:") => {
                temperature_to_humidity = Some(build_map(lines)?);
            }
            Some(line) if line.starts_with("humidity-to-location map:") => {
                humidity_to_location = Some(build_map(lines)?);
            }
            _ => return Err(format!("invalid part: {}", part).into()),
        }
    }

    let soil_map = seed_to_soil.ok_or("missing seed-to-soil map")?;
    let soil_to_fertilizer = soil_to_fertilizer.ok_or("missing soil-to-fertilizer map")?;
    let fertilizer_to_water = fertilizer_to_water.ok_or("missing fertilizer-to-water map")?;
    let water_to_light = water_to_light.ok_or("missing water-to-light map")?;
    let light_to_temperature = light_to_temperature.ok_or("missing light-to-temperature map")?;
    let temperature_to_humidity = temperature_to_humidity.ok_or("missing temperature-to-humidity map")?;
    let humidity_to_location = humidity_to_location.ok_or("missing humidity-to-location map")?;

    seeds
        .iter()
        .map(|&seed| {
            let soil = dbg!(soil_map.get(seed));
            let fertilizer = dbg!(soil_to_fertilizer.get(soil));
            let water = dbg!(fertilizer_to_water.get(fertilizer));
            let light = dbg!(water_to_light.get(water));
            let temperature = dbg!(light_to_temperature.get(light));
            let humidity = dbg!(temperature_to_humidity.get(temperature));
            humidity_to_location.get(humidity)
        })
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
