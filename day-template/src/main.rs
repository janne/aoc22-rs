use aoc_utils::{empty, read_lines, run, Result};

fn main() -> Result<()> {
    let xs = read_lines::<i32>("day-template/input")?;

    run(&xs, part_a, empty)
}

fn part_a(xs: &[i32]) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero() {
        if let Ok(r) = part_a(&vec![1, 2, 3]) {
            assert_eq!(r, 0);
        }
    }
}
