#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    /// The remaining portion of the string to be split.
    remainder: Option<&'haystack str>,
    /// The delimiter used to split the string.
    delimiter: D,
}

// Explanation:
// The `StrSplit` struct is a string splitter that holds a reference to the string being split (`remainder`)
// and a delimiter (`delimiter`). The lifetime `'haystack` ensures that `remainder` does not outlive the
// original string slice (`haystack`). This is crucial because we are borrowing from `haystack`, and Rust's
// lifetime system ensures memory safety by preventing dangling references.

// Lifetimes:
// - `'haystack`: Marks the lifetime of the borrowed string slice. All references to `&'haystack str`
//   must not outlive the `haystack` they borrow from.

impl<'haystack, D> StrSplit<'haystack, D> {
    /// Creates a new `StrSplit` iterator.
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    /// Finds the next occurrence of the delimiter in the string `s`.
    /// Returns the start and end indices of the delimiter.
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    /// Advances the iterator and returns the next split string slice.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            // If there is a remainder to process
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                // If the delimiter is found
                let until_delim = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delim)
            } else {
                // No more delimiters found; return the remainder
                self.remainder.take()
            }
        } else {
            // No remainder left; iteration is complete
            None
        }
    }
}

// Lifetimes in `Iterator` implementation:
// - The `Item` type is `&'haystack str`, ensuring that each slice returned does not outlive the `haystack`.
// - This is enforced by the lifetime `'haystack` in the struct definition and `impl`.

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // Finds the next occurrence of the substring delimiter
        s.find(*self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // Finds the next occurrence of the character delimiter
        s.char_indices()
            .find(|&(_, c)| c == *self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

// Explanation:
// Implementing `Delimiter` for `&str` and `char` allows `StrSplit` to accept both types as delimiters.
// This showcases Rust's trait system and how it can be used to create flexible and reusable code.

pub fn until_char(s: &str, c: char) -> &str {
    // Returns the substring until the first occurrence of the character `c`.
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

// Explanation of `str` vs `[char]`:
// - `str`: An unsized string slice representing a sequence of UTF-8 bytes. Usually accessed via `&str`.
// - `[char]`: A slice of Unicode scalar values (`char`), each 4 bytes. Accessed via `&[char]`.
// - `str` is more memory-efficient than `[char]` because it stores text as UTF-8 bytes.

// Explanation of `&str` vs `&[char]`:
// - `&str`: A reference to a string slice (`str`). Commonly used for string data.
// - `&[char]`: A reference to a slice of `char`. Useful for manipulating individual characters.

// Explanation of `String` vs `Vec<char>`:
// - `String`: A growable, heap-allocated string type storing UTF-8 encoded text.
// - `Vec<char>`: A growable, heap-allocated vector of `char` values. Less memory-efficient.

// Explanation of conversions:
// - **String to `&str` (cheap -- AsRef)**: Taking a `&str` from a `String` is cheap; it borrows a slice.
//   ```rust
//   let string = String::from("Hello, Rust!");
//   let string_slice: &str = &string; // Borrowing as &str, no data copied.
//   ```
// - **`&str` to `String` (expensive -- memcpy)**: Creating a `String` from a `&str` involves allocating memory and copying data.
//   ```rust
//   let string_slice: &str = "Hello, Rust!";
//   let string = string_slice.to_string(); // Creates a new String by copying data.
//   ```
