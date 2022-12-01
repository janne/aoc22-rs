use aoc_utils::{empty, read_lines, run, Result};

fn main() -> Result<()> {
    let xs = read_lines::<String>("day1/input")?;

    run(&xs, part_a, empty)
}

fn part_a(xs: &[String]) -> Result<i32> {
    let calories: Vec<i32> = xs.iter().fold([0].to_vec(), |mut acc, x| {
        if x.is_empty() {
            acc.insert(0, 0);
            return acc;
        }
        acc[0] += x.parse::<i32>().unwrap();
        acc
    });
    Ok(calories.into_iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero() {
        let input: Vec<String> = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        if let Ok(r) = part_a(&input) {
            assert_eq!(r, 24000);
        }
    }
}
