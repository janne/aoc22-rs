use std::{fmt, fs, str::FromStr};

pub fn read_input<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: fmt::Debug,
{
    let contents = fs::read_to_string(path).expect("failed to read input");
    let lines: Vec<&str> = contents.lines().collect();
    return lines
        .iter()
        .map(|line| line.parse().expect("Invalid input"))
        .collect();
}
