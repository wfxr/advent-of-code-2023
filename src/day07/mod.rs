use crate::*;
use std::cmp::Reverse;

#[rustfmt::skip]
fn compute_rank(s: &str, jocker: bool) -> Result<u32> {
    let mut cards = [0; 5];
    for (i, c) in s.chars().enumerate() {
        cards[i] = match c {
            '2'..='9' => c as u8 - b'0',
            'T' => 10,
            'J' => if jocker { 0 } else { 11 },
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => bail!("Invalid card: {}", c),
        };
    }
    Ok(cards_rank(&cards))
}

fn cards_rank(cards: &[u8; 5]) -> u32 {
    let mut rank = 0;
    let mut counts = [0; 15];
    for &card in cards {
        counts[card as usize] += 1;
        rank = rank * 15 + card as u32;
    }

    counts[1..].sort_unstable_by_key(|&c| Reverse(c));
    let [jokers, first, second, ..] = counts;

    let kind = (jokers + first) * 5 + second;
    kind * 15u32.pow(5) + rank
}

fn solve(input: &str, jocker: bool) -> Result<usize> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').ok_or_else(|| anyhow!("Invalid hand: {}", line))?;
            let bid: usize = bid.parse().map_err(|e| anyhow!("Invalid bid: {}", e))?;
            let rank = compute_rank(hand, jocker)?;
            Ok((rank, bid))
        })
        .collect::<Result<_>>()?;
    hands.sort_unstable();
    Ok(hands.into_iter().zip(1..).map(|((_, bid), i)| bid * i).sum())
}

fn part1(input: &str) -> Result<usize> {
    solve(input, false)
}

fn part2(input: &str) -> Result<usize> {
    solve(input, true)
}

solution!(part1 => 251927063, part2 => 255632664);

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    crate::test!(part1, t1: EXAMPLE.trim() => 6440);
    crate::test!(part2, t1: EXAMPLE.trim() => 5905);
}
