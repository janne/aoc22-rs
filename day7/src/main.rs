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
}

fn main() -> Result<()> {
    let lines = read_lines::<String>("day7/input")?;

    run(&lines, part_a, empty)
}

fn part_a(lines: &[String]) -> Result<u32> {
    let root = Rc::new(RefCell::new(Dir::new(".", None)));
    let mut current = Rc::clone(&root);
    for line in lines {
        let input: Vec<&str> = (*line).split(' ').collect();
        match input[0] {
            "$" => match input[1] {
                "cd" => match input[2] {
                    "/" => {
                        current = Rc::clone(&root);
                    }
                    ".." => {
                        let current_clone = Rc::clone(&current);
                        current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    }
                    a => {
                        let current_clone = Rc::clone(&current);
                        let current_ref = current_clone.borrow();
                        let child = current_ref
                            .children
                            .iter()
                            .find(|&d| d.borrow().name == a)
                            .unwrap();
                        current = Rc::clone(&child);
                    }
                },
                "ls" => (),
                _ => panic!("Unknown input: {:?}", input),
            },
            "dir" => {
                let dir = Rc::new(RefCell::new(Dir::new(input[1], None)));
                current.borrow_mut().children.push(Rc::clone(&dir));
                let mut mut_dir = dir.borrow_mut();
                mut_dir.parent = Some(Rc::clone(&current));
            }
            i => {
                let size: u32 = i.parse().unwrap();
                let dir = Rc::new(RefCell::new(Dir::new(input[1], Some(size))));
                current.borrow_mut().children.push(Rc::clone(&dir));
                let mut mut_dir = dir.borrow_mut();
                mut_dir.parent = Some(Rc::clone(&current));
            }
        }
    }
    let mut sum: u32 = 0;

    let mut summer = |v| sum += v;

    traverse(&root, &mut summer);

    Ok(sum)
}

fn traverse<F>(root: &Rc<RefCell<Dir>>, f: &mut F) -> u32
where
    F: FnMut(u32),
{
    let dir = root.borrow();
    let mut sum = 0;
    for child in &dir.children {
        sum += traverse(child, f);
    }

    if dir.size.is_none() && sum <= 100_000 {
        f(sum);
    }
    dir.size.unwrap_or(0) + sum
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
