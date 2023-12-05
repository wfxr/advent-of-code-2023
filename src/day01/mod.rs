use crate::{solution, AocResult};

fn resolve<F1, F2>(input: &str, first_digit: F1, last_digit: F2) -> AocResult<usize>
where
    F1: Fn(&str) -> Option<usize>,
    F2: Fn(&str) -> Option<usize>,
{
    input
        .lines()
        .map(|line| match (first_digit(line), last_digit(line)) {
            (Some(first), Some(last)) => Ok(first * 10 + last),
            _ => Err(format!("Invalid input: {}", line)),
        })
        .sum::<Result<usize, _>>()
        .map_err(|e| e.into())
}

fn part1(input: &str) -> AocResult<usize> {
    resolve(
        input,
        |line| line.bytes().find(|b| b.is_ascii_digit()).map(|b| (b - b'0') as usize),
        |line| line.bytes().rfind(|b| b.is_ascii_digit()).map(|b| (b - b'0') as usize),
    )
}

fn part2(input: &str) -> AocResult<usize> {
    resolve(input, |line| find_digit(line, false), |line| find_digit(line, true))
}

const DIGIT_WORDS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn find_digit(line: &str, reverse: bool) -> Option<usize> {
    let mut m = [0; 10];
    let line = line.as_bytes();
    for i in 0..line.len() {
        let lc = if reverse { line[line.len() - 1 - i] } else { line[i] };
        if lc.is_ascii_digit() {
            return Some(lc as usize - '0' as usize);
        }
        for (x, w) in DIGIT_WORDS.iter().enumerate() {
            let wc = if reverse { w[w.len() - 1 - m[x]] } else { w[m[x]] };
            let w0 = if reverse { w[w.len() - 1] } else { w[0] };
            m[x] = match lc == wc {
                true => m[x] + 1,
                false => (lc == w0) as usize,
            };
            if m[x] == w.len() {
                return Some(x + 1);
            }
        }
    }

    None
}

solution!(part1 => 54331, part2 => 54518);

#[cfg(test)]
mod tests {
    crate::test!(part1,
    t1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ".trim() => 142);

    crate::test!(part2,
    t1: "oxonetwo1nine" => 19,
    t2: "jrvcznlvfgntthree5fivejqrheightwoxkh" => 32,
    t3: "tthree5five2" => 32,
    t4: "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    ".trim() => 281,
    );
}
