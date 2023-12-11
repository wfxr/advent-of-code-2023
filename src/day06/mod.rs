use crate::{solution, AocResult};

fn part1(input: &str) -> AocResult<usize> {
    let (time, dis) = input.split_once('\n').ok_or("invalid input")?;

    let time: Vec<usize> = time
        .strip_prefix("Time:")
        .ok_or(format!("invalid time: {}", time))?
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let dis: Vec<usize> = dis
        .strip_prefix("Distance:")
        .ok_or(format!("invalid distance: {}", dis))?
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let res = time
        .into_iter()
        .zip(dis)
        .map(|(t, d)| succeed_ways(t, d))
        .product::<usize>();

    Ok(res)
}

fn part2(input: &str) -> AocResult<usize> {
    let (time, dis) = input.split_once('\n').ok_or("invalid input")?;

    let time: usize = time
        .strip_prefix("Time:")
        .ok_or(format!("invalid time: {}", time))?
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, c| acc * 10 + (c - b'0') as usize);
    let dis: usize = dis
        .strip_prefix("Distance:")
        .ok_or(format!("invalid distance: {}", dis))?
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, c| acc * 10 + (c - b'0') as usize);

    Ok(succeed_ways(time, dis))
}

fn succeed_ways(time: usize, dis: usize) -> usize {
    //    (x * 1) * (t - x) = d + 1
    // => x1 = (t - sqrt(t^2 - 4(d + 1))) / 2
    //    x2 = (t + sqrt(t^2 - 4(d + 1))) / 2
    let (t, d) = (time as f64, dis as f64);
    let t1 = (t - (t * t - 4.0 * (d + 1.0)).sqrt()) / 2.0;
    let t2 = (t + (t * t - 4.0 * (d + 1.0)).sqrt()) / 2.0;
    let t1 = t1.ceil() as usize;
    let t2 = t2.floor() as usize;
    t2 - t1 + 1
}

solution!(part1 => 449550, part2 => 28360140);

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "
Time:      7  15   30
Distance:  9  40  200
";

    crate::test!(part1, t1: EXAMPLE.trim() => 288);
    crate::test!(part2, t1: EXAMPLE.trim() => 71503);
}
