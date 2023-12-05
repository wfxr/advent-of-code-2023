use crate::{solution, AocResult};

fn part1(input: &str) -> AocResult<usize> {
    input
        .lines()
        .map(|line| {
            let first = line.bytes().find(|b| b.is_ascii_digit()).map(|b| (b - b'0') as usize);
            let last = line.bytes().rfind(|b| b.is_ascii_digit()).map(|b| (b - b'0') as usize);
            match (first, last) {
                (Some(first), Some(last)) => Ok(first * 10 + last),
                _ => Err(format!("Invalid input: {}", line)),
            }
        })
        .sum::<Result<usize, _>>()
        .map_err(|e| e.into())
}

fn part2(input: &str) -> AocResult<usize> {
    input
        .lines()
        .map(|line| {
            let first = first_digit(line, false).ok_or("Invalid input")?;
            let last = first_digit(line, true).ok_or("Invalid input")?;
            Ok(first * 10 + last)
        })
        .sum::<Result<usize, &'static str>>()
        .map_err(|e| e.into())
}

fn first_digit(line: &str, reverse: bool) -> Option<usize> {
    let mut m = [0; 10];
    let line = line.as_bytes();
    for i in 0..line.len() {
        let c = if reverse { line[line.len() - 1 - i] } else { line[i] };
        if c.is_ascii_digit() {
            return Some(c as usize - '0' as usize);
        }
        for (j, w) in DIGIT_WORDS.iter().enumerate().skip(1) {
            let wc = if reverse { w[w.len() - 1 - m[j]] } else { w[m[j]] };
            let w0 = if reverse { w[w.len() - 1] } else { w[0] };
            m[j] = if wc == c {
                m[j] + 1
            } else if w0 == c {
                1
            } else {
                0
            };
            if m[j] == w.len() {
                return Some(j);
            }
        }
    }

    None
}

const DIGIT_WORDS: &[&[u8]] = &[
    b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

solution!(part1 => 54331, part2 => 54518);

#[cfg(test)]
mod tests {
    crate::test!(part1, t1: "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    ".trim() => 142);

    crate::test!(part2,
    t1: "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
    ".trim() => 281,

    t2: "oxonetwo1nine" => 19,
    t3: "jrvcznlvfgntthree5fivejqrheightwoxkh" => 32,
    t4: "tthree5five2" => 32,
    );
}
