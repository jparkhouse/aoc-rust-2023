use std::fs;

fn main() {}

pub fn solve() -> Result<(usize, usize), String> {
    let input = match fs::read_to_string("src/day_3/input.txt") {
        Ok(result) => result,
        Err(err) => return Err(format!("Error in reading file: {}", err)),
    };

    let part_1 = solve_part_1(&input)?;
    let part_2 = solve_part_2(&input)?;

    return Ok((part_1, part_2));
}

fn solve_part_1(input: &str) -> Result<usize, String> {
    let mut output: usize = 0;
    let mut grid = ThreeRowGrid::new(None, None, None);

    for line in input.lines() {
        grid.insert_next_row(line);
        output += match get_machine_part_numbers(&grid) {
            Ok(result) => match result {
                Some(vec) => vec.into_iter().sum(),
                None => 0,
            },
            Err(err) => return Err(format!("Error in get_machine_part_numbers: {err}")),
        }
    }

    return Ok(output);
}

fn solve_part_2(input: &str) -> Result<usize, String> {
    return Ok(0);
}

fn get_chars(input: &str) -> Vec<char> {
    return input
        .as_bytes()
        .iter()
        .map(|item| {
            return item.clone() as char;
        })
        .collect();
}

struct ThreeRowGrid {
    top_row: Option<Vec<char>>,
    middle_row: Option<Vec<char>>,
    bottom_row: Option<Vec<char>>,
}

enum ThreeRowGridCase {
    TopRowOnly,
    TopAndMiddleRowOnly,
    AllRows,
    Empty,
    Invalid,
}

impl ThreeRowGrid {
    fn new(
        top_row: Option<&str>,
        middle_row: Option<&str>,
        bottom_row: Option<&str>,
    ) -> ThreeRowGrid {
        let mut collector: Vec<&str> = Vec::new();
        match top_row {
            Some(row) => collector.push(row),
            _ => {}
        }
        match middle_row {
            Some(row) => collector.push(row),
            _ => {}
        }
        match bottom_row {
            Some(row) => collector.push(row),
            _ => {}
        }
        match collector.len() {
            1 => {
                return ThreeRowGrid {
                    top_row: Some(get_chars(collector[0])),
                    middle_row: None,
                    bottom_row: None,
                }
            }
            2 => {
                return ThreeRowGrid {
                    top_row: Some(get_chars(collector[0])),
                    middle_row: Some(get_chars(collector[1])),
                    bottom_row: None,
                }
            }
            3 => {
                return ThreeRowGrid {
                    top_row: Some(get_chars(collector[0])),
                    middle_row: Some(get_chars(collector[1])),
                    bottom_row: Some(get_chars(collector[2])),
                }
            }
            _ => {
                // return empty
                return ThreeRowGrid {
                    top_row: None,
                    middle_row: None,
                    bottom_row: None,
                };
            }
        }
    }

    fn get_case(&self) -> ThreeRowGridCase {
        let top_row: bool = match &self.top_row {
            Some(row) => true,
            None => false,
        };
        let middle_row: bool = match &self.middle_row {
            Some(row) => true,
            None => false,
        };
        let bottom_row: bool = match &self.bottom_row {
            Some(row) => true,
            None => false,
        };

        match (top_row, middle_row, bottom_row) {
            (true, true, true) => return ThreeRowGridCase::AllRows,
            (true, true, false) => return ThreeRowGridCase::TopAndMiddleRowOnly,
            (true, false, false) => return ThreeRowGridCase::TopRowOnly,
            (false, false, false) => return ThreeRowGridCase::Empty,
            _ => return ThreeRowGridCase::Invalid,
        }
    }

    fn insert_next_row(&mut self, row: &str) -> Result<(), String> {
        match self.get_case() {
            ThreeRowGridCase::Empty => {
                self.top_row = Some(get_chars(row));
                return Ok(());
            }
            ThreeRowGridCase::TopRowOnly => {
                self.middle_row = Some(get_chars(row));
                return Ok(());
            }
            ThreeRowGridCase::TopAndMiddleRowOnly => {
                self.bottom_row = Some(get_chars(row));
                return Ok(());
            }
            ThreeRowGridCase::AllRows => {
                self.top_row = self.middle_row.take(); // yoink
                self.middle_row = self.bottom_row.take();
                self.bottom_row = Some(get_chars(row));
                return Ok(());
            }
            ThreeRowGridCase::Invalid => {
                return Err(String::from("Unable to add row to an invalid ThreeRowGrid"))
            }
        }
    }
}

