use aoc_utils::{empty, read_lines, run, Result};

type Coord = (usize, usize);

struct Position {
    coord: Coord,
    height: char,
    explored: bool,
    from: Option<Coord>,
}

impl Position {
    fn new(coord: Coord, height: char) -> Self {
        Position {
            coord,
            height,
            explored: false,
            from: None,
        }
    }
}

fn main() -> Result<()> {
    let lines = read_lines::<String>("day12/input")?;

    run(&lines, part_a, empty)
}

fn find_start_goal(lines: &[String]) -> (Coord, Coord) {
    let mut start: Coord = (0, 0);
    let mut end: Coord = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start = (x, y),
                'E' => end = (x, y),
                _ => (),
            }
        }
    }
    (start, end)
}

fn parse(lines: &[String]) -> Vec<Vec<Position>> {
    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    Position::new(
                        (x, y),
                        match c {
                            'S' => 'a',
                            'E' => 'z',
                            x => x,
                        },
                    )
                })
                .collect()
        })
        .collect()
}

fn part_a(lines: &[String]) -> Result<i32> {
    let mut map: Vec<Vec<Position>> = parse(&lines);
    let (start, goal) = find_start_goal(&lines);
    let mut queue: Vec<Position> = Vec::new();
    let mut curr = start;

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines() -> Vec<String> {
        ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"]
            .map(String::from)
            .to_vec()
    }

    #[test]
    fn solves_part_a() {
        if let Ok(r) = part_a(&lines()) {
            assert_eq!(r, 0);
        }
    }
}
