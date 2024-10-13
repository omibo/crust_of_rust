//! A module providing the `StrSplit` iterator for splitting string slices.
//!
//! This module defines the `StrSplit` struct, which implements the `Iterator` trait
//! to allow for iteration over substrings of a string slice, separated by a specified delimiter.
//!

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// An iterator over substrings of a string slice, separated by a delimiter string slice.
///
/// The `StrSplit` struct holds references to the original string slice (`remainder`)
/// and the delimiter string slice (`delimiter`). It iteratively yields slices of the original
/// string, split by the specified delimiter.
///
/// # Lifetimes
///
/// * `'a`: The lifetime of the string slices that `StrSplit` holds references to.
///   - The lifetime `'a` ensures that the `remainder` and `delimiter` fields are valid
///     for at least as long as the `StrSplit` instance.
///   - By tying the lifetime of the struct to the lifetime of the data it references,
///     we prevent the struct from outliving the data, ensuring memory safety.

#[derive(Debug)]
pub struct StrSplit<'a> {
    /// The remaining portion of the string to be split.
    remainder: &'a str,
    /// The delimiter used to split the string.
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    /// Creates a new `StrSplit` iterator over the given `haystack` string slice,
    /// using the specified `delimiter`.
    ///
    /// Both `haystack` and `delimiter` must live at least as long as the returned
    /// `StrSplit` iterator, which is ensured by the lifetime parameter `'a`.
    ///
    /// # Parameters
    ///
    /// - `haystack`: The string slice to be split.
    /// - `delimiter`: The string slice used as the delimiter.
    ///
    /// # Lifetimes
    ///
    /// * `'a`: The lifetime of the references to `haystack` and `delimiter`.
    ///   - Ensures that `haystack` and `delimiter` are valid for the lifetime of the `StrSplit` instance.
    ///   - The `new` function ties the lifetime of the input slices to the lifetime of the struct,
    ///     preventing the `StrSplit` from outliving the data it references.

    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    /// The type of items yielded by the iterator.
    ///
    /// # Lifetimes
    ///
    /// * `'a`: The lifetime of the substrings yielded by the iterator.
    ///   - Each item is a `&'a str`, a slice of the original `haystack`.
    ///   - This ensures that the substrings are valid as long as the original string slice is valid.
    type Item = &'a str;

    /// Advances the iterator and returns the next substring, or `None` if there are no more substrings.
    ///
    /// Each substring returned is a slice of the original `haystack` string slice, and thus has the lifetime `'a`.
    ///
    /// # Lifetimes
    ///
    /// * `'a`: The lifetime of the substrings yielded.
    ///   - Guarantees that each substring is valid for at least as long as the `haystack` string slice.
    ///   - Ensures that users of the iterator cannot obtain a substring that outlives the original data.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            // Slice up to the delimiter.
            let until_delim = &self.remainder[..next_delim];
            // Update remainder to exclude the processed part and the delimiter.
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delim)
        } else if !self.remainder.is_empty() {
            // Return the rest of the string if it's non-empty.
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        } else {
            // Return None when there is no more data to process.
            None
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}