fn get_machine_part_numbers(grid: &ThreeRowGrid) -> Result<Option<Vec<usize>>, String> {
    match grid.get_case() {
        ThreeRowGridCase::Empty => return Ok(None),
        ThreeRowGridCase::Invalid => {
            return Err(String::from("Unable to parse Invalid ThreeRowGrid"))
        }
        ThreeRowGridCase::TopRowOnly => {
            return match get_machine_part_numbers_from_top_row(grid.top_row.as_ref().unwrap()) {
                Ok(result) => Ok(result),
                Err(err) => Err(err),
            }
        }
        ThreeRowGridCase::TopAndMiddleRowOnly => {
            return match get_machine_part_numbers_from_top_and_middle_row(
                grid.top_row.as_ref().unwrap(),
                grid.middle_row.as_ref().unwrap(),
            ) {
                Ok(result) => Ok(result),
                Err(err) => Err(err),
            }
        }
        ThreeRowGridCase::AllRows => {
            return match get_machine_part_numbers_from_all_rows(
                grid.top_row.as_ref().unwrap(),
                grid.middle_row.as_ref().unwrap(),
                grid.bottom_row.as_ref().unwrap(),
            ) {
                Ok(result) => Ok(result),
                Err(err) => Err(err),
            }
        }
    };
}

fn get_machine_part_numbers_from_top_row(
    top_row: &Vec<char>,
) -> Result<Option<Vec<usize>>, String> {
    let (symbol_bitmask, num_bitmask) = match get_bitmasks(top_row) {
        Ok(result) => result,
        Err(err) => return Err(format!("Error from bitmasks: {err}")),
    };

    let numbers = match get_all_numbers(top_row, num_bitmask) {
        Ok(result) => match result {
            Some(nums) => nums,
            None => return Ok(None),
        },
        Err(err) => return Err(format!("Error in get_all_numbers: {err}")),
    };

    let output: Vec<usize> = numbers
        .into_iter()
        .filter(|num| {
            let pre = match symbol_bitmask.get(num.start - 1) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let post = match symbol_bitmask.get(num.stop) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            return pre || post;
        })
        .map(|num| num.value)
        .collect::<Vec<_>>();

    match output.len() {
        0 => return Ok(None),
        _ => return Ok(Some(output)),
    };
}

fn get_machine_part_numbers_from_top_and_middle_row(
    top_row: &Vec<char>,
    middle_row: &Vec<char>,
) -> Result<Option<Vec<usize>>, String> {
    let t_symbol_bitmask = match get_bitmasks(top_row) {
        Ok(result) => result.0,
        Err(err) => return Err(format!("Error from bitmasks: {err}")),
    };

    let (m_symbol_bitmask, m_num_bitmask) = match get_bitmasks(middle_row) {
        Ok(result) => result,
        Err(err) => return Err(format!("Error from bitmasks: {err}")),
    };

    let numbers = match get_all_numbers(top_row, m_num_bitmask) {
        Ok(result) => match result {
            Some(nums) => nums,
            None => return Ok(None),
        },
        Err(err) => return Err(format!("Error in get_all_numbers: {err}")),
    };

    let output: Vec<usize> = numbers
        .into_iter()
        .filter(|num| {
            let pre = match m_symbol_bitmask.get(num.start - 1) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let post = match m_symbol_bitmask.get(num.stop) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let above = t_symbol_bitmask[num.start..num.stop]
                .iter()
                .any(|val| val == &true);
            let upper_left = match t_symbol_bitmask.get(num.start - 1) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let upper_right = match t_symbol_bitmask.get(num.stop) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            return pre || post || upper_left || above || upper_right;
        })
        .map(|num| num.value)
        .collect::<Vec<_>>();

    match output.len() {
        0 => return Ok(None),
        _ => return Ok(Some(output)),
    };
}

