use std::fs;
use crate::shared;

pub fn solve() -> Result<(usize, usize), String> {
    let input = match fs::read_to_string("src/day_X/input.txt") {
        Ok(result) => result,
        Err(err) => return Err(format!("Error in reading file: {}", err)),
    };

    let part_1 = solve_part_1(&input)?;
    let part_2 = solve_part_2(&input)?;

    return Ok((part_1, part_2));
}

fn solve_part_1(input: &str) -> Result<usize, String> {
    return Ok(0);
}

fn solve_part_2(input: &str) -> Result<usize, String> {
    return Ok(0);
}