use std::collections::HashMap;

use crate::{solution, AocResult};

struct Grid<'a> {
    data:  Vec<&'a [u8]>,
    nrows: usize,
    ncols: usize,
}

#[rustfmt::skip]
const ADJS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> AocResult<Self> {
        let data = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
        let nrows = data.len();
        let cols = data.first().ok_or("empty input")?.len();
        for (i, row) in data.iter().enumerate() {
            if row.len() != cols {
                return Err(format!("rows have different lengths at row {}: {} != {}", i, row.len(), cols).into());
            }
        }
        Ok(Self { data, nrows, ncols: cols })
    }

    fn is_symbol(&self, row: usize, col: usize) -> bool {
        match self.data.get(row).and_then(|row| row.get(col)) {
            Some(c) => !c.is_ascii_digit() && c != &b'.',
            None => false,
        }
    }

    fn adjacent_to_symbol(&self, row: usize, col: usize) -> bool {
        ADJS.iter()
            .any(|&(i, j)| self.is_symbol(row.wrapping_add_signed(i), col.wrapping_add_signed(j)))
    }

    fn num_at(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row >= self.nrows || col >= self.ncols || !self.data[row][col].is_ascii_digit() {
            return None;
        }

        let num_start = match self.data[row][..col].iter().rev().position(|c| !c.is_ascii_digit()) {
            Some(n) => col - n,
            None => 0,
        };

        let num = self.data[row][num_start..]
            .iter()
            .take_while(|c| c.is_ascii_digit())
            .fold(0, |acc, &c| acc * 10 + (c - b'0') as usize);

        Some((row * self.ncols + num_start, num))
    }

    fn gear_ratio(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.nrows || col >= self.ncols || self.data[row][col] != b'*' {
            return None;
        }

        let nums: HashMap<usize, usize> = ADJS
            .iter()
            .filter_map(|&(i, j)| self.num_at(row.wrapping_add_signed(i), col.wrapping_add_signed(j)))
            .collect();

        match nums.len() {
            2 => Some(nums.values().product()),
            _ => None,
        }
    }
}

fn part1(input: &str) -> AocResult<usize> {
    let grid = Grid::new(input)?;

    let mut sum = 0;
    for i in 0..grid.nrows {
        let mut num = 0;
        let mut is_part_num = false;
        for j in 0..grid.ncols {
            let c = grid.data[i][j];
            if c.is_ascii_digit() {
                num = num * 10 + (c - b'0') as usize;
                is_part_num |= grid.adjacent_to_symbol(i, j);
            } else {
                if is_part_num {
                    sum += num;
                }
                num = 0;
                is_part_num = false;
            }
        }

        if is_part_num {
            sum += num;
        }
    }

    Ok(sum)
}

fn part2(input: &str) -> AocResult<usize> {
    let grid = Grid::new(input)?;

    let sum = (0..grid.nrows)
        .flat_map(|i| (0..grid.ncols).map(move |j| (i, j)))
        .filter_map(|(i, j)| grid.gear_ratio(i, j))
        .sum();

    Ok(sum)
}

solution!(part1 => 529618, part2 => 77509019);

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    crate::test!(part1, t1: EXAMPLE.trim() => 4361);

    crate::test!(part2, t1: EXAMPLE.trim() => 467835);
}
