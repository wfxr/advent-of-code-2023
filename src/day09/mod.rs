use crate::*;

#[derive(Clone, Copy)]
enum PredictMode {
    Forward,
    Backward,
}

fn part1(input: &str) -> Result<usize> {
    solve(input, PredictMode::Forward)
}

fn part2(input: &str) -> Result<usize> {
    solve(input, PredictMode::Backward)
}

fn solve(input: &str, mode: PredictMode) -> Result<usize> {
    let res = input
        .lines()
        .map(|line| {
            let history: Vec<_> = line.split_ascii_whitespace().map(|n| n.parse()).try_collect()?;
            predict(&history, mode)
        })
        .sum::<Result<i32>>()?
        .try_into()?;
    Ok(res)
}

fn predict(history: &[i32], mode: PredictMode) -> Result<i32> {
    let diff = history.array_windows().map(|[a, b]| b - a).collect_vec();
    let diff = match diff.iter().all_equal_value() {
        Ok(&x) => x,
        Err(Some(..)) => predict(&diff, mode)?,
        Err(None) => bail!("history should have at least 2 elements"),
    };
    match mode {
        PredictMode::Forward => Ok(history[history.len() - 1] + diff),
        PredictMode::Backward => Ok(history[0] - diff),
    }
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
