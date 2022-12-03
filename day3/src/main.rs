use aoc_utils::{empty, read_lines, run, Result};

fn main() -> Result<()> {
    let xs = read_lines::<String>("day3/input")?;

    run(&xs, part_a, empty)
}

fn part_a(xs: &[String]) -> Result<i32> {
    let mut result = 0;
    for x in xs {
        let left = &x[..x.len() / 2];
        let right = &x[x.len() / 2..];
        for c in left.chars() {
            if right.contains(c) {
                result += char_value(c);
                break;
            }
        }
    }
    Ok(result)
}

fn char_value(c: char) -> i32 {
    if c as u8 >= b'a' {
        return ((c as u8) - b'a' + 1) as i32;
    }
    ((c as u8) - b'A' + 27) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn solves_part_a() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, 157);
        }
    }
}
