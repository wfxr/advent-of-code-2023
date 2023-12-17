use crate::*;

type Direction = (isize, isize);
const L: Direction = (0, -1);
const R: Direction = (0, 1);
const U: Direction = (-1, 0);
const D: Direction = (1, 0);

struct Maze<'a> {
    grids: Vec<&'a [u8]>,
    nrows: usize,
    ncols: usize,
    start: (usize, usize),
}

impl<'a> Maze<'a> {
    fn get(&self, (row, col): (usize, usize)) -> Option<u8> {
        (row < self.nrows && col < self.ncols).then(|| self.grids[row][col])
    }

    fn route(&self, mut pos: (usize, usize), mut dir: Direction) -> Option<usize> {
        let mut steps = 1;
        while pos != self.start {
            dir = match (dir, self.get(pos)?) {
                (L, b'F') | (R, b'7') | (D, b'|') => D,
                (R, b'J') | (L, b'L') | (U, b'|') => U,
                (U, b'F') | (D, b'L') | (R, b'-') => R,
                (U, b'7') | (D, b'J') | (L, b'-') => L,
                _ => return None,
            };
            steps += 1;
            pos = forward(pos, dir);
        }
        Some(steps)
    }
}

fn forward(pos: (usize, usize), dir: Direction) -> (usize, usize) {
    (pos.0.wrapping_add_signed(dir.0), pos.1.wrapping_add_signed(dir.1))
}

fn part1(input: &str) -> Result<usize> {
    let grids = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let nrows = grids.len();
    let ncols = match grids.iter().map(|line| line.len()).all_equal_value() {
        Ok(ncols) => ncols,
        _ => bail!("invalid input"),
    };

    let start = (0..nrows)
        .cartesian_product(0..ncols)
        .find(|&(i, j)| grids[i][j] == b'S')
        .ok_or_else(|| anyhow!("no start found"))?;

    let maze = Maze { grids, nrows, ncols, start };

    let steps = [L, R, U, D]
        .into_iter()
        .find_map(|dir| maze.route(forward(start, dir), dir))
        .ok_or_else(|| anyhow!("no loop found"))?;

    Ok(steps / 2)
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => 6778, part2 => todo!());

#[cfg(test)]
mod tests {
    const EXAMPLE1: &str = indoc::indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    crate::test!(part1, t1: EXAMPLE1 => 4);
    crate::test!(part2, t1: EXAMPLE1 => todo!());
}
