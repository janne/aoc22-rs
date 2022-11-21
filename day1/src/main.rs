use aoc_utils::{read_input, Done};

fn main() -> Done {
    part_one()?;
    part_two()?;

    Ok(())
}

fn part_one() -> Done {
    let xs = read_input::<i32>("day1/input")?;

    println!("Part one: {:?}", xs);

    Ok(())
}

fn part_two() -> Done {
    let xs = read_input::<i32>("day1/input")?;

    println!("Part two: {:?}", xs);

    Ok(())
}
