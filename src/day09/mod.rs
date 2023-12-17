use crate::*;

fn part1(input: &str) -> Result<usize> {
    solve(input, false)
}

fn part2(input: &str) -> Result<usize> {
    solve(input, true)
}

fn solve(input: &str, reverse: bool) -> Result<usize> {
    input
        .lines()
        .map(|line| {
            let iter = line.split_ascii_whitespace().map(|n| n.parse());
            let history: Vec<_> = match reverse {
                true => iter.rev().try_collect()?,
                false => iter.try_collect()?,
            };
            predict(&history)
        })
        .sum::<Result<i32>>()
        .and_then(|x| usize::try_from(x).map_err(Into::into))
}

fn predict(history: &[i32]) -> Result<i32> {
    let diff = history.array_windows().map(|[a, b]| b - a).collect_vec();
    let diff = match diff.iter().all_equal_value() {
        Ok(&x) => x,
        Err(Some(..)) => predict(&diff)?,
        Err(None) => bail!("history should have at least 2 elements"),
    };
    Ok(history[history.len() - 1] + diff)
}

solution!(part1 => 1868368343, part2 => 1022);

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = indoc::indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    crate::test!(part1, t1: EXAMPLE => 114);
    crate::test!(part2, t1: EXAMPLE => 2);
}
