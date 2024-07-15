use regex::Regex;
use thiserror::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct CardInfo {
    pub card_id: usize,
    pub winning_numbers: Vec<usize>,
    pub card_numbers: Vec<usize>,
}

#[derive(Debug, Error)]
pub enum CardParseError {
    #[error("Failed to compile regex: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Failed to capture card ID")]
    CardIdCaptureError,

    #[error("Failed to parse card ID: {0}")]
    CardIdParseError(#[from] ParseIntError),

    #[error("Failed to capture winning numbers")]
    WinningNumbersCaptureError,

    #[error("Failed to parse winning numbers: {0}")]
    WinningNumbersParseError(ParseIntError),

    #[error("Failed to capture card numbers")]
    CardNumbersCaptureError,

    #[error("Failed to parse card numbers: {0}")]
    CardNumbersParseError(ParseIntError),
}

pub fn parse_card(input: &str) -> Result<CardInfo, CardParseError> {
    let pattern = r"Card\s+(\d+): ([\d\s]+) \| ([\d\s]+)";
    let re = Regex::new(pattern)?;

    let captures = re.captures(input).ok_or(CardParseError::CardIdCaptureError)?;

    let card_id = captures.get(1)
        .ok_or(CardParseError::CardIdCaptureError)?
        .as_str()
        .parse::<usize>()?;

    let winning_numbers_str = captures.get(2).ok_or(CardParseError::WinningNumbersCaptureError)?.as_str();
    let winning_numbers = winning_numbers_str
        .split_whitespace()
        .map(|s| s.parse::<usize>().map_err(CardParseError::WinningNumbersParseError))
        .collect::<Result<Vec<usize>, CardParseError>>()?;

    let card_numbers_str = captures.get(3).ok_or(CardParseError::CardNumbersCaptureError)?.as_str();
    let card_numbers = card_numbers_str
        .split_whitespace()
        .map(|s| s.parse::<usize>().map_err(CardParseError::CardNumbersParseError))
        .collect::<Result<Vec<usize>, CardParseError>>()?;

    Ok(CardInfo { card_id, winning_numbers, card_numbers })
}

mod tests {

    #[test]
    fn test_parse_card() {
        use super::parse_card;

        let input = "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = parse_card(input);
        assert!(result.is_ok());
        let card_info = result.unwrap();
        assert_eq!(card_info.card_id, 1);
        assert_eq!(card_info.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(card_info.card_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }
}