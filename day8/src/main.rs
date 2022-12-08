use aoc_utils::{empty, read_lines, run, Result};

fn main() -> Result<()> {
    let xs = read_lines::<String>("day8/input")?;

    run(&xs, part_a, empty)
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
            if up(&trees, x, y) || right(&trees, x, y) || down(&trees, x, y) || left(&trees, x, y) {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn left(trees: &Trees, x: usize, y: usize) -> bool {
    let height = trees.at(x, y);
    for i in 0..x {
        if trees.at(i, y) >= height {
            return false;
        }
    }
    return true;
}

fn right(trees: &Trees, x: usize, y: usize) -> bool {
    let height = trees.at(x, y);
    for i in x + 1..trees.width() {
        if trees.at(i, y) >= height {
            return false;
        }
    }
    return true;
}

fn down(trees: &Trees, x: usize, y: usize) -> bool {
    let height = trees.at(x, y);
    for i in y + 1..trees.height() {
        if trees.at(x, i) >= height {
            return false;
        }
    }
    return true;
}

fn up(trees: &Trees, x: usize, y: usize) -> bool {
    let height = trees.at(x, y);
    for i in 0..y {
        if trees.at(x, i) >= height {
            return false;
        }
    }
    return true;
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
}
