use regex::Regex;
use std::{cell::RefCell, fs};

use aoc_utils::Result;

#[derive(Debug)]
struct Monkey {
    id: u128,
    items: Vec<u128>,
    operation: char,
    operator: Option<u128>,
    divisor: u128,
    monkey_true: u128,
    monkey_false: u128,
    visited: u128,
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("day11/input")?;

    println!("Part A: {}", part_a(&contents)?);
    println!("Part B: {}", part_b(&contents)?);

    Ok(())
}

fn parse(contents: &str) -> Vec<RefCell<Monkey>> {
    let re = Regex::new(
        r"Monkey (\d+):
  Starting items: (.*)
  Operation: new = old ([+*]) (\d+|old)
  Test: divisible by (\d+)
    If true: throw to monkey (\d)
    If false: throw to monkey (\d)",
    )
    .unwrap();

    re.captures_iter(&contents)
        .map(|cap| {
            RefCell::new(Monkey {
                id: cap[1].parse().unwrap(),
                items: cap[2]
                    .split(',')
                    .map(|s| s.trim().parse::<u128>().unwrap())
                    .collect(),
                operation: cap[3].chars().nth(0).unwrap(),
                operator: cap[4].parse().ok(),
                divisor: cap[5].parse().unwrap(),
                monkey_true: cap[6].parse().unwrap(),
                monkey_false: cap[7].parse().unwrap(),
                visited: 0,
            })
        })
        .collect()
}

fn part_a(contents: &str) -> Result<u128> {
    let mut monkeys = parse(contents);
    for _round in 0..20 {
        for monkey in monkeys.iter() {
            while !monkey.borrow().items.is_empty() {
                let item = monkey.borrow_mut().items.remove(0);
                let worry_level = monkey.borrow().operator.unwrap_or(item);
                let worry_level = match monkey.borrow().operation {
                    '*' => item * worry_level,
                    '+' => item + worry_level,
                    _ => panic!("Unknown operation"),
                };
                let worry_level = (worry_level as f64 / 3.0).floor() as u128;
                let to_monkey = if worry_level % monkey.borrow().divisor == 0 {
                    monkey.borrow().monkey_true
                } else {
                    monkey.borrow().monkey_false
                };
                let to_monkey = monkeys
                    .iter()
                    .find(|m| (*m).borrow().id == to_monkey)
                    .unwrap();
                to_monkey.borrow_mut().items.push(worry_level);
                monkey.borrow_mut().visited += 1;
            }
        }
    }
    monkeys.sort_by(|a, b| b.borrow().visited.cmp(&a.borrow().visited));
    let a = monkeys[0].borrow().visited;
    let b = monkeys[1].borrow().visited;
    Ok(a * b)
}

fn part_b(contents: &str) -> Result<u128> {
    let mut monkeys = parse(contents);
    let super_modulo = monkeys
        .iter()
        .map(|m| m.borrow().divisor)
        .reduce(|a, b| a * b)
        .unwrap();
    for _round in 0..10_000 {
        for monkey in monkeys.iter() {
            while !monkey.borrow().items.is_empty() {
                let item = monkey.borrow_mut().items.remove(0) % super_modulo;
                let worry_level = monkey.borrow().operator.unwrap_or(item);
                let worry_level = match monkey.borrow().operation {
                    '*' => item * worry_level,
                    '+' => item + worry_level,
                    _ => panic!("Unknown operation"),
                };
                // let worry_level = (worry_level as f64 / 3.0).floor() as u128;
                let to_monkey = if worry_level % monkey.borrow().divisor == 0 {
                    monkey.borrow().monkey_true
                } else {
                    monkey.borrow().monkey_false
                };
                let to_monkey = monkeys
                    .iter()
                    .find(|m| (*m).borrow().id == to_monkey)
                    .unwrap();
                to_monkey.borrow_mut().items.push(worry_level);
                monkey.borrow_mut().visited += 1;
            }
        }
    }
    monkeys.sort_by(|a, b| b.borrow().visited.cmp(&a.borrow().visited));
    let a = monkeys[0].borrow().visited;
    let b = monkeys[1].borrow().visited;
    Ok(a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            .to_owned()
    }

    #[test]
    fn solves_part_a() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, 10605);
        }
    }
    #[test]
    fn solves_part_b() {
        if let Ok(r) = part_b(&input()) {
            assert_eq!(r, 2713310158);
        }
    }
}
