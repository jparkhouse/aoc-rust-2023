use std::fs;
use bitvec::vec::BitVec;

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
    return Ok(0);
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

fn get_machine_part_numbers(grid: ThreeRowGrid) -> Result<Option<Vec<usize>>, String> {
    match grid.get_case() {
        ThreeRowGridCase::Empty => return Ok(None),
        ThreeRowGridCase::Invalid => return Err(String::from("Unable to parse Invalid ThreeRowGrid")),
        ThreeRowGridCase::TopRowOnly => return Ok(get_machine_part_numbers_from_top_row(row)),
        ThreeRowGridCase::TopAndMiddleRowOnly => return Ok(get_machine_part_numbers_from_top_and_middle_row(row)),
        ThreeRowGridCase::AllRows => return Ok(get_machine_part_numbers_from_all_rows(row)),
        
    };
}

fn get_machine_part_numbers_from_top_row(top_row: &Vec<char>) -> Option<Vec<usize>> {    
    todo!()
}

fn get_machine_part_numbers_from_top_and_middle_row(top_row: &Vec<char>, middle_row: &Vec<char>) -> Option<Vec<usize>> {    
    todo!()
}

fn get_machine_part_numbers_from_all_rows(top_row: &Vec<char>, middle_row: &Vec<char>, bottom_row: &Vec<char>) -> Option<Vec<usize>> {    
    todo!()
}

fn get_bitmasks(row: &Vec<char>) -> Result<(BitVec, BitVec), String> {
    
    let mut symbols = BitVec::with_capacity(141);
    let mut numbers = BitVec::with_capacity(141);

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
            CharType::FullStop => {},
            CharType::Numeric => numbers.set(index, true),
            CharType::Symbol => symbols.set(index, true),
        }
    }

    return Ok((symbols, numbers));
}

struct Number {
    start: usize,
    stop: usize,
    value: usize,
}

fn get_all_numbers(row: &Vec<char>, bitmask) -> Option<Vec<Number>> {
    todo!()
}