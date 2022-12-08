use aoc_utils::{read_lines, run, Result};

fn main() -> Result<()> {
    let xs = read_lines::<String>("day8/input")?;

    run(&xs, part_a, part_b)
}

struct Trees {
    lines: Vec<String>,
}

impl Trees {
    fn new(lines: &[String]) -> Self {
        Trees {
            lines: lines.to_vec(),
        }
    }

    fn height(self: &Self) -> usize {
        self.lines.len()
    }

    fn width(self: &Self) -> usize {
        self.lines[0].len()
    }

    fn at(self: &Self, x: usize, y: usize) -> usize {
        self.lines[y].chars().nth(x).unwrap() as usize
    }

    fn perimeter(self: &Self) -> usize {
        self.height() * 2 + self.width() * 2 - 4
    }
}

fn part_a(lines: &[String]) -> Result<i32> {
    let trees = Trees::new(lines);

    let mut count = trees.perimeter() as i32;

    for y in 1..trees.height() - 1 {
        for x in 1..trees.width() - 1 {
            if up(&trees, x, y).is_none()
                || right(&trees, x, y).is_none()
                || down(&trees, x, y).is_none()
                || left(&trees, x, y).is_none()
            {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn part_b(lines: &[String]) -> Result<i32> {
    let trees = Trees::new(lines);

    let mut score: usize = 0;

    for y in 1..trees.height() - 1 {
        for x in 1..trees.width() - 1 {
            let tree_score = up(&trees, x, y).unwrap_or(y)
                * right(&trees, x, y).unwrap_or(trees.width() - x - 1)
                * down(&trees, x, y).unwrap_or(trees.height() - y - 1)
                * left(&trees, x, y).unwrap_or(x);
            if tree_score > score {
                score = tree_score;
            }
        }
    }

    Ok(score as i32)
}

fn left(trees: &Trees, x: usize, y: usize) -> Option<usize> {
    let height = trees.at(x, y);
    for (distance, i) in (0..x).rev().enumerate() {
        if trees.at(i, y) >= height {
            return Some(distance + 1);
        }
    }
    None
}

fn right(trees: &Trees, x: usize, y: usize) -> Option<usize> {
    let height = trees.at(x, y);
    for (distance, i) in (x + 1..trees.width()).enumerate() {
        if trees.at(i, y) >= height {
            return Some(distance + 1);
        }
    }
    None
}

fn down(trees: &Trees, x: usize, y: usize) -> Option<usize> {
    let height = trees.at(x, y);
    for (distance, i) in (y + 1..trees.height()).enumerate() {
        if trees.at(x, i) >= height {
            return Some(distance + 1);
        }
    }
    None
}

fn up(trees: &Trees, x: usize, y: usize) -> Option<usize> {
    let height = trees.at(x, y);
    for (distance, i) in (0..y).rev().enumerate() {
        if trees.at(x, i) >= height {
            return Some(distance + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec!["30373", "25512", "65332", "33549", "35390"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    #[test]
    fn solves_part_a() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, 21);
        }
    }

    #[test]
    fn solves_part_b() {
        if let Ok(r) = part_b(&input()) {
            assert_eq!(r, 8);
        }
    }
}
