use crate::day_5::linear_range::{LinearRange, LinearRangeComparison};
use std::collections::HashMap;
use thiserror::Error;

pub struct LinearRangeBinarySearchTree {
    tree: HashMap<u64, LinearRange>,
}

#[derive(Debug, Error)]
pub enum LinearRangeBinarySearchTreeError {
    #[error("Overflowing key value")]
    OverflowInKeyValue,

    #[error("Attempted to calculate the extended range of two non-intersecting ranges")]
    ExtendedRangeFromNonIntersectingRanges,
}

type LRBSTResult<T> = Result<T, LinearRangeBinarySearchTreeError>;

impl LinearRangeBinarySearchTree {
    pub fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    /// helper function to recursively construct and return a subtree
    fn construct_subtree(&mut self, input_vec: &[LinearRange]) -> LRBSTResult<()> {
        if input_vec.is_empty() {
            return Ok(());
        }

        let mid = input_vec.len() / 2;

        let node = &input_vec[mid];
        self.unbalanced_insert(node.clone())?;

        // Recursively construct left and right subtrees
        self.construct_subtree(&input_vec[..mid])?;
        self.construct_subtree(&input_vec[mid + 1..])?;
        return Ok(());
    }

    /// inserts a value into the binary tree with no balancing
    pub fn unbalanced_insert(&mut self, inserting_range: LinearRange) -> LRBSTResult<()> {
        if self.tree.is_empty() {
            self.tree.insert(1, inserting_range);
            return Ok(());
        }

        use LinearRangeComparison::*;

        let mut constructed_key = 1 as u64;
        while let Some(current_node) = self.tree.get(&constructed_key) {
            match inserting_range.compare(current_node) {
                StrictlyLessThan => {
                    constructed_key = manage_bitshift(constructed_key, false)
                        .ok_or(LinearRangeBinarySearchTreeError::OverflowInKeyValue)?
                }
                LessThanWithIntersection | GreaterThanWithIntersection => {
                    let extended_range = inserting_range.get_extended_range(current_node).ok_or(
                        LinearRangeBinarySearchTreeError::ExtendedRangeFromNonIntersectingRanges,
                    )?;
                    self.tree.insert(constructed_key, extended_range);
                    self.merge_children(constructed_key)?;
                    return Ok(());
                }
                Equal | ContainsOther | ContainedInOther => return Ok(()), // range already contained in tree, no more to do
                StrictlyGreaterThan => {
                    constructed_key = manage_bitshift(constructed_key, true)
                        .ok_or(LinearRangeBinarySearchTreeError::OverflowInKeyValue)?
                }
            }
        }
        // if we make it through all of that without breaking, we have now constructed a key to an empty node
        // now we just insert inserting_range
        self.tree.insert(constructed_key, inserting_range);
        return Ok(());
    }

    /// helper function to manage the recursive implementation of get_sorted_vec
    fn in_order_traversal(&self, key: u64) -> Vec<LinearRange> {
        if let Some(current_node) = self.tree.get(&key) {
            let mut result = self.in_order_traversal(key << 1);
            result.push(current_node.clone());
            let right_result = self.in_order_traversal((key << 1) + 1);
            result.extend(right_result);
            return result;
        } else {
            // empty node returns empty vec
            return Vec::new();
        }
    }

    /// helper function to manage the recursive implementation of get_sorted_vec
    fn in_order_traversal_of_keys(&self, key: u64) -> Vec<u64> {
        if let Some(_) = self.tree.get(&key) {
            let mut result = self.in_order_traversal_of_keys(key << 1);
            result.push(key);
            let right_result = self.in_order_traversal_of_keys((key << 1) + 1);
            result.extend(right_result);
            return result;
        } else {
            // empty node returns empty vec
            return Vec::new();
        }
    }

    /// returns the tree as a sorted Vec
    pub fn get_sorted_vec(&self) -> Vec<LinearRange> {
        return self.in_order_traversal(1);
    }

    /// helper function to manage pruning mergable ranges
    fn merge_children(&mut self, key: u64) -> LRBSTResult<()> {
        // collect all ranges
        let contents = self.in_order_traversal(key);
        // collect all keys
        let keys = self.in_order_traversal_of_keys(key);
        // iterate over the keys and clear the tree
        for k in keys {
            self.tree.remove(&k);
        }
        // construct a new subtree
        self.construct_subtree(&contents)?;
        Ok(())
    }
}

fn manage_bitshift(key: u64, right: bool) -> Option<u64> {
    if key & (1 << 63) != 0 {
        return None;
    }
    if right {
        return Some((key << 1) + 1);
    }
    return Some(key << 1);
}
