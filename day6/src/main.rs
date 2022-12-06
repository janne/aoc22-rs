use std::{
    collections::{hash_map::RandomState, HashSet},
    fs,
};

use aoc_utils::{run, Result};

fn main() -> Result<()> {
    let xs: Vec<char> = fs::read_to_string("day6/input")?.chars().collect();

    run(&xs, part_a, part_b)
}

fn part_a(xs: &[char]) -> Result<i32> {
    let mut i = 0;
    loop {
        let cs = &xs[i..i + 4];
        if is_distinct(cs) {
            return Ok((i + 4) as i32);
        }
        i += 1;
    }
}

fn part_b(xs: &[char]) -> Result<i32> {
    let mut i = 0;
    loop {
        let cs = &xs[i..i + 14];
        if is_distinct(cs) {
            return Ok((i + 14) as i32);
        }
        i += 1;
    }
}

fn is_distinct(xs: &[char]) -> bool {
    let set: HashSet<&char, RandomState> = HashSet::from_iter(xs);
    set.len() == xs.len()
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

    #[test]
    fn solves_part_b() {
        let a: Vec<char> = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
        assert_eq!(part_b(&a).unwrap(), 19);
        let b: Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        assert_eq!(part_b(&b).unwrap(), 23);
        let c: Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        assert_eq!(part_b(&c).unwrap(), 23);
        let d: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        assert_eq!(part_b(&d).unwrap(), 29);
        let e: Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();
        assert_eq!(part_b(&e).unwrap(), 26);
    }
}
