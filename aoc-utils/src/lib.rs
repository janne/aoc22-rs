use core::{fmt, result};
use std::{error, fs, str::FromStr};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

type PartFn<T, R> = fn(&[T]) -> Result<R>;

pub fn run<T, R1, R2>(input: &[T], part_a: PartFn<T, R1>, part_b: PartFn<T, R2>) -> Result<()>
where
    R1: fmt::Display,
    R2: fmt::Display,
{
    let a = part_a(input)?;
    let b = part_b(input)?;

    println!("Part A: {}", a);
    println!("Part B: {}", b);

    Ok(())
}

pub fn empty<T>(_xs: &[T]) -> Result<char> {
    Ok('-')
}

pub fn read_chars(path: &str) -> result::Result<Vec<char>, Box<dyn error::Error>> {
    let contents: Vec<char> = fs::read_to_string(path)?.chars().collect();
    Ok(contents)
}

pub fn read_lines<T>(path: &str) -> result::Result<Vec<T>, Box<dyn error::Error>>
where
    T: FromStr,
    T::Err: error::Error + 'static,
{
    let contents = fs::read_to_string(path)?;
    let lines: Vec<&str> = contents.lines().collect();
    let lines: result::Result<Vec<T>, _> = lines.iter().map(|line| line.parse()).collect();
    Ok(lines?)
}
