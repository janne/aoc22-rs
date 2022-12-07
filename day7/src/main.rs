use std::{cell::RefCell, rc::Rc};

use aoc_utils::{empty, read_lines, run, Result};

struct Dir {
    name: String,
    size: Option<u32>,
    children: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new(name: &str, size: Option<u32>) -> Self {
        Dir {
            name: name.to_string(),
            size,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Dir>>) {
        self.children.push(new_node);
    }
}

fn main() -> Result<()> {
    let lines = read_lines::<String>("day7/input")?;

    run(&lines, part_a, empty)
}

fn part_a(lines: &[String]) -> Result<i32> {
    let root = Rc::new(RefCell::new(Dir::new(".", None)));
    let mut current = Rc::clone(&root);

    for line in lines {
        let input: Vec<&str> = (*line).split(' ').collect();
        println!("{:?}", input);
        match input[0] {
            "$" => match input[1] {
                "cd" => match input[2] {
                    "/" => {
                        current = Rc::clone(&root);
                    }
                    _ => panic!("Unknown input: {:?}", input),
                },
                "ls" => (),
                _ => panic!("Unknown input: {:?}", input),
            },
            "dir" => {
                let dir = Dir::new(input[1], None);
                current.borrow_mut().add_child(Rc::new(RefCell::new(dir)));
            }
            i => {
                let size: u32 = i.parse().unwrap();
                let dir = Dir::new(input[1], Some(size));
                current.borrow_mut().add_child(Rc::new(RefCell::new(dir)));
            }
            _ => panic!("Unknown input: {:?}", input),
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn returns_zero() {
        if let Ok(r) = part_a(&input()) {
            assert_eq!(r, 95437);
        }
    }
}
