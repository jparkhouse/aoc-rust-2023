pub mod map_to;

use map_to::MapTo;
use regex::Regex;
use thiserror::Error;

use crate::shared::DayResult;
use std::{error, fs, num::ParseIntError};

pub fn solve() -> Result<DayResult, String> {
    let input = match fs::read_to_string("src/day_5/input.txt") {
        Ok(result) => result,
        Err(err) => return Err(format!("Error in reading file: {}", err)),
    };
    let output = DayResult {
        part_1: solve_part_1(&input)?,
        part_2: solve_part_2(&input)?,
    };

    return Ok(output);
}

fn solve_part_1(input: &str) -> Result<usize, String> {
    let input = parse_input(input).map_err(|e| e.to_string())?;
    let seed_locations: Vec<usize> = input
        .seeds
        .into_iter()
        .map(|seed| seed_to_location(seed, input.maps.as_ref()))
        .collect();
    match seed_locations.into_iter().min() {
        Some(result) => Ok(result),
        None => Err("No seeds found".to_string()),
    }
}

struct SeedsAndMaps {
    seeds: Vec<usize>,
    maps: Vec<MapTo>,
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("This should never happen")]
    NoInputProvided,

    #[error("There was no double new lines found, invalid format")]
    NoSplitsThereforeInvalidFormat,

    #[error("Error parsing seeds: {0}")]
    SeedParseError(#[from] SeedParseError),

    #[error("Error parsing maps: {0}")]
    MapParseError(#[from] MapParseError),
}

fn parse_input(input: &str) -> Result<SeedsAndMaps, ParseInputError> {
    if input.trim().is_empty() {
        return Err(ParseInputError::NoInputProvided);
    }

    let rows: Vec<&str> = input.split("\n\n").collect();

    if rows.len() < 2 {
        return Err(ParseInputError::NoSplitsThereforeInvalidFormat);
    }

    let mut rows_iter = rows.into_iter();
    let output_seeds = parse_seeds(rows_iter.next().unwrap())?;
    let output_maps = parse_maps(rows_iter.collect::<Vec<&str>>())?;
    return Ok(SeedsAndMaps {
        seeds: output_seeds,
        maps: output_maps,
    });
}

#[derive(Debug, Error)]
enum SeedParseError {
    #[error("Regex error: {0}")]
    RegexError(String),

    #[error("Failed to parse seed number: {0}")]
    InvalidNumber(#[from] ParseIntError),

    #[error("The 'seeds' keyword is missing or malformed")]
    MissingKeyword,
}

fn parse_seeds(seeds: &str) -> Result<Vec<usize>, SeedParseError> {
    let pattern = match Regex::new(r"seeds:\s*((\d+\s*)+)") {
        Ok(result) => result,
        Err(e) => return Err(SeedParseError::RegexError(e.to_string())),
    };
    let captures = pattern
        .captures(seeds)
        .ok_or(SeedParseError::MissingKeyword)?;
    let seeds_str = captures
        .get(1)
        .ok_or(SeedParseError::MissingKeyword)?
        .as_str();
    let output: Result<Vec<usize>, ParseIntError> = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect();
    match output {
        Ok(result) => Ok(result),
        Err(e) => Err(SeedParseError::InvalidNumber(e)),
    }
}

#[derive(Debug, Error)]
enum MapParseError {
    #[error("Regex error: {0}")]
    RegexError(String),

    #[error("Failed to parse mapping number: {0}")]
    InvalidNumber(#[from] ParseIntError),

    #[error("Invalid row format")]
    InvalidRowFormat,

    #[error("Missing map block or invalid format")]
    MissingBlock,

    #[error("Missing map input string or invalid format")]
    MissingInputString,

    #[error("Missing map output string or invalid format")]
    MissingOutputString,

    #[error("Missing mappping row(s) or invalid format")]
    MissingMappingRows,
}

fn parse_maps(map_blocks: Vec<&str>) -> Result<Vec<MapTo>, MapParseError> {
    let block_pattern = Regex::new(r"(\w+)-to-(\w+) map:\n((?:\d+\s+\d+\s+\d+\n?)*)")
        .map_err(|e| MapParseError::RegexError(e.to_string()))?;

    let mut maps = Vec::new();

    for block in map_blocks {
        let captures = block_pattern
            .captures(block)
            .ok_or(MapParseError::MissingBlock)?;

        let input = captures.get(1).ok_or(MapParseError::MissingInputString)?.as_str();
        let output = captures.get(2).ok_or(MapParseError::MissingOutputString)?.as_str();
        let rows_str = captures.get(3).ok_or(MapParseError::MissingMappingRows)?.as_str();

        let rows: Result<Vec<(usize, usize, usize)>, MapParseError> = rows_str
            .lines()
            .map(|line| {
                let nums: Vec<&str> = line.split_whitespace().collect();
                if nums.len() != 3 {
                    return Err(MapParseError::InvalidRowFormat);
                }
                let start_input = nums[0].parse::<usize>().map_err(|e| MapParseError::InvalidNumber(e))?;
                let start_output = nums[1].parse::<usize>().map_err(|e| MapParseError::InvalidNumber(e))?;
                let steps = nums[2].parse::<usize>().map_err(|e| MapParseError::InvalidNumber(e))?;
                Ok((start_input, start_output, steps))
            })
            .collect();

        let rows = rows?;

        let mut map_to = MapTo::new(input, output);
        for (start_input, start_output, steps) in rows {
            map_to.add_range_map(start_input, start_output, steps);
        }
        maps.push(map_to);
    }

    Ok(maps)
}

fn seed_to_location(seed: usize, maps: &Vec<MapTo>) -> usize {
    for map in maps {
        let seed = map.get(seed);
    }
    return seed;
}
fn solve_part_2(input: &str) -> Result<usize, String> {
    return Ok(0);
}

#[cfg(test)]
mod tests {
    use super::solve_part_1;
    mod test_examples {
        #[test]
        fn test_example_1() {
            use super::solve_part_1;
            use std::fs;

            let input = match fs::read_to_string("src/day_5/test_input_part_1.txt") {
                Ok(result) => result,
                Err(err) => panic!("Error in file reading: {err}"),
            };

            let result = match solve_part_1(&input) {
                Ok(result) => result,
                Err(err) => panic!("Error in solve_part_1: {err}"),
            };

            assert_eq!(result, 35);
        }
    }
    mod test_parse_input {
        use crate::day_5::parse_input;
        use std::fs;
        #[test]
        fn parse_example_1() {
            let input = match fs::read_to_string("src/day_5/test_input_part_1.txt") {
                Ok(result) => result,
                Err(err) => panic!("Error in file reading: {err}"),
            };

            let result = match parse_input(&input) {
                Ok(result) => result,
                Err(e) => panic!("Error in parse input: {e}"),
            };

            assert_eq!(result.seeds, vec![79, 14, 55, 13]);
            assert_eq!(result.maps.len(), 7);
            assert_eq!(result.maps[0].input, "seed");
            assert_eq!(result.maps[0].get(50), 98);
            assert_eq!(result.maps[0].get(51), 99);
            assert_eq!(result.maps[0].get(52), 50);
        }
    }
}
