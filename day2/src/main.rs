use aoc_utils::{read_lines, run, Result};

fn main() -> Result<()> {
    let xs = read_lines::<String>("day2/input")?;
    run(&xs, part_a, part_b)
}

fn make_round(x: &str) -> (char, char) {
    let chars: Vec<&str> = x.split(' ').collect();
    (
        chars[0].chars().next().unwrap(),
        chars[1].chars().next().unwrap(),
    )
}

fn shape_point((_, me): (char, char)) -> i32 {
    match me {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid shape"),
    }
}

fn outcome_point((you, me): (char, char)) -> i32 {
    match (you, me) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        _ => 0,
    }
}

fn part_a(xs: &[String]) -> Result<i32> {
    let mut sum = 0;
    for x in xs {
        let round = make_round(x);
        let point = shape_point(round) + outcome_point(round);
        sum += point
    }
    Ok(sum)
}

fn part_b(xs: &[String]) -> Result<i32> {
    let mut sum = 0;
    for x in xs {
        let round = make_round(x);
        let adjusted = adjust_round(round);
        let point = shape_point(adjusted) + outcome_point(adjusted);
        sum += point
    }
    Ok(sum)
}

fn adjust_round((you, me): (char, char)) -> (char, char) {
    match (you, me) {
        ('A', 'X') => ('A', 'Z'),
        ('B', 'X') => ('B', 'X'),
        ('C', 'X') => ('C', 'Y'),
        ('A', 'Y') => ('A', 'X'),
        ('B', 'Y') => ('B', 'Y'),
        ('C', 'Y') => ('C', 'Z'),
        ('A', 'Z') => ('A', 'Y'),
        ('B', 'Z') => ('B', 'Z'),
        ('C', 'Z') => ('C', 'X'),
        _ => (you, me),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec!["A Y", "B X", "C Z"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_part_a() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, 15);
        }
    }

    #[test]
    fn test_part_b() {
        if let Ok(r) = part_b(&input()) {
            assert_eq!(r, 12);
        }
    }
}
