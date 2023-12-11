use crate::{solution, AocResult};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind:  HandKind,
    cards: [u8; 5],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn new(cards: [u8; 5]) -> Self {
        let mut counts = [0; 15];
        for &card in &cards {
            counts[card as usize] += 1;
        }

        let mut kind = HandKind::HighCard;
        for &count in counts.iter().skip(1) {
            match count {
                5 => {
                    kind = HandKind::FiveOfAKind;
                    break;
                }
                4 => {
                    kind = HandKind::FourOfAKind;
                    break;
                }
                3 => match kind {
                    HandKind::OnePair => {
                        kind = HandKind::FullHouse;
                        break;
                    }
                    _ => kind = HandKind::ThreeOfAKind,
                },
                2 => match kind {
                    HandKind::OnePair => {
                        kind = HandKind::TwoPair;
                        break;
                    }
                    HandKind::ThreeOfAKind => {
                        kind = HandKind::FullHouse;
                        break;
                    }
                    _ => kind = HandKind::OnePair,
                },
                _ => {}
            }
        }

        match counts[0] {
            0 => {}
            1 => match kind {
                HandKind::HighCard => kind = HandKind::OnePair,
                HandKind::OnePair => kind = HandKind::ThreeOfAKind,
                HandKind::TwoPair => kind = HandKind::FullHouse,
                HandKind::ThreeOfAKind => kind = HandKind::FourOfAKind,
                HandKind::FourOfAKind => kind = HandKind::FiveOfAKind,
                _ => {}
            },
            2 => match kind {
                HandKind::HighCard => kind = HandKind::ThreeOfAKind,
                HandKind::OnePair => kind = HandKind::FourOfAKind,
                HandKind::ThreeOfAKind => kind = HandKind::FiveOfAKind,
                _ => {}
            },
            3 => match kind {
                HandKind::HighCard => kind = HandKind::FourOfAKind,
                HandKind::OnePair => kind = HandKind::FiveOfAKind,
                _ => {}
            },
            4 | 5 => {
                kind = HandKind::FiveOfAKind;
            }
            _ => {}
        }

        Self { cards, kind }
    }
}

#[rustfmt::skip]
fn parse_hand(s: &str, jocker: bool) -> AocResult<Hand> {
    let mut cards = [0; 5];
    for (i, c) in s.chars().enumerate() {
        cards[i] = match c {
            '2'..='9' => c as u8 - b'0',
            'T' => 10,
            'J' => if jocker { 0 } else { 11 },
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => return Err(format!("Invalid card: {}", c).into()),
        };
    }
    Ok(Hand::new(cards))
}

fn solve(input: &str, jocker: bool) -> AocResult<usize> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').ok_or(format!("Invalid hand: {}", line))?;
            let bid: usize = bid.parse().map_err(|e| format!("Invalid bid: {}", e))?;
            let hand: Hand = parse_hand(hand, jocker)?;
            Ok((hand, bid))
        })
        .collect::<AocResult<_>>()?;
    hands.sort_unstable();
    Ok(hands.into_iter().enumerate().map(|(i, (_, bid))| bid * (i + 1)).sum())
}

fn part1(input: &str) -> AocResult<usize> {
    solve(input, false)
}

fn part2(input: &str) -> AocResult<usize> {
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
