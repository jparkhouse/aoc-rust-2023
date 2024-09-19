use crate::{
    day_5::{
        linear_range_binary_search_tree::LinearRangeBinarySearchTree,
        map_binary_search_tree::{MapBinarySearchTree, MapBinarySearchTreeError},
        seed_set::SeedSet,
    },
    shared::DayResult,
};
use linear_range::{LinearRange, LinearRangeError};
use linear_range_binary_search_tree::LinearRangeBinarySearchTreeError;
use map_tuple::{MapTuple, MapTupleError};
use regex::Regex;
use std::{fs, num::ParseIntError};
use thiserror::Error;

mod linear_range;
mod linear_range_binary_search_tree;
mod map_binary_search_tree;
mod map_tuple;

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
    let parsed_input = parse_input_part_1(input).map_err(|e| e.to_string())?;
    println!(
        "Calculating locations for {0} seeds",
        parsed_input.seeds.len()
    );
    let minimum_location = parsed_input
        .seeds
        .into_iter()
        .map(|seed| {
            parsed_input
                .maps
                .iter()
                .fold(seed, |current, tree| tree.get_mapped_value(current))
        })
        .min();
    match minimum_location {
        Some(value) => {
            println!("Part 1 done!");
            Ok(value)
        }
        None => Err("Empty seeds".to_string()),
    }
}

fn solve_part_2(input: &str) -> Result<usize, String> {
    let parsed_input = match parse_input_part_2(input) {
        Ok(mut result) => {
            result
                .seed_sets
                .sort_by(|a, b| a.compare_without_overlap(b).unwrap());
            result
        }
        Err(e) => return Err(e.to_string()),
    };

    let map_iter = parsed_input.maps.into_iter();
    let mut current_input_ranges: Vec<LinearRange> = parsed_input
        .seed_sets
        .iter()
        .map(|seed_set| LinearRange::from_seed_set(seed_set))
        .collect::<Result<Vec<LinearRange>, LinearRangeError>>()
        .map_err(|e| e.to_string())?;

    for map in map_iter {
        let next_input_ranges =
            merge_linear_ranges_and_map_tuples(&current_input_ranges, &map.get_sorted_vec())
                .map_err(|e| e.to_string())?;
        current_input_ranges = next_input_ranges;
    }

    let output = current_input_ranges
        .first()
        .ok_or(String::from("No LinearMaps survived"))?
        .start;

    Ok(output)
}

