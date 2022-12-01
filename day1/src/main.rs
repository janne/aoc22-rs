use aoc_utils::{read_lines, run, Result};

fn main() -> Result<()> {
    let xs = read_lines::<String>("day1/input")?;

    run(&xs, part_a, part_b)
}

fn part_a(xs: &[String]) -> Result<i32> {
    let calories = calories_per_elf(xs);
    Ok(calories.into_iter().max().unwrap())
}

fn part_b(xs: &[String]) -> Result<i32> {
    let mut calories = calories_per_elf(xs);
    calories.sort();
    Ok(calories.iter().rev().take(3).sum())
}

fn calories_per_elf(xs: &[String]) -> Vec<i32> {
    let calories: Vec<i32> = xs.iter().fold([0].to_vec(), |mut acc, x| {
        if x.is_empty() {
            acc.insert(0, 0);
            return acc;
        }
        acc[0] += x.parse::<i32>().unwrap();
        acc
    });
    calories
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn part_a_returns_top_elf() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, 24000);
        }
    }

    #[test]
    fn part_b_returns_top_tree_elves() {
        if let Ok(r) = part_b(&input()) {
            assert_eq!(r, 45000);
        }
    }
}