fn get_machine_part_numbers_from_all_rows(
    top_row: &Vec<char>,
    middle_row: &Vec<char>,
    bottom_row: &Vec<char>,
) -> Result<Option<Vec<usize>>, String> {
    let t_symbol_bitmask = match get_bitmasks(top_row) {
        Ok(result) => result.0,
        Err(err) => return Err(format!("Error from bitmasks: {err}")),
    };

    let (m_symbol_bitmask, m_num_bitmask) = match get_bitmasks(middle_row) {
        Ok(result) => result,
        Err(err) => return Err(format!("Error from bitmasks: {err}")),
    };

    let b_symbol_bitmask = match get_bitmasks(bottom_row) {
        Ok(result) => result.0,
        Err(err) => return Err(format!("Error from bitmasks: {err}")),
    };

    let numbers = match get_all_numbers(top_row, m_num_bitmask) {
        Ok(result) => match result {
            Some(nums) => nums,
            None => return Ok(None),
        },
        Err(err) => return Err(format!("Error in get_all_numbers: {err}")),
    };

    let output: Vec<usize> = numbers
        .into_iter()
        .filter(|num| {
            let pre = match m_symbol_bitmask.get(num.start - 1) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let post = match m_symbol_bitmask.get(num.stop) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let above = t_symbol_bitmask[num.start..num.stop]
                .iter()
                .any(|val| val == &true);
            let upper_left = match t_symbol_bitmask.get(num.start - 1) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let upper_right = match t_symbol_bitmask.get(num.stop) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let below = b_symbol_bitmask[num.start..num.stop]
                .iter()
                .any(|val| val == &true);
            let lower_left = match b_symbol_bitmask.get(num.start - 1) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            let lower_right = match b_symbol_bitmask.get(num.stop) {
                Some(val) => match val {
                    true => true,
                    false => false,
                },
                None => false,
            };
            return pre
                || post
                || upper_left
                || above
                || upper_right
                || lower_left
                || below
                || lower_right;
        })
        .map(|num| num.value)
        .collect::<Vec<_>>();

    match output.len() {
        0 => return Ok(None),
        _ => return Ok(Some(output)),
    };
}

