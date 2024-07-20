use std::{collections::HashMap, iter::zip};


pub struct MapTo {
    map: HashMap<usize, usize>,
    pub input: String,
    pub output: String,
}

impl MapTo {
    pub fn new(input: &str, output: &str) -> MapTo {
        return MapTo {
            input: input.to_string(),
            output: output.to_string(),
            map: HashMap::new(),
        };
    }

    pub fn get(&self, input: usize) -> usize {
        let output = match self.map.get(&input) {
            Some(result) => *result,
            None => input,
        };
        return output;
    }
    
    pub fn add_map(&mut self, input: usize, output: usize) {
        self.map.insert(input, output);
    }

    pub fn add_range_map(&mut self, start_input: usize, start_output: usize, steps: usize) {
        // Reserve capacity to prevent multiple allocations
        self.map.reserve(steps);

        let ranges = zip(
            start_input..start_input + steps,
            start_output..start_output + steps,
        );

        for (input, output) in ranges {
            self.map.insert(input, output);
        }
    }
}