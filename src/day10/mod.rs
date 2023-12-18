use crate::*;

type Direction = (isize, isize);
const L: Direction = (0, -1);
const R: Direction = (0, 1);
const U: Direction = (-1, 0);
const D: Direction = (1, 0);
const DIRS: &[Direction] = &[L, R, U, D];

struct Maze<'a> {
    grids: Vec<&'a [u8]>,
    nrows: usize,
    ncols: usize,
}

impl<'a> Maze<'a> {
    fn get(&self, (row, col): (usize, usize)) -> Option<u8> {
        (row < self.nrows && col < self.ncols).then(|| self.grids[row][col])
    }

    fn loop_length(&self, start: (usize, usize)) -> Option<usize> {
        DIRS.iter().find_map(|&dir| {
            let (mut dir, mut pos, mut steps) = (dir, start, 0);
            loop {
                pos = forward(pos, dir);
                steps += 1;
                if pos == start {
                    break;
                }

                dir = match (dir, self.get(pos)?) {
                    (L, b'F') | (R, b'7') | (D, b'|') => D,
                    (R, b'J') | (L, b'L') | (U, b'|') => U,
                    (U, b'F') | (D, b'L') | (R, b'-') => R,
                    (U, b'7') | (D, b'J') | (L, b'-') => L,
                    _ => None?,
                };
            }
            Some(steps)
        })
    }

    fn loop_enclosed(&self, start: (usize, usize)) -> Option<usize> {
        DIRS.iter().find_map(|&dir| {
            let (mut dir, mut pos) = (dir, start);
            let mut grids = vec![vec![b'I'; self.ncols * 2]; self.nrows * 2];

            loop {
                grids[pos.0 * 2][pos.1 * 2] = b'#';
                let interpos = match dir {
                    L => (pos.0 * 2, pos.1 * 2 - 1),
                    R => (pos.0 * 2, pos.1 * 2 + 1),
                    U => (pos.0 * 2 - 1, pos.1 * 2),
                    D => (pos.0 * 2 + 1, pos.1 * 2),
                    _ => unreachable!(),
                };
                grids[interpos.0][interpos.1] = b'#';

                pos = forward(pos, dir);
                if pos == start {
                    break;
                }

                dir = match (dir, self.get(pos)?) {
                    (L, b'F') | (R, b'7') | (D, b'|') => D,
                    (R, b'J') | (L, b'L') | (U, b'|') => U,
                    (U, b'F') | (D, b'L') | (R, b'-') => R,
                    (U, b'7') | (D, b'J') | (L, b'-') => L,
                    _ => None?,
                };
            }

            (0..self.nrows * 2)
                .cartesian_product([0, self.ncols * 2 - 1])
                .chain([0, self.nrows * 2 - 1].into_iter().cartesian_product(0..self.ncols * 2))
                .for_each(|pos| dfs(&mut grids, pos));

            grids
                .into_iter()
                .step_by(2)
                .flat_map(|row| row.into_iter().step_by(2))
                .filter(|&x| x == b'I')
                .count()
                .into()
        })
    }
}

fn dfs(grids: &mut [Vec<u8>], (row, col): (usize, usize)) {
    if row >= grids.len() || col >= grids[row].len() || grids[row][col] != b'I' {
        return;
    }

    grids[row][col] = b' ';
    DIRS.iter()
        .map(|&dir| forward((row, col), dir))
        .for_each(|(row, col)| dfs(grids, (row, col)));
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
        .find(|&(r, c)| grids[r][c] == b'S')
        .ok_or_else(|| anyhow!("no start found"))?;

    let maze = Maze { grids, nrows, ncols };

    maze.loop_length(start).map(|n| n / 2).ok_or(anyhow!("no loop found"))
}

fn part2(input: &str) -> Result<usize> {
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

    let maze = Maze { grids, nrows, ncols };

    maze.loop_enclosed(start).ok_or(anyhow!("no loop found"))
}

solution!(part1 => 6778, part2 => 433);

#[cfg(test)]
mod tests {
    const EXAMPLE1: &str = indoc::indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    const EXAMPLE2: &str = indoc::indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    const EXAMPLE3: &str = indoc::indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    "};

    const EXAMPLE4: &str = indoc::indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    const EXAMPLE5: &str = indoc::indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    crate::test!(part1, t1: EXAMPLE1 => 4);

    crate::test!(part2,
        t1: EXAMPLE1 => 1,
        t2: EXAMPLE2 => 4,
        t3: EXAMPLE3 => 4,
        t4: EXAMPLE4 => 8,
        t5: EXAMPLE5 => 10,
    );
}
