use std::fs;

pub fn read_input(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("failed to read input");
    let lines: Vec<&str> = contents.lines().collect();
    return lines.into_iter().map(|line| line.into()).collect();
}