/// Takes a sorted `LinearRange` vec and a sorted `MapTuple` vec and iterates over both,
/// pairing where appropriate to create a new vec of (sorted) `LinearRange`s.
/// Designed to ensure all values in the original `LinearRange`s are either carried over or mapped into the output
fn merge_linear_ranges_and_map_tuples(
    input_ranges: &[LinearRange],
    map_tuples: &[MapTuple],
) -> Result<Vec<LinearRange>, MergeError> {
    // initialise out-of-loop values
    let mut input_range_iter = input_ranges.iter().peekable();
    let mut map_tuple_iter = map_tuples.iter().peekable();
    let mut current_input_value = input_ranges.first().map(|range| range.start).unwrap_or(0);
    let mut output_tree = LinearRangeBinarySearchTree::new();

    loop {
        // copy the values
        let next_input_range = input_range_iter.peek().copied();
        let next_map_tuple = map_tuple_iter.peek().copied();
        // then work with the copied values to avoid locking the iterators
        match (next_input_range, next_map_tuple) {
            (Some(input_range), Some(map_tuple)) => {
                if map_tuple.source_range_start + map_tuple.range_length >= input_range.start {
                    match get_critical_point(current_input_value, input_range, map_tuple) {
                        CriticalPointEnum::StartOfInputRange(critical_point) => {
                            current_input_value = critical_point; // move to start of next input range
                        }
                        CriticalPointEnum::StartOfMapTuple(critical_point) => {
                            // from start of input range to start of map tuple, maps to itself
                            let steps = critical_point
                                .checked_sub(current_input_value)
                                .ok_or(MergeError::Underflow)?;
                            let range = LinearRange::new(current_input_value, steps)
                                .map_err(|e| MergeError::FailedToCreateLinearRange(e))?;
                            output_tree.unbalanced_insert(range)?;
                            current_input_value = critical_point;
                        }
                        CriticalPointEnum::EndOfMapTuple(critical_point) => {
                            // from the current_input point to the end of the MapTuple
                            let start = map_tuple.calculate_output(current_input_value)?;
                            let steps = critical_point
                                .checked_sub(current_input_value)
                                .ok_or(MergeError::Underflow)?;

                            let range = LinearRange::new(start, steps)
                                .map_err(|e| MergeError::FailedToCreateLinearRange(e))?;
                            output_tree.unbalanced_insert(range)?;
                            current_input_value = critical_point;
                            map_tuple_iter.next();
                        }
                        CriticalPointEnum::EndOfInputRange(critical_point) => {
                            // from the current_input point to the end of the input range
                            // but we have to check if we are in a MapTuple or not
                            if map_tuple.contains(current_input_value) {
                                // we are in a MapTuple
                                let start = map_tuple.calculate_output(current_input_value)?;
                                let steps = critical_point
                                    .checked_sub(current_input_value)
                                    .ok_or(MergeError::Underflow)?;

                                let range = LinearRange::new(start, steps)
                                    .map_err(|e| MergeError::FailedToCreateLinearRange(e))?;
                                output_tree.unbalanced_insert(range)?;
                            } else {
                                // we are not in a MapTuple
                                let steps = critical_point
                                    .checked_sub(current_input_value)
                                    .ok_or(MergeError::Underflow)?;
                                let range = LinearRange::new(current_input_value, steps)
                                    .map_err(|e| MergeError::FailedToCreateLinearRange(e))?;
                                output_tree.unbalanced_insert(range)?;
                            }
                            current_input_value = critical_point;
                            input_range_iter.next();
                        }
                        CriticalPointEnum::StartOfNextInputRange => {
                            // we have to consume the current value to get to the next one
                            let _ = input_range_iter.next();
                        }
                    }
                } else {
                    let _ = map_tuple_iter.next();
                }
            }
            (Some(&input), None) => {
                if current_input_value > input.start {
                    // we are part way through the current LinearRange
                    let steps = input
                        .steps
                        .checked_sub(
                            current_input_value
                                .checked_sub(input.start)
                                .ok_or(MergeError::Underflow)?,
                        )
                        .ok_or(MergeError::Underflow)?;
                    let rest_of_input = LinearRange::new(current_input_value, steps)?;
                    output_tree
                        .unbalanced_insert(rest_of_input)
                        .map_err(|e| MergeError::BinaryTreeOverflow(e))?;
                } else {
                    output_tree
                        .unbalanced_insert(input)
                        .map_err(|e| MergeError::BinaryTreeOverflow(e))?;
                }

                input_range_iter.next();
            }
            (None, _) => break,
        }
    }

    return Ok(output_tree.get_sorted_vec());
}

