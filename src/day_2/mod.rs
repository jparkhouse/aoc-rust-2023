use std::{collections::HashMap, fs};
extern crate regex;
use regex::Regex;

fn main() {}

pub fn solve() -> Result<(usize, usize), String> {
    let input = match fs::read_to_string("src/day_2/input.txt") {
        Ok(result) => result,
        Err(err) => return Err(format!("Error in reading file: {}", err)),
    };

    let part_1 = solve_part_1(&input)?;
    let part_2 = solve_part_2(&input)?;

    return Ok((part_1, part_2));
}

struct Game {
    id: usize,
    red: Vec<usize>,
    green: Vec<usize>,
    blue: Vec<usize>,
}

fn load_game(game_string: &str) -> Result<Game, String> {

    let re_game = match Regex::new(r"Game (\d+):") {
        Ok(result) => result,
        Err(err) => return Err(format!("Regex error: {}", err))
    };

    let re_round = match Regex::new(r"(\d+ \w+(?:, )?)+") {
        Ok(result) => result,
        Err(err) => return Err(format!("Regex error: {}", err))
    };

    let re_colour = match Regex::new(r"(\d+) (\w+)") {
        Ok(result) => result,
        Err(err) => return Err(format!("Regex error: {}", err))
    };

    if let Some(game_cap) = re_game.captures(game_string) {
        let game_id = match game_cap[1].parse::<usize>() {
            Ok(result) => result,
            Err(err) => return Err(format!("Parse error: {}", err)),
        };

        let mut rounds = vec![];
        for round_cap in re_round.captures_iter(game_string) {
            rounds.push(round_cap[0].to_string());
        }

        let num_rounds = rounds.len();
        let mut red = vec![0; num_rounds];
        let mut green = vec![0; num_rounds];
        let mut blue = vec![0; num_rounds];

        for (round_idx, round_str) in rounds.iter().enumerate() {
            let mut counts: HashMap<String, usize> = HashMap::new();

            for colour_cap in re_colour.captures_iter(&round_str) {
                let count = match colour_cap[1].parse::<usize>() {
                    Ok(result) => result,
                    Err(err) => return Err(format!("Parse error: {}", err))
                };
                let colour = colour_cap[2].to_string();
                counts.insert(colour, count);
            }

            if let Some(&count) = counts.get("red") {
                red[round_idx] = count;
            }
            if let Some(&count) = counts.get("green") {
                green[round_idx] = count;
            }
            if let Some(&count) = counts.get("blue") {
                blue[round_idx] = count;
            }
        }
        return Ok(Game { id: game_id, red, green, blue });
    } else {
        return Err(String::from("No game found"));
    }
}

fn solve_part_1(input: &str) -> Result<usize, String> {
    let games: Vec<Game> = input.lines().map(|line| {
        return load_game(line).expect(&format!("Error reading game: {}", line));
    })
    .collect();
    let games_r = games.into_iter()
    .filter(|game| game.red.iter().all(|cubes| cubes <= &12)).collect::<Vec<_>>();
    let games_rg = games_r.into_iter()
    .filter(|game| game.green.iter().all(|cubes| cubes <= &13)).collect::<Vec<_>>();
    let games_rgb = games_rg.into_iter()
    .filter(|game| game.blue.iter().all(|cubes| cubes <= &14)).collect::<Vec<_>>();
    let result = games_rgb.iter().fold(0, |count, game| count + game.id);
    return Ok(result);
}

fn solve_part_2(input: &str) -> Result<usize, String> {
    let games: Vec<Game> = input.lines().map(|line| {
        return load_game(line).expect(&format!("Error reading game: {}", line));
    })
    .collect();
    let powers = games.iter().map(|game| {
        let red_max = game.red.iter().max().unwrap();
        let green_max = game.green.iter().max().unwrap();
        let blue_max = game.blue.iter().max().unwrap();
        return red_max * green_max * blue_max;
    }).collect::<Vec<_>>();
    let result = powers.iter().sum();
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;
    mod examples {
        #[test]
        fn test_worked_example_part_1() {
            use super::solve_part_1;
            use std::fs;

            let input =
                fs::read_to_string("src/day_2/test_input_part_1.txt").expect("File read errored");
            let expected_output = 8;

            let actual_output = solve_part_1(&input).unwrap();

            assert_eq!(actual_output, expected_output);
        }
        #[test]
        fn test_worked_example_part_2() {
            use super::solve_part_2;
            use std::fs;

            let input =
                fs::read_to_string("src/day_2/test_input_part_2.txt").expect("File read errored");
            let expected_output = 2286;

            let actual_output = solve_part_2(&input).unwrap();

            assert_eq!(actual_output, expected_output);
        }
    }
}