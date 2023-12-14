use crate::*;

fn part1(input: &str) -> Result<usize> {
    my_won(input)
        .filter(|n| matches!(n, Ok(n) if *n > 0))
        .map(|n| n.map(|n| 2usize.pow(n - 1)))
        .sum()
}

fn part2(input: &str) -> Result<usize> {
    let w: Vec<_> = my_won(input).try_collect()?;
    let mut cards = vec![1usize; w.len()];

    for i in 0..w.len() {
        for j in 1..=w[i] {
            cards[i + j as usize] += cards[i];
        }
    }

    Ok(cards.into_iter().sum::<usize>())
}

fn my_won(input: &str) -> impl Iterator<Item = Result<u32>> + '_ {
    input.lines().map(|line| {
        let mut parts = line.split(|c| c == ':' || c == '|').skip(1);
        match (parts.next(), parts.next(), parts.next()) {
            (Some(wins), Some(mine), None) => {
                let wins: HashSet<usize> = wins.split_ascii_whitespace().map(|n| n.parse()).try_collect()?;
                mine.split_ascii_whitespace()
                    .map(|n| n.parse())
                    .try_fold(0, |acc, num| Ok(acc + wins.contains(&num?) as u32))
            }
            _ => bail!("Invalid input: {}", line),
        }
    })
}

solution!(part1 => 23235, part2 => 5920640);

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = indoc::indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    crate::test!(part1, t1: EXAMPLE => 13);
    crate::test!(part2, t1: EXAMPLE => 30);
}
