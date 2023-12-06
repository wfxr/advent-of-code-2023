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
            .any(|&(r, c)| self.is_symbol(row.wrapping_add_signed(r), col.wrapping_add_signed(c)))
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

fn part2(_input: &str) -> AocResult<usize> {
    todo!()
}

solution!(part1 => 529618, part2 => todo!());

#[cfg(test)]
mod tests {
    crate::test!(part1, t1: "
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
    ".trim() => 4361);

    crate::test!(part2, t1: "" => 42);
}