fn get_bitmasks(row: &Vec<char>) -> Result<(Vec<bool>, Vec<bool>), String> {
    let mut symbols: Vec<bool> = vec![false; row.len()];
    let mut numbers: Vec<bool> = vec![false; row.len()];

    enum CharType {
        FullStop,
        Numeric,
        Symbol,
    }

    fn get_char_type(input: &char) -> CharType {
        match input {
            '.' => return CharType::FullStop,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => return CharType::Numeric,
            _ => return CharType::Symbol,
        }
    }

    for (index, ch) in row.iter().enumerate() {
        match get_char_type(ch) {
            CharType::FullStop => {}
            CharType::Numeric => numbers[index] = true,
            CharType::Symbol => symbols[index] = true,
        }
    }

    return Ok((symbols, numbers));
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

struct Number {
    start: usize,
    stop: usize,
    value: usize,
}

fn get_all_numbers(row: &Vec<char>, bitmask: Vec<bool>) -> Result<Option<Vec<Number>>, String> {
    let mut output: Vec<Number> = Vec::new();

    let mut i: usize = 0;

    while i < row.len() {
        match bitmask.get(i) {
            Some(val) => match val {
                true => {
                    let mut j = 1;
                    while match bitmask.get(i + j) {
                        Some(result) => match result {
                            true => true,
                            false => false,
                        },
                        None => {
                            if i + j == row.len() {
                                false // end of row
                            } else {
                                return Err(String::from("Accessed a bit value out of scope"));
                            }
                        }
                    } {
                        j += 1;
                    }
                    output.push(Number {
                        start: i,
                        stop: i + j,
                        value: row[i..i + j]
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(index, value)| {
                                get_int_part_1(value).unwrap() * 10u32.pow(index as u32) as usize
                            })
                            .sum(),
                    });
                    i += j;
                }
                false => i += 1,
            },
            None => return Err(String::from("Accessed a bit value out of scope")),
        }
    }

    match output.len() {
        0 => return Ok(None),
        _ => return Ok(Some(output)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_get_bitmasks {
        use core::panic;

        #[test]
        fn returns_all_false_with_full_stops() {
            use super::get_bitmasks;
            use super::get_chars;

            let row = get_chars(".......");

            let result = match get_bitmasks(&row) {
                Ok(result) => result,
                Err(_err) => panic!(),
            };

            assert!(result.0.iter().all(|val| val == &false));
            assert!(result.1.iter().all(|val| val == &false));
            assert_eq!(row.len(), result.0.len());
            assert_eq!(row.len(), result.1.len());
        }

        #[test]
        fn finds_symbols() {
            use super::get_bitmasks;
            use super::get_chars;

            let row = get_chars(".*.%.$.");

            let result = match get_bitmasks(&row) {
                Ok(result) => result,
                Err(_err) => panic!(),
            };

            assert_eq!(result.0, vec![false, true, false, true, false, true, false]);
            assert!(result.1.iter().all(|val| val == &false));
            assert_eq!(row.len(), result.0.len());
            assert_eq!(row.len(), result.1.len());
        }

        #[test]
        fn finds_numbers() {
            use super::get_bitmasks;
            use super::get_chars;

            let row = get_chars(".12.");

            let result = match get_bitmasks(&row) {
                Ok(result) => result,
                Err(_err) => panic!(),
            };

            assert!(result.0.iter().all(|val| val == &false));
            assert_eq!(result.1, vec![false, true, true, false]);
            assert_eq!(row.len(), result.0.len());
            assert_eq!(row.len(), result.1.len());
        }

        #[test]
        fn finds_both() {
            use super::get_bitmasks;
            use super::get_chars;

            let row = get_chars(".1*.");

            let result = match get_bitmasks(&row) {
                Ok(result) => result,
                Err(_err) => panic!(),
            };

            assert_eq!(result.0, vec![false, false, true, false]);
            assert_eq!(result.1, vec![false, true, false, false]);
            assert_eq!(row.len(), result.0.len());
            assert_eq!(row.len(), result.1.len());
        }
    }
    mod test_get_all_numbers {

        #[test]
        fn finds_no_numbers() {
            use super::get_all_numbers;
            use super::get_bitmasks;
            use super::get_chars;

            let row = get_chars("......");

            let bitmask = match get_bitmasks(&row) {
                Ok(result) => result.1,
                Err(_err) => panic!(),
            };

            let result = match get_all_numbers(&row, bitmask) {
                Ok(result) => result,
                Err(_err) => panic!(),
            };

            assert!(result.is_none())
        }

        #[test]
        fn can_find_a_number() {
            use super::get_all_numbers;
            use super::get_bitmasks;
            use super::get_chars;

            let row = get_chars(".12...");

            let bitmask = match get_bitmasks(&row) {
                Ok(result) => result.1,
                Err(_err) => panic!(),
            };

            let result = match get_all_numbers(&row, bitmask) {
                Ok(result) => result.unwrap(),
                Err(_err) => panic!(),
            };

            assert_eq!(result.len(), 1);
            assert_eq!(result[0].start, 1);
            assert_eq!(result[0].stop, 3);
            assert_eq!(result[0].value, 12);
        }

        #[test]
        fn can_find_multiple_numbers() {
            use super::get_all_numbers;
            use super::get_bitmasks;
            use super::get_chars;

            let row = get_chars(".32.623..45");

            let bitmask = match get_bitmasks(&row) {
                Ok(result) => result.1,
                Err(_err) => panic!(),
            };

            let o_result = match get_all_numbers(&row, bitmask) {
                Ok(result) => result,
                Err(err) => panic!("error from get_all_numbers: {err}"),
            };

            let result = match o_result {
                Some(res) => res,
                None => panic!("no numbers found"),
            };

            assert_eq!(result.len(), 3);
            assert_eq!(
                result.iter().map(|x| x.value as i32).collect::<Vec<_>>(),
                vec![32, 623, 45]
            )
        }
    }
}
