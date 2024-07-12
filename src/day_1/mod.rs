use std::fs;

use crate::shared::{get_chars, DayResult};

pub fn solve() -> Result<DayResult, String> {
    let input = match fs::read_to_string("src/day_1/input.txt") {
        Ok(result) => result,
        Err(err) => return Err(format!("Error in reading file: {}", err)),
    };

    let part_1 = solve_part_1(&input)?;
    let part_2 = solve_part_2(&input)?;

    return Ok(DayResult { part_1: part_1, part_2: part_2});
}

fn solve_part_1(input: &str) -> Result<usize, String> {
    let result: usize;
    let first_ints: Vec<usize> = input
        .lines()
        .map(|row| {
            return find_first_int_part_1(row).unwrap();
        })
        .collect();
    let last_ints: Vec<usize> = input
        .lines()
        .map(|row| {
            return find_last_int_part_1(row).unwrap();
        })
        .collect();
    let combined_ints: Vec<usize> = first_ints
        .iter()
        .enumerate()
        .map(|item| return item.1 * 10 + last_ints[item.0])
        .collect();
    result = combined_ints.iter().sum();
    return Ok(result);
}

fn find_first_int_part_1(input: &str) -> Result<usize, String> {
    let chars: Vec<char> = get_chars(input);
    let numeric_chars = chars
        .iter()
        .filter(|item| is_int_part_1(item))
        .collect::<Vec<_>>();
    match numeric_chars.first() {
        Some(result) => return Ok(get_int_part_1(result).unwrap()),
        None => return Err(format!("No integers found in {}", input)),
    };
}

fn find_last_int_part_1(input: &str) -> Result<usize, String> {
    let chars: Vec<char> = get_chars(input);
    let numeric_chars = chars
        .iter()
        .filter(|item| is_int_part_1(item))
        .collect::<Vec<_>>();
    match numeric_chars.last() {
        Some(result) => return Ok(get_int_part_1(result).unwrap()),
        None => return Err(format!("No integers found in {}", input)),
    };
}

fn is_int_part_1(input: &char) -> bool {
    match input {
        '0' => return true,
        '1' => return true,
        '2' => return true,
        '3' => return true,
        '4' => return true,
        '5' => return true,
        '6' => return true,
        '7' => return true,
        '8' => return true,
        '9' => return true,
        _ => return false,
    }
}

fn get_int_part_1(input: &char) -> Option<usize> {
    match input {
        '0' => return Some(0),
        '1' => return Some(1),
        '2' => return Some(2),
        '3' => return Some(3),
        '4' => return Some(4),
        '5' => return Some(5),
        '6' => return Some(6),
        '7' => return Some(7),
        '8' => return Some(8),
        '9' => return Some(9),
        _ => return None,
    }
}

fn solve_part_2(input: &str) -> Result<usize, String> {
    let result: usize;
    let all_ints: Vec<Vec<usize>> = input
        .lines()
        .map(|row| {
            return find_all_ints_part_2(row).unwrap();
        })
        .collect();
    let first_ints: Vec<usize> = all_ints
        .iter()
        .map(|row| {
            return row.first().unwrap();
        })
        .cloned()
        .collect();
    let last_ints: Vec<usize> = all_ints
        .iter()
        .map(|row| {
            return row.last().unwrap();
        })
        .cloned()
        .collect();
    let combined_ints: Vec<usize> = first_ints
        .iter()
        .enumerate()
        .map(|item| return item.1 * 10 + last_ints[item.0])
        .collect();
    result = combined_ints.iter().sum();
    return Ok(result);
}

