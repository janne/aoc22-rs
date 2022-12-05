use aoc_utils::{empty, read_lines, run, Result};
use regex::Regex;

fn main() -> Result<()> {
    let xs = read_lines::<String>("day5/input")?;

    run(&xs, part_a, empty)
}

fn part_a(input: &[String]) -> Result<String> {
    let mut boxes = extract_boxes(input);
    for (count, from, to) in extract_moves(input) {
        for _ in 0..count {
            let c = boxes[from - 1].pop().unwrap();
            boxes[to - 1].push(c);
        }
    }
    Ok(boxes.iter().map(|b| b.last().unwrap()).collect::<String>())
}

fn extract_moves(input: &[String]) -> Vec<(usize, usize, usize)> {
    let mut moves = vec![];
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    input
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .for_each(|line| moves.push(line.to_owned()));
    moves
        .iter()
        .map(|m| {
            let caps = re.captures_iter(m).next().unwrap();
            (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
                caps[3].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn extract_boxes(input: &[String]) -> Vec<Vec<char>> {
    let count = (input[0].len() + 1) / 4;
    let mut boxes: Vec<Vec<char>> = vec![vec![]; count];
    input
        .iter()
        .take_while(|line| !line.starts_with(" 1"))
        .for_each(|line| {
            (0..count).for_each(|i| {
                let c = line.chars().nth(i * 4 + 1).unwrap();
                if c != ' ' {
                    boxes[i].insert(0, c);
                }
            });
        });
    boxes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn returns_zero() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, "CMZ".to_string());
        }
    }
}
