use crate::day_5::map_tuple::MapTuple;
use crate::day_5::SearchDirection;
use std::{cmp::Ordering, collections::HashMap};
use thiserror::Error;

pub struct MapBinarySearchTree {
    tree: HashMap<u64, MapTuple>,
}

#[derive(Debug, Error)]
pub enum MapBinarySearchTreeError {
    #[error("Attempted to use overlapping MapTuples")]
    OverlappingMapTuples,

    #[error("Overflowing key value")]
    OverflowInKeyValue,
}

impl MapBinarySearchTree {
    pub fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    /// constructs a balanced tree from a Vec of MapTuples
    pub fn from_vec(mut input_vec: Vec<MapTuple>) -> Result<Self, MapBinarySearchTreeError> {
        if input_vec.is_empty() {
            return Ok(MapBinarySearchTree::new());
        }
        input_vec.sort_by(|a, b| a.compare_without_overlap(b).unwrap());

        let middle = input_vec.len() / 2;
        let root = &input_vec[middle];

        let mut new_tree = MapBinarySearchTree::new();
        new_tree.unbalanced_insert(root.to_owned())?;

        new_tree.construct_subtree(&input_vec[..middle])?;
        new_tree.construct_subtree(&input_vec[middle + 1..])?;
        return Ok(new_tree);
    }

    /// helper function to recursively construct and return a subtree
    fn construct_subtree(
        &mut self,
        input_vec: &[MapTuple],
    ) -> Result<(), MapBinarySearchTreeError> {
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
    pub fn unbalanced_insert(
        &mut self,
        map_tuple: MapTuple,
    ) -> Result<(), MapBinarySearchTreeError> {
        /// helper function to compare two MapTuples, returning true if B > A, false if B < A, and raising an error if they overlap
        fn compare_map_tuples(
            map_tuple_a: &MapTuple,
            map_tuple_b: &MapTuple,
        ) -> Result<bool, MapBinarySearchTreeError> {
            if !map_tuple_a.does_not_overlap(map_tuple_b) {
                return Err(MapBinarySearchTreeError::OverlappingMapTuples);
            }
            match map_tuple_b.compare_without_overlap(map_tuple_a) {
                Ok(Ordering::Greater) => Ok(true),
                Ok(Ordering::Less) => Ok(false),
                Ok(Ordering::Equal) | Err(_) => {
                    Err(MapBinarySearchTreeError::OverlappingMapTuples)
                }
            }
        }
        if self.tree.is_empty() {
            self.tree.insert(1, map_tuple);
            return Ok(());
        }

        let mut key_construction: u64 = 1;
        while let Some(node) = self.tree.get(&key_construction) {
            if key_construction & (1 << 63) != 0 {
                return Err(MapBinarySearchTreeError::OverflowInKeyValue);
            }
            match compare_map_tuples(node, &map_tuple)? {
                true => key_construction = (key_construction << 1) + 1,
                false => key_construction = key_construction << 1,
            }
        }

        self.tree.insert(key_construction, map_tuple);
        return Ok(());
    }

    /// traverses the binary tree to try and find the MapTuple that contains the source_input value.
    /// if a MapTuple is found, then we calculate the destination output and return;
    /// if there is no MapTuple in the tree that contains the source_input value, it is unmapped, and we return the same value;
    pub fn get_mapped_value(&self, source_input: usize) -> usize {
        let mut key_constructor = 1 as u64;
        while let Some(node) = self.tree.get(&key_constructor) {
            match node.get_search_direction(source_input) {
                SearchDirection::Contains => {
                    return node
                        .calculate_output(source_input)
                        .expect("returned enum Contains when it did not contain")
                }
                SearchDirection::Greater => key_constructor = (key_constructor << 1) + 1, // navigate right
                SearchDirection::Less => key_constructor = key_constructor << 1, // navigate left
            }
        } // if we have not returned yet, it is because we have not found the value in any of our MapTuples
        return source_input;
    }

    /// helper function to manage the recursive implementation of get_sorted_vec
    fn in_order_traversal(&self, key: u64) -> Vec<MapTuple> {
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

    /// returns the tree as a sorted Vec
    pub fn get_sorted_vec(&self) -> Vec<MapTuple> {
        return self.in_order_traversal(1);
    }
}
