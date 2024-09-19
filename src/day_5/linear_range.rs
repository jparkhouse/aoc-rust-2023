use thiserror::Error;

use super::{map_tuple::MapTuple, seed_set::SeedSet};

#[derive(Debug, Clone, Copy)]
pub struct LinearRange {
    pub start: usize,
    pub steps: usize,
}

#[derive(Debug, Error)]
pub enum LinearRangeError {
    #[error("Range provided exceeds usize max")]
    Overflow,
    #[error("Zero steps results in null range which is not allowed")]
    ZeroSteps,
}

pub enum LinearRangeComparison {
    StrictlyLessThan,
    LessThanWithIntersection,
    Equal,
    ContainsOther,
    ContainedInOther,
    GreaterThanWithIntersection,
    StrictlyGreaterThan,
}

impl LinearRange {
    pub fn new(start: usize, steps: usize) -> Result<Self, LinearRangeError> {
        if let None = start.checked_add(steps) {
            return Err(LinearRangeError::Overflow);
        }
        if steps == 0 {
            return Err(LinearRangeError::ZeroSteps);
        }
        return Ok(Self {
            start: start,
            steps: steps,
        });
    }

    pub fn from_map_tuple_source(map_tuple: &MapTuple) -> Result<Self, LinearRangeError> {
        Self::new(map_tuple.source_range_start, map_tuple.range_length)
    }

    pub fn from_map_tuple_destination(map_tuple: &MapTuple) -> Result<Self, LinearRangeError> {
        Self::new(map_tuple.source_range_start, map_tuple.range_length)
    }

    pub fn from_seed_set(seed_set: &SeedSet) -> Result<Self, LinearRangeError> {
        Self::new(seed_set.start, seed_set.steps)
    }

    pub fn get_last_value(&self) -> usize {
        self.start + self.steps - 1
    }

    pub fn contains(&self, value: usize) -> bool {
        (value >= self.start) && (value <= self.get_last_value())
    }

    pub fn intersects(&self, other: &Self) -> bool {
        !(self.start > other.get_last_value() || self.get_last_value() < other.start)
    }

    pub fn compare(&self, other: &Self) -> LinearRangeComparison {
        use LinearRangeComparison::*;

        if self.start == other.start && self.get_last_value() == other.get_last_value() {
            return Equal;
        }
        if self.start <= other.start && self.get_last_value() >= other.get_last_value() {
            return ContainsOther;
        }
        if other.start <= self.start && other.get_last_value() >= self.get_last_value() {
            return ContainedInOther;
        }
        match (self.start < other.start, self.intersects(other)) {
            (true, false) => StrictlyLessThan,
            (true, true) => LessThanWithIntersection,
            (false, true) => GreaterThanWithIntersection,
            (false, false) => StrictlyGreaterThan,
        }
    }

    pub fn get_extended_range(&self, other: &Self) -> Option<Self> {
        use LinearRangeComparison::*;
        match self.compare(other) {
            Equal | ContainsOther => Some(self.clone()),
            StrictlyGreaterThan | StrictlyLessThan => None,
            LessThanWithIntersection => {
                return Some(Self {
                    start: other.start,
                    steps: self.get_last_value() - other.start + 1,
                })
            }
            GreaterThanWithIntersection => {
                return Some(Self {
                    start: self.start,
                    steps: other.get_last_value() - self.start + 1,
                })
            }
            ContainedInOther => Some(other.clone()),
        }
    }
}
