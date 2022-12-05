use aoc_utils::{empty, read_lines, run, Result};

fn main() -> Result<()> {
    let xs = read_lines::<String>("day5/input")?;

    run(&xs, part_a, empty)
}

fn part_a(input: &[String]) -> Result<String> {
    let boxes = extract_boxes(input);
    println!("{:?}", boxes);
    let moves = extract_moves(input);
    println!("{:?}", moves);
    Ok("".to_string())
}

fn extract_moves(input: &[String]) -> Vec<String> {
    let mut moves = vec![];
    input
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .for_each(|line| moves.push(line.to_owned()));
    moves
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