#[derive(Debug, Error)]
enum MergeError {
    #[error("Overflow in LRBST key")]
    BinaryTreeOverflow(#[from] LinearRangeBinarySearchTreeError),

    #[error("Error in linear range creation: {0}")]
    FailedToCreateLinearRange(#[from] LinearRangeError),

    #[error("Error getting output value from MapTuple: {0}")]
    MapTupleOutput(#[from] MapTupleError),

    #[error("`critical_point` is before `current_input`, resulting in a negative usize")]
    Underflow,
}

enum CriticalPointEnum {
    StartOfInputRange(usize),
    StartOfMapTuple(usize),
    EndOfInputRange(usize),
    EndOfMapTuple(usize),
    StartOfNextInputRange,
}

/// Evaluates where the next critical point is based on the provided current value, current LinearRange, and current MapTuple
fn get_critical_point(
    current_value: usize,
    range: &LinearRange,
    map_tuple: &MapTuple,
) -> CriticalPointEnum {
    let input_range_end = range.start + range.steps - 1;
    let map_tuple_end = map_tuple.source_range_start + map_tuple.range_length - 1;

    if current_value < range.start {
        // Not yet in the seed set
        CriticalPointEnum::StartOfInputRange(range.start)
    } else if current_value >= input_range_end {
        // Beyond the seed set
        CriticalPointEnum::StartOfNextInputRange
    } else if current_value < map_tuple.source_range_start {
        // Inside the seed set, but before the map tuple
        CriticalPointEnum::StartOfMapTuple(map_tuple.source_range_start)
    } else if current_value < map_tuple_end {
        // Inside the map tuple
        if map_tuple_end > input_range_end {
            // The map tuple extends beyond the current input range
            CriticalPointEnum::EndOfInputRange(input_range_end)
        } else {
            // The map tuple is contained within the input range
            CriticalPointEnum::EndOfMapTuple(map_tuple_end)
        }
    } else {
        // Inside the seed set, but after the map tuple
        CriticalPointEnum::EndOfInputRange(input_range_end)
    }
}

struct SeedsAndMaps {
    seeds: Vec<usize>,
    maps: Vec<MapBinarySearchTree>,
}

enum SearchDirection {
    Contains,
    Greater,
    Less,
}

mod seed_set {
    use std::cmp::Ordering;
    use thiserror::Error;

    use crate::day_5::SearchDirection;

    pub struct SeedSet {
        pub start: usize,
        pub steps: usize,
    }

    #[derive(Debug, Error)]
    pub enum SeedSetError {
        #[error("Compared two seed sets that overlap")]
        Overlap,
    }

    impl SeedSet {
        pub fn new(start: usize, steps: usize) -> Self {
            Self {
                start: start,
                steps: steps,
            }
        }

        pub fn contains(&self, value: usize) -> bool {
            return self.start <= value && value < self.start + self.steps;
        }

        pub fn does_not_overlap(&self, other: &Self) -> bool {
            return self.start + self.steps <= other.start
                || other.start + other.steps <= self.start;
        }

        pub fn get_search_direction(&self, value: usize) -> SearchDirection {
            match (self.contains(value), value < self.start) {
                (true, _) => SearchDirection::Contains,
                (false, true) => SearchDirection::Less,
                (false, false) => SearchDirection::Greater,
            }
        }

        pub fn compare_without_overlap(&self, other: &Self) -> Result<Ordering, SeedSetError> {
            if self.does_not_overlap(other) {
                match self.start < other.start {
                    true => return Ok(Ordering::Less),
                    false => return Ok(Ordering::Greater),
                }
            } else {
                return Err(SeedSetError::Overlap);
            }
        }
    }
}

struct SeedSetsAndMaps {
    seed_sets: Vec<SeedSet>,
    maps: Vec<MapBinarySearchTree>,
}

fn parse_input_part_1(input: &str) -> Result<SeedsAndMaps, ParseInputError> {
    if input.trim().is_empty() {
        return Err(ParseInputError::NoInputProvided);
    }

    let rows: Vec<&str> = input.split("\n\n").collect();

    if rows.len() < 2 {
        return Err(ParseInputError::NoSplitsThereforeInvalidFormat);
    }

    let mut rows_iter = rows.into_iter();
    let output_seeds = parse_seeds_part_1(rows_iter.next().unwrap())?;
    let output_maps = parse_maps(rows_iter.collect::<Vec<&str>>())?;

    println!("Successfully parsed input file!");

    return Ok(SeedsAndMaps {
        seeds: output_seeds,
        maps: output_maps,
    });
}

fn parse_input_part_2(input: &str) -> Result<SeedSetsAndMaps, ParseInputError> {
    if input.trim().is_empty() {
        return Err(ParseInputError::NoInputProvided);
    }

    let rows: Vec<&str> = input.split("\n\n").collect();

    if rows.len() < 2 {
        return Err(ParseInputError::NoSplitsThereforeInvalidFormat);
    }

    let mut rows_iter = rows.into_iter();
    let output_seed_sets = parse_seeds_part_2(rows_iter.next().unwrap())?;
    let output_maps = parse_maps(rows_iter.collect::<Vec<&str>>())?;

    println!("Successfully parsed input file!");

    return Ok(SeedSetsAndMaps {
        seed_sets: output_seed_sets,
        maps: output_maps,
    });
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

#[derive(Debug, Error)]
enum SeedParseError {
    #[error("Regex error: {0}")]
    RegexError(String),

    #[error("Failed to parse seed number: {0}")]
    InvalidNumber(#[from] ParseIntError),

    #[error("The 'seeds' keyword is missing or malformed")]
    MissingKeyword,
}

fn parse_seeds_part_1(seeds: &str) -> Result<Vec<usize>, SeedParseError> {
    let pattern = match Regex::new(r"seeds:\s*((\d+\s*)+)") {
        Ok(result) => result,
        Err(e) => return Err(SeedParseError::RegexError(e.to_string())),
    };

    println!("Beginning to parse seeds...");

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
        Ok(result) => {
            println!("Successfully parsed seeds!");
            Ok(result)
        }
        Err(e) => Err(SeedParseError::InvalidNumber(e)),
    }
}

fn parse_seeds_part_2(seeds: &str) -> Result<Vec<SeedSet>, SeedParseError> {
    let pattern = match Regex::new(r"seeds:\s*((\d+\s*)+)") {
        Ok(result) => result,
        Err(e) => return Err(SeedParseError::RegexError(e.to_string())),
    };

    println!("Beginning to parse seeds...");

    let captures = pattern
        .captures(seeds)
        .ok_or(SeedParseError::MissingKeyword)?;
    let seeds_str = captures
        .get(1)
        .ok_or(SeedParseError::MissingKeyword)?
        .as_str();
    let all_seeds_result: Result<Vec<usize>, ParseIntError> = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect();
    let all_seeds = match all_seeds_result {
        Ok(result) => {
            println!("Successfully parsed seeds!");
            result
        }
        Err(e) => return Err(SeedParseError::InvalidNumber(e)),
    };
    let output = all_seeds
        .chunks_exact(2)
        .map(|chunk| SeedSet {
            start: chunk[0],
            steps: chunk[1],
        })
        .collect::<Vec<_>>();
    return Ok(output);
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

    #[error("MapBinarySearchTreeError: {0}")]
    MapBinarySearchTreeError(#[from] MapBinarySearchTreeError),
}

fn parse_maps(map_blocks: Vec<&str>) -> Result<Vec<MapBinarySearchTree>, MapParseError> {
    let block_pattern = Regex::new(r"(\w+)-to-(\w+) map:\n((?:\d+\s+\d+\s+\d+\n?)*)")
        .map_err(|e| MapParseError::RegexError(e.to_string()))?;

    println!("Beginning to parse maps...");
    let mut maps: Vec<MapBinarySearchTree> = Vec::new();

    for block in map_blocks {
        let captures = block_pattern
            .captures(block)
            .ok_or(MapParseError::MissingBlock)?;
        let _input = captures
            .get(1)
            .ok_or(MapParseError::MissingInputString)?
            .as_str();
        let _output = captures
            .get(2)
            .ok_or(MapParseError::MissingOutputString)?
            .as_str();
        let rows_str = captures
            .get(3)
            .ok_or(MapParseError::MissingMappingRows)?
            .as_str();

        let rows: Result<Vec<MapTuple>, MapParseError> = rows_str
            .lines()
            .map(|line| {
                let nums: Vec<&str> = line.split_whitespace().collect();
                if nums.len() != 3 {
                    return Err(MapParseError::InvalidRowFormat);
                }
                let destination_range_start = nums[0]
                    .parse::<usize>()
                    .map_err(|e| MapParseError::InvalidNumber(e))?;
                let source_range_start = nums[1]
                    .parse::<usize>()
                    .map_err(|e| MapParseError::InvalidNumber(e))?;
                let range_length = nums[2]
                    .parse::<usize>()
                    .map_err(|e| MapParseError::InvalidNumber(e))?;
                Ok(MapTuple::new(
                    destination_range_start,
                    source_range_start,
                    range_length,
                ))
            })
            .collect();

        let rows = rows?;

        maps.push(
            MapBinarySearchTree::from_vec(rows)
                .map_err(|e| MapParseError::MapBinarySearchTreeError(e))?,
        )
    }
    println!("Successfully parsed maps!");
    return Ok(maps);
}

#[cfg(test)]
mod tests {
    mod examples {
        use crate::day_5::solve_part_2;

        #[test]
        fn example_for_part_2() {
            use std::fs;

            let example_input = match fs::read_to_string("src/day_5/test_input_part_1.txt") {
                Ok(result) => result,
                Err(err) => panic!("Error in file reading: {err}"),
            };

            let answer = match solve_part_2(&example_input) {
                Ok(result) => result,
                Err(err) => panic!("Error in solving part 2 example: {err}"),
            };

            assert_eq!(answer, 46)
        }
    }

    mod merge_linear_ranges_and_map_tuples {
        use crate::day_5::{
            linear_range::LinearRange, map_tuple::MapTuple, merge_linear_ranges_and_map_tuples,
        };

        #[test]
        fn first_example_from_part_2() {
            let seeds = vec![
                match LinearRange::new(55, 13) {
                    Ok(result) => result,
                    Err(e) => panic!("This should never happen: {e}"),
                },
                match LinearRange::new(79, 25) {
                    Ok(result) => result,
                    Err(e) => panic!("This should never happen: {e}"),
                },
            ];

            let seed_to_soil_maps = vec![MapTuple::new(52, 50, 48), MapTuple::new(50, 98, 2)];

            let output = match merge_linear_ranges_and_map_tuples(&seeds, &seed_to_soil_maps) {
                Ok(result) => result,
                Err(e) => panic!("Error in merge: {e}"),
            };

            print!("{:?}", output)
        }
    }
}