fn find_all_ints_part_2(input: &str) -> Result<Vec<usize>, String> {
    let chars_from_input = get_chars(input);
    let len = chars_from_input.len();
    let mut idx = 0;
    let mut chars_for_output: Vec<usize> = Vec::new();

    while idx < len {
        match chars_from_input[idx] {
            '0' => {
                chars_for_output.push(0);
                idx += 1;
            }
            '1' => {
                chars_for_output.push(1);
                idx += 1;
            }
            '2' => {
                chars_for_output.push(2);
                idx += 1;
            }
            '3' => {
                chars_for_output.push(3);
                idx += 1;
            }
            '4' => {
                chars_for_output.push(4);
                idx += 1;
            }
            '5' => {
                chars_for_output.push(5);
                idx += 1;
            }
            '6' => {
                chars_for_output.push(6);
                idx += 1;
            }
            '7' => {
                chars_for_output.push(7);
                idx += 1;
            }
            '8' => {
                chars_for_output.push(8);
                idx += 1;
            }
            '9' => {
                chars_for_output.push(9);
                idx += 1;
            }
            'z' => {
                let next = get_next_x(&chars_from_input, idx, 4);
                match next {
                    Ok(next) => {
                        if next == as_vec_of_char_refs("zero") {
                            chars_for_output.push(0);
                            idx += 1;
                        } else {
                            idx += 1;
                        }
                    }
                    Err(_err) => idx += 1,
                }
            }
            'o' => {
                let next = get_next_x(&chars_from_input, idx, 3);
                match next {
                    Ok(next) => {
                        if next == as_vec_of_char_refs("one") {
                            chars_for_output.push(1);
                            idx += 1;
                        } else {
                            idx += 1;
                        }
                    }
                    Err(_err) => idx += 1,
                }
            }
            't' => {
                // two and three
                let next_3 = get_next_x(&chars_from_input, idx, 3);
                match next_3 {
                    Ok(next_3) => {
                        if next_3 == as_vec_of_char_refs("two") {
                            chars_for_output.push(2);
                            idx += 1;
                        } else {
                            let next_5 = get_next_x(&chars_from_input, idx, 5);
                            match next_5 {
                                Ok(next_5) => {
                                    if next_5 == as_vec_of_char_refs("three") {
                                        chars_for_output.push(3);
                                        idx += 1;
                                    } else {
                                        idx += 1;
                                    }
                                }
                                Err(_err) => idx += 1,
                            }
                        }
                    }
                    Err(_err) => idx += 1,
                }
            }
            'f' => {
                // four and five
                let next = get_next_x(&chars_from_input, idx, 4);
                match next {
                    Ok(next) => {
                        if next == as_vec_of_char_refs("four") {
                            chars_for_output.push(4);
                            idx += 1;
                        } else if next == as_vec_of_char_refs("five") {
                            chars_for_output.push(5);
                            idx += 1;
                        } else {
                            idx += 1;
                        }
                    }
                    Err(_err) => idx += 1,
                }
            }
            's' => {
                // six and seven
                let next_3 = get_next_x(&chars_from_input, idx, 3);
                match next_3 {
                    Ok(next_3) => {
                        if next_3 == as_vec_of_char_refs("six") {
                            chars_for_output.push(6);
                            idx += 1;
                        } else {
                            let next_5 = get_next_x(&chars_from_input, idx, 5);
                            match next_5 {
                                Ok(next_5) => {
                                    if next_5 == as_vec_of_char_refs("seven") {
                                        chars_for_output.push(7);
                                        idx += 1;
                                    } else {
                                        idx += 1;
                                    }
                                }
                                Err(_err) => idx += 1,
                            }
                        }
                    }
                    Err(_err) => idx += 1,
                }
            }
            'e' => {
                let next = get_next_x(&chars_from_input, idx, 5);
                match next {
                    Ok(next) => {
                        if next == as_vec_of_char_refs("eight") {
                            chars_for_output.push(8);
                            idx += 1;
                        } else {
                            idx += 1;
                        }
                    }
                    Err(_err) => idx += 1,
                }
            }
            'n' => {
                let next = get_next_x(&chars_from_input, idx, 4);
                match next {
                    Ok(next) => {
                        if next == as_vec_of_char_refs("nine") {
                            chars_for_output.push(9);
                            idx += 1;
                        } else {
                            idx += 1;
                        }
                    }
                    Err(_err) => idx += 1,
                }
            }
            _ => idx += 1,
        }
    }
    return Ok(chars_for_output);
}

fn get_next_x(input: &Vec<char>, start: usize, x: usize) -> Result<Vec<char>, String> {
    if start > input.len() {
        return Err(format!(
            "Start point {} is out of bounds of vec of length {}",
            start,
            input.len()
        ));
    }
    if start + x > input.len() {
        return Err(format!(
            "{} + {} ({}) is out of bounds of vec of length {}",
            start,
            x,
            start + x,
            input.len()
        ));
    }
    let output: Vec<char> = input[start..start + x].iter().cloned().collect();
    return Ok(output);
}

