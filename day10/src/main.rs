use std::io::Write;

use aoc_utils::{empty, read_lines, run, Result};

struct Cpu {
    x: i32,
    cycles: Vec<i32>,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            x: 1,
            cycles: Vec::new(),
        }
    }

    fn add_cycles(&mut self, count: i32) {
        for _ in 0..count {
            self.cycles.push(self.x);
        }
    }

    fn signal_strength(&self) -> i32 {
        self.cycles[20 - 1] * 20
            + self.cycles[60 - 1] * 60
            + self.cycles[100 - 1] * 100
            + self.cycles[140 - 1] * 140
            + self.cycles[180 - 1] * 180
            + self.cycles[220 - 1] * 220
    }

    fn process(&mut self, ops: &[Op]) {
        for op in ops {
            match op {
                Op::Noop => self.add_cycles(1),
                Op::Addx(v) => {
                    self.add_cycles(2);
                    self.x += v;
                }
            }
        }
    }
}

enum Op {
    Noop,
    Addx(i32),
}

impl Op {
    fn new(line: &str) -> Op {
        let mut parts = line.split(' ');
        let op = parts.next().unwrap();
        let v = parts.next();
        match op {
            "noop" => Op::Noop,
            "addx" => Op::Addx(v.unwrap().parse().unwrap()),
            _ => panic!("Unknown op code!"),
        }
    }
}

fn main() -> Result<()> {
    let lines = read_lines::<String>("day10/input")?;

    run(&lines, part_a, empty)
}

fn part_a(lines: &[String]) -> Result<i32> {
    let ops: Vec<Op> = parse(&lines);
    let mut cpu = Cpu::new();
    cpu.process(&ops);

    for line in 0i32..6 {
        for col in 0i32..40 {
            let cycle = line * 40 + col;
            let x = cpu.cycles[cycle as usize];
            let char = if col >= x - 1 && col <= x + 1 {
                "#"
            } else {
                "."
            };
            print!("{}", char);
        }
        println!("");
    }
    std::io::stdout().flush().unwrap();

    Ok(cpu.signal_strength())
}

fn parse(input: &[String]) -> Vec<Op> {
    input
        .into_iter()
        .map(|line| Op::new(line))
        .collect::<Vec<Op>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        [
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ]
        .map(String::from)
        .to_vec()
    }

    #[test]
    fn solves_part_a() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, 13140);
        }
    }
}
