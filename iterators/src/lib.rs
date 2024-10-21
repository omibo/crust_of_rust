/// # Iteration Example
///
/// In Rust, the `for` loop is syntactic sugar for iterating over items using an iterator.
/// Here’s an example of a `for` loop:
///
/// ```rust
/// for x in vec!["a", "b", "c"] {
///     // do something with x
/// }
/// ```
///
/// This is actually syntactic sugar for the following code:
///
/// ```rust
/// let mut iter = vec!["a", "b", "c"].into_iter();
/// while let Some(e) = iter.next() {
///     // do something with e
/// }
/// ```
///
/// The `for` loop implicitly calls the `.into_iter()` method to create an iterator,
/// and then uses the `while let Some` pattern to iterate over the elements. The iterator's
/// `.next()` method is called in each iteration to get the next element, and when the
/// iterator is exhausted (returns `None`), the loop ends.
///
///

/// # Iterating Over a Vector in Rust
///
/// There are multiple ways to iterate over a `Vec` in Rust, each with different ownership semantics.
/// Let’s consider a vector `vs` and three ways of iterating over it.
///
/// ```rust
/// let vs = vec![1, 2, 3];
/// ```
///
/// ## 1. Consuming the Vector (Ownership Transfer)
///
/// When you use `for v in vs`, the vector `vs` is *consumed*:
///
/// ```rust
/// for v in vs {
///     // consumes `vs`, ownership of each element is moved into `v`
/// }
/// ```
///
/// In this case, `vs` is no longer usable after the loop because ownership of the vector is moved
/// into the loop. This uses the `into_iter` method implicitly, which transfers ownership of each element.
///
/// ## 2. Borrowing the Vector’s Elements (Immutable References)
///
/// You can borrow each element of the vector without consuming it using `vs.iter()`:
///
/// ```rust
/// for v in vs.iter() {
///     // borrows `vs`, `v` is an immutable reference (&i32)
/// }
/// ```
///
/// Here, `vs` is not consumed, and the elements are accessed as immutable references (`&i32`).
/// After the loop, `vs` is still available for use.
///
/// ## 3. Borrowing the Entire Vector (Equivalent to `iter()`)
///
/// Alternatively, you can iterate over references to the elements of the vector using `&vs`:
///
/// ```rust
/// for v in &vs {
///     // equivalent to `vs.iter()`, `v` is an immutable reference (&i32)
/// }
/// ```
///
/// This syntax is a shorthand for calling `vs.iter()`, and it behaves the same by borrowing the
/// vector and giving you immutable references to its elements.
///
/// ## Source
///
/// For a detailed explanation of the differences between `iter`, `into_iter`, and `&` on vectors,
/// you can refer to this helpful discussion:
/// [StackOverflow: What is the difference between iter and into_iter?](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter)

fn hello() {}
