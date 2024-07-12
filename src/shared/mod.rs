/// Converts a string slice into a vector of characters.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input string.
///
/// # Returns
///
/// A `Vec<char>` containing each character from the input string.
///
/// # Examples
///
/// ```
/// let input = "hello";
/// let chars = get_chars(input);
/// assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o']);
///
/// let input = "こんにちは";
/// let chars = get_chars(input);
/// assert_eq!(chars, vec!['こ', 'ん', 'に', 'ち', 'は']);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Notes
///
/// This function handles all valid UTF-8 characters correctly, including
/// non-ASCII characters.
pub fn get_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub struct DayResult {
    pub part_1: usize,
    pub part_2: usize,
}