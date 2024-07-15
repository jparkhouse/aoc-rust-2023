use card_parser::{parse_card, CardInfo};
use std::fs;

use crate::shared::DayResult;

pub mod card_parser;

pub fn solve() -> Result<DayResult, String> {
    let input = match fs::read_to_string("src/day_4/input.txt") {
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
    let mut output: usize = 0;
    for line in input.lines() {
        let card = parse_card(line).map_err(|err| err.to_string())?;
        output += calculate_part_1_card_score(&card);
    }
    return Ok(output);
}

fn calculate_part_1_card_score(card: &CardInfo) -> usize {
    let mut score: usize = 0;
    for winning_number in card.winning_numbers.iter() {
        let wins = card
            .card_numbers
            .iter()
            .map(|number| winning_number == number)
            .fold(0 as usize, |count, win| match win {
                true => count + 1,
                false => count,
            });
        for _ in 0..wins {
            score = increment_score(score);
        }
    }

    return score;
}

fn increment_score(score: usize) -> usize {
    match score {
        0 => return 1,
        _ => return score * 2,
    }
}

struct CardCounts {
    card_counts: Vec<usize>
}

impl CardCounts {
    fn new() -> CardCounts{
        return CardCounts { card_counts: Vec::new() }
    }

    fn get_count(&self, index: usize) -> &usize {
        match self.card_counts.get(index) {
            Some(result) => return result,
            None => return &1,
        }
    }

    fn add_count(&mut self, index: usize, count: usize) {
        while self.card_counts.len() <= index {
            self.card_counts.push(1 as usize) // we always have the original card
        };
        self.card_counts[index] += count;
    }

    fn get_sum_of_all_counts(&self) -> usize {
        let result: usize = self.card_counts.iter().sum();
        return result
    }
}

fn solve_part_2(input: &str) -> Result<usize, String> {
    let mut counts = CardCounts::new();
    
    for line in input.lines() {
        let card = parse_card(line).map_err(|err| err.to_string())?;
        let wins = get_card_wins(&card);
        for i in 0..wins {
            counts.add_count(card.card_id + i, *counts.get_count(card.card_id - 1))
        }
    }

    return Ok(counts.get_sum_of_all_counts())
}

fn get_card_wins(card: &CardInfo) -> usize {
    let mut total_wins:usize = 0;
    for winning_number in card.winning_numbers.iter() {
        let wins = card
            .card_numbers
            .iter()
            .map(|number| winning_number == number)
            .fold(0 as usize, |count, win| match win {
                true => count + 1,
                false => count,
            });
        total_wins += wins;
        }
    return total_wins
}


mod tests {
    use super::{solve_part_1, parse_card, calculate_part_1_card_score};

    mod test_examples {
        #[test]
        fn test_example_1() {
            use super::solve_part_1;
            use std::fs;

            let input = match fs::read_to_string("src/day_4/test_input_part_1.txt") {
                Ok(result) => result,
                Err(err) => panic!("Error in file reading: {err}"),
            };

            let result = match solve_part_1(&input) {
                Ok(result) => result,
                Err(err) => panic!("Error in solve_part_1: {err}"),
            };

            assert_eq!(result, 13);
        }
    }

    mod test_calculate_part_1_card_score {

        #[test]
        fn test_example_1_card_1() {
            use super::parse_card;
            use super::calculate_part_1_card_score;

            let card = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").expect("error parsing card");
            let result = calculate_part_1_card_score(&card);

            assert_eq!(result, 8)
        }
    }
}