use std::{cell::RefCell, collections::HashSet};

use aoc_utils::{read_lines, run, Result};

struct Moveable(i32, i32);

impl Moveable {
    fn new() -> Self {
        Moveable(0, 0)
    }

    fn exec(&mut self, direction: &str) {
        match direction {
            "R" => self.0 += 1,
            "U" => self.1 -= 1,
            "L" => self.0 -= 1,
            "D" => self.1 += 1,
            _ => (),
        }
    }

    fn follow(&mut self, head: &Moveable) {
        let x_diff = head.0 - self.0;
        let y_diff = head.1 - self.1;
        if x_diff.abs() > 1 || y_diff.abs() > 1 {
            if x_diff != 0 {
                self.0 += x_diff / x_diff.abs();
            }
            if y_diff != 0 {
                self.1 += y_diff / y_diff.abs();
            }
        }
    }
}

fn main() -> Result<()> {
    let xs = read_lines::<String>("day9/input")?;

    run(&xs, part_a, part_b)
}

fn part_a(lines: &[String]) -> Result<i32> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = Moveable::new();
    let mut tail = Moveable::new();
    for line in lines {
        let mut line_parts = line.split(' ');
        let dir = line_parts.next().unwrap();
        let distance = line_parts.next().unwrap();
        let distance = distance.parse::<usize>().unwrap();
        for _ in 0..distance {
            head.exec(dir);
            tail.follow(&head);
            visited.insert((tail.0, tail.1));
        }
    }
    Ok(visited.len() as i32)
}

fn part_b(lines: &[String]) -> Result<i32> {
    const COUNT: usize = 10;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let moveables: Vec<RefCell<Moveable>> =
        (0..COUNT).map(|_| RefCell::new(Moveable::new())).collect();
    for line in lines {
        let mut line_parts = line.split(' ');
        let dir = line_parts.next().unwrap();
        let distance = line_parts.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..distance {
            moveables[0].borrow_mut().exec(dir);
            for i in 1..COUNT {
                moveables[i].borrow_mut().follow(&moveables[i - 1].borrow());
            }
            let tail = moveables[COUNT - 1].borrow();
            visited.insert((tail.0, tail.1));
        }
    }
    Ok(visited.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_a() {
        let input = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"]
            .map(String::from)
            .to_vec();

        let r = part_a(&input).unwrap();

        assert_eq!(r, 13);
    }

    #[test]
    fn solves_part_b() {
        let input = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"]
            .map(String::from)
            .to_vec();

        let r = part_b(&input).unwrap();
        assert_eq!(r, 36);
    }
}
