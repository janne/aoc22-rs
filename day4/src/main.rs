use std::ops::Range;

use aoc_utils::{empty, read_lines, run, Result};

fn parse_line(line: &String) -> (Range<i32>, Range<i32>) {
    let xs: Vec<_> = line
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        .map(|r| {
            let pair: Vec<_> = r.split('-').map(|i| i.parse::<i32>().unwrap()).collect();
            pair[0]..pair[1]
        })
        .collect();
    (xs[0].clone(), xs[1].clone())
}

fn is_fully_overlapping((a, b): &(Range<i32>, Range<i32>)) -> bool {
    (a.start >= b.start && a.end <= b.end) || (b.start >= a.start && b.end <= a.end)
}

fn main() -> Result<()> {
    let xs = read_lines::<String>("day4/input")?;

    run(&xs, part_a, empty)
}

fn part_a(xs: &[String]) -> Result<i32> {
    let pairs: Vec<(Range<i32>, Range<i32>)> = xs.iter().map(parse_line).collect();
    Ok(pairs.iter().filter(|x| is_fully_overlapping(*x)).count() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        [
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn solves_part_a() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, 2);
        }
    }
}