fn as_vec_of_char_refs(input: &str) -> Vec<char> {
    return input.chars().collect::<Vec<_>>();
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
                fs::read_to_string("src/day_1/test_input_part_1.txt").expect("File read errored");
            let expected_output = 142;

            let actual_output = solve_part_1(&input).unwrap();

            assert_eq!(actual_output, expected_output);
        }
        #[test]
        fn test_worked_example_part_2() {
            use super::solve_part_2;
            use std::fs;

            let input =
                fs::read_to_string("src/day_1/test_input_part_2.txt").expect("File read errored");
            let expected_output = 281;

            let actual_output = solve_part_2(&input).unwrap();

            assert_eq!(actual_output, expected_output);
        }
    }
    mod test_find_first_int {
        #[test]
        fn successfully_returns_an_int() {
            use super::find_first_int_part_1;

            let test_string = "1";
            let result = find_first_int_part_1(&test_string);

            assert_eq!(result, Ok(1))
        }
        #[test]
        fn errors_when_no_ints() {
            use super::find_first_int_part_1;

            let test_string = "abc";
            let result = find_first_int_part_1(&test_string);

            assert!(result.is_err())
        }
        #[test]
        fn successfully_skips_none_int() {
            use super::find_first_int_part_1;

            let test_string = "abc1";
            let result = find_first_int_part_1(&test_string);

            assert_eq!(result, Ok(1))
        }
        #[test]
        fn only_returns_first_int() {
            use super::find_first_int_part_1;

            let test_string = "321";
            let result = find_first_int_part_1(&test_string);

            assert_eq!(result, Ok(3))
        }
    }
    mod test_find_last_int {
        #[test]
        fn successfully_returns_an_int() {
            use super::find_last_int_part_1;

            let test_string = "1";
            let result = find_last_int_part_1(&test_string);

            assert_eq!(result, Ok(1))
        }
        #[test]
        fn errors_when_no_ints() {
            use super::find_last_int_part_1;

            let test_string = "abc";
            let result = find_last_int_part_1(&test_string);

            assert!(result.is_err())
        }
        #[test]
        fn successfully_skips_none_int() {
            use super::find_last_int_part_1;

            let test_string = "1abc";
            let result = find_last_int_part_1(&test_string);

            assert_eq!(result, Ok(1))
        }
        #[test]
        fn only_returns_last_int() {
            use super::find_last_int_part_1;

            let test_string = "321";
            let result = find_last_int_part_1(&test_string);

            assert_eq!(result, Ok(1))
        }
    }
    mod test_get_next_x {
        #[test]
        fn successfully_returns_x() {
            use super::get_next_x;

            let test_input: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let result = get_next_x(&test_input, 2, 5);

            assert!(result.unwrap().len() == 5)
        }
        #[test]
        fn collects_correct_values() {
            use super::get_next_x;

            let test_input: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let result = get_next_x(&test_input, 2, 2);

            assert!(result.unwrap() == vec!['2', '3']);
        }
    }
    mod test_find_all_ints {
        #[test]
        fn successfully_returns_numerics() {
            use super::find_all_ints_part_2;

            let test = "0123456789";

            let result = find_all_ints_part_2(test);

            assert_eq!(result.unwrap(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        }
        #[test]
        fn successfully_returns_strings_as_numerics() {
            use super::find_all_ints_part_2;

            let test = "zero";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[0]);

            let test = "one";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[1]);

            let test = "two";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[2]);

            let test = "three";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[3]);

            let test = "four";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[4]);

            let test = "five";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[5]);

            let test = "six";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[6]);

            let test = "seven";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[7]);

            let test = "eight";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[8]);

            let test = "nine";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[9]);
        }
        #[test]
        fn handles_random_chars() {
            use super::find_all_ints_part_2;

            let test = "onextwoxxsxftxfoursix";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[1, 2, 4, 6]);
        }
        #[test]
        fn handles_mixed_numerics_and_chars() {
            use super::find_all_ints_part_2;

            let test = "one1twoxxsxft7foursix";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[1, 1, 2, 7, 4, 6]);
        }
        #[test]
        fn handles_examples() {
            use super::find_all_ints_part_2;

            let test = "two1nine";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[2, 1, 9]);

            let test = "eightwothree";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[8, 2, 3]);

            let test = "abcone2threexyz";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[1, 2, 3]);

            let test = "xtwone3four";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[2, 1, 3, 4]);

            let test = "4nineeightseven2";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[4, 9, 8, 7, 2]);

            let test = "zoneight234";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[1, 8, 2, 3, 4]);

            let test = "7pqrstsixteen";
            let result = find_all_ints_part_2(test);
            assert_eq!(result.unwrap(), &[7, 6]);

            
        }
    }
}
