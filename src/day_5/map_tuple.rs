use crate::day_5::SearchDirection;
use std::cmp::Ordering;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MapTuple {
    pub destination_range_start: usize,
    pub source_range_start: usize,
    pub range_length: usize,
}

#[derive(Debug, Error)]
pub enum MapTupleError {
    #[error("Attempted to calculate the output for a value that is not in range")]
    DoesNotContain,

    #[error("Source range overlaps")]
    Overlap,
}

impl MapTuple {
    pub fn new(
        destination_range_start: usize,
        source_range_start: usize,
        range_length: usize,
    ) -> Self {
        Self {
            destination_range_start: destination_range_start,
            source_range_start: source_range_start,
            range_length: range_length,
        }
    }

    pub fn contains(&self, value: usize) -> bool {
        value >= self.source_range_start && value < self.source_range_start + self.range_length
    }

    pub fn calculate_output(&self, value: usize) -> Result<usize, MapTupleError> {
        if self.contains(value) {
            Ok(self.destination_range_start + (value - self.source_range_start))
        } else {
            Err(MapTupleError::DoesNotContain)
        }
    }

    pub fn does_not_overlap(&self, other: &Self) -> bool {
        return self.source_range_start + self.range_length <= other.source_range_start
            || other.source_range_start + other.range_length <= self.source_range_start;
    }

    pub fn compare_without_overlap(&self, other: &Self) -> Result<Ordering, MapTupleError> {
        if !self.does_not_overlap(other) {
            return Err(MapTupleError::Overlap);
        }
        match self.source_range_start > other.source_range_start {
            true => return Ok(Ordering::Greater),
            false => return Ok(Ordering::Less),
        }
    }

    pub fn get_search_direction(&self, value: usize) -> SearchDirection {
        match (self.contains(value), value < self.source_range_start) {
            (true, _) => return SearchDirection::Contains,
            (false, true) => return SearchDirection::Less,
            (false, false) => return SearchDirection::Greater,
        }
    }

    /* /// calculates and returns the
        pub fn compose(&self, other: &Self) -> Vec<Self> {
            if self.does_not_overlap(other) {
                // if they do not overlap, they do not interact
                return vec![self.clone(), other.clone()];
            }

            struct TupleRange {
                start: usize,
                stop: usize,
                steps: usize,
            }

            impl TupleRange {
                fn new(start: usize, stop: usize) -> Option<Self> {
                    if stop - start <= 0 {
                        return None;
                    }
                    return Some(TupleRange {
                        start: start,
                        stop: stop,
                        steps: stop - start,
                    });
                }

                fn intersection_range(&self, other: &Self) -> Option<Self> {
                    if self.start >= other.stop || self.stop <= other.start {
                        return None; // no intersection
                    }
                    match (self.start <= other.start, self.stop < other.stop) {
                        (true, true) => {
                            // self, self + other, other
                            return TupleRange::new(other.start, self.stop);
                        }
                        (false, true) => {
                            // other, self + other, other
                            return TupleRange::new(self.start, self.stop);
                        }
                        (true, false) => {
                            // self, other + self, self
                            return TupleRange::new(other.start, other.stop);
                        }
                        (false, false) => {
                            // other, other + self, self
                            return TupleRange::new(self.start, other.stop);
                        }
                    }
                }

                fn inclusive_intersection_range(&self, other: &Self) -> Vec<TupleRange> {
                    let mut output: Vec<TupleRange> = Vec::new();
                    if self.start >= other.stop || self.stop <= other.start {
                        // no intersection
                        if let Some(range) = TupleRange::new(self.start, self.stop) {
                            output.push(range);
                        }
                        if let Some(range) = TupleRange::new(other.start, other.stop) {
                            output.push(range);
                        }
                        return output;
                    }
                    match (self.start <= other.start, self.stop < other.stop) {
                        (true, true) => {
                            // self, self + other, other

                            if let Some(range) = TupleRange::new(self.start, other.start) {
                                output.push(range);
                            }
                            if let Some(range) = TupleRange::new(other.start, self.stop) {
                                output.push(range);
                            }
                            if let Some(range) = TupleRange::new(self.stop, other.stop) {
                                output.push(range);
                            }
                            return output;
                        }
                        (false, true) => {
                            // other, self + other, other
                            if let Some(range) = TupleRange::new(other.start, self.start) {
                                output.push(range);
                            }
                            if let Some(range) = TupleRange::new(self.start, self.stop) {
                                output.push(range);
                            }
                            if let Some(range) = TupleRange::new(self.stop, other.stop) {
                                output.push(range);
                            }
                            return output;
                        }
                        (true, false) => {
                            // self, other + self, self
                            if let Some(range) = TupleRange::new(self.start, other.start) {
                                output.push(range);
                            }
                            if let Some(range) = TupleRange::new(other.start, other.stop) {
                                output.push(range);
                            }
                            if let Some(range) = TupleRange::new(other.stop, self.stop) {
                                output.push(range);
                            }
                            return output;
                        }
                        (false, false) => {
                            // other, other + self, self
                            if let Some(range) = TupleRange::new(other.start, self.start) {
                                output.push(range);
                            }
                            if let Some(range) = TupleRange::new(self.start, other.stop) {
                                output.push(range);
                            }
                            if let Some(range) = TupleRange::new(other.stop, self.stop) {
                                output.push(range);
                            }
                            return output;
                        }
                    }
                }

                fn range_map(&self, diff: isize) -> Result<Self, String> {
                    if self.start as isize + diff < 0 {
                        return Err("Cannot create negative values in range".to_string());
                    } else {
                        if let Some(range) = TupleRange::new(
                            (self.start as isize + diff) as usize,
                            (self.stop as isize + diff) as usize,
                        ) {
                            return Ok(range);
                        } else {
                            return Err("Invalid range".to_string());
                        }
                    }
                }
            }

            let self_input_range = TupleRange::new(
                self.source_range_start,
                self.source_range_start + self.range_length,
            )
            .unwrap();
            let self_output_range = TupleRange::new(
                self.destination_range_start,
                self.destination_range_start + self.range_length,
            )
            .unwrap();

            let other_input_range = TupleRange::new(
                other.source_range_start,
                other.source_range_start + other.range_length,
            )
            .unwrap();
            let other_output_range = TupleRange::new(
                other.destination_range_start,
                other.destination_range_start + other.range_length,
            )
            .unwrap();

            let a = self_input_range;
            let ab = self_output_range.intersection_range(&other_input_range).range_map(other.source_range_start);
            let b = other_input_range
        } */
}
