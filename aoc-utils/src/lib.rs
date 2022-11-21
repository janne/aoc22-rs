use std::{error, fs, str::FromStr};

pub type Done = Result<(), Box<dyn error::Error>>;

pub fn read_chars(path: &str) -> Result<Vec<char>, Box<dyn error::Error>> {
    let contents: Vec<char> = fs::read_to_string(path)?.chars().collect();
    Ok(contents)
}

pub fn read_lines<T>(path: &str) -> Result<Vec<T>, Box<dyn error::Error>>
where
    T: FromStr,
    T::Err: error::Error + 'static,
{
    let contents = fs::read_to_string(path)?;
    let lines: Vec<&str> = contents.lines().collect();
    let lines: Result<Vec<T>, _> = lines.iter().map(|line| line.parse()).collect();
    Ok(lines?)
}
