use std::fs;

use aoc_utils::{empty, run, Result};

fn main() -> Result<()> {
    let xs: Vec<char> = fs::read_to_string("day6/input")?.chars().collect();

    run(&xs, part_a, empty)
}

fn part_a(xs: &[char]) -> Result<i32> {
    let mut i = 3;
    loop {
        let (a, b, c, d) = (xs[i - 3], xs[i - 2], xs[i - 1], xs[i]);
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return Ok((i + 1) as i32);
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_a() {
        let a: Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        assert_eq!(part_a(&a).unwrap(), 5);
        let b: Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        assert_eq!(part_a(&b).unwrap(), 6);
        let c: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        assert_eq!(part_a(&c).unwrap(), 10);
        let d: Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();
        assert_eq!(part_a(&d).unwrap(), 11);
    }
}
