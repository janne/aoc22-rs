use std::{cell::RefCell, collections::VecDeque};

use aoc_utils::{empty, read_lines, run, Result};

type Coord = (i32, i32);

struct Position {
    coord: Coord,
    height: char,
    explored: bool,
    distance: i32,
}
struct Map {
    goal: Coord,
    start: Coord,
    positions: Vec<Vec<RefCell<Position>>>,
}

impl Map {
    fn get(&self, (x, y): Coord) -> Option<&RefCell<Position>> {
        self.positions
            .get(y as usize)
            .and_then(|line| line.get(x as usize))
    }

    fn edges(&self, (x, y): Coord) -> Vec<&RefCell<Position>> {
        let pos = &self.get((x, y)).unwrap().borrow();
        vec![
            self.get((x - 1, y)),
            self.get((x, y - 1)),
            self.get((x + 1, y)),
            self.get((x, y + 1)),
        ]
        .into_iter()
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .filter(|p| {
            let p: &Position = &p.borrow();
            p.height as u8 <= pos.height as u8 + 1
        })
        .collect()
    }

    // fn distance_from_start(&self, (x, y): Coord) -> i32 {
    // let mut distance = 0;
    // let mut pos: &Position = &self.get((x, y)).unwrap().borrow();

    // loop {
    //     let from = pos.from;
    //     if from.is_none() {
    //         return distance;
    //     }
    //     let new_pos = &self.get(from.unwrap()).unwrap().borrow();
    //     pos = new_pos;
    // }

    // while pos != None {
    //     let from = pos.from;
    //     if from.is_none() {
    //         pos = None;
    //     } else {
    //         distance += 1;
    //     }
    // }
    // }
}

impl Position {
    fn new(coord: Coord, height: char) -> Self {
        Position {
            coord,
            height,
            explored: false,
            distance: 0,
        }
    }
}

fn main() -> Result<()> {
    let lines = read_lines::<String>("day12/input")?;

    run(&lines, part_a, empty)
}

fn parse(lines: &[String]) -> Map {
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let positions = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    RefCell::new(Position::new(
                        (x as i32, y as i32),
                        match c {
                            'S' => {
                                start = (x as i32, y as i32);
                                'a'
                            }
                            'E' => {
                                goal = (x as i32, y as i32);
                                'z'
                            }
                            x => x,
                        },
                    ))
                })
                .collect()
        })
        .collect();
    Map {
        positions,
        start,
        goal,
    }
}

fn part_a(lines: &[String]) -> Result<i32> {
    let map = parse(&lines);
    let mut queue: VecDeque<Coord> = VecDeque::new();
    queue.push_back(map.start);
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if pos == map.goal {
            let dist = map.get(pos).unwrap().borrow().distance;
            return Ok(dist);
        }
        map.edges(pos).into_iter().for_each(|edge| {
            if !edge.borrow().explored {
                let mut edge_mut = edge.borrow_mut();
                edge_mut.explored = true;
                let distance = map.get(pos).unwrap().borrow().distance;
                edge_mut.distance = distance + 1;
                queue.push_back(edge_mut.coord);
            }
        });
    }

    Ok(1)
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
            assert_eq!(r, 31);
        }
    }
}
