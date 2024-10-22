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

/// An extension trait for iterators that provides the `our_flatten` method.
pub trait IteratorExt: Iterator {
    /// Flattens an iterator of iterables into a single iterator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use your_crate_name::IteratorExt; // Replace `your_crate_name` with your crate's name
    ///
    /// let nested = vec![vec![1, 2], vec![3, 4]];
    /// let flat_iter = nested.into_iter().our_flatten();
    ///
    /// let collected: Vec<_> = flat_iter.collect();
    /// assert_eq!(collected, vec![1, 2, 3, 4]);
    /// ```
    fn our_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

/// Creates a `Flatten` iterator from any iterable of iterables.
///
/// # Example
///
/// ```rust
/// let nested = vec![vec![1, 2], vec![3, 4]];
/// let flat_iter = flatten(nested);
///
/// let collected: Vec<_> = flat_iter.collect();
/// assert_eq!(collected, vec![1, 2, 3, 4]);
/// ```
pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

/// An iterator that flattens an iterator of iterators into a single iterator.
///
/// This struct is created by the [`flatten`] function or the [`our_flatten`] method on [`IteratorExt`].
///
/// [`flatten`]: fn.flatten.html
/// [`our_flatten`]: trait.IteratorExt.html#method.our_flatten
pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_iter {
                if let Some(item) = front_iter.next() {
                    return Some(item);
                }
                self.front_iter = None;
            }

            if let Some(next_inner) = self.outer.next() {
                self.front_iter = Some(next_inner.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(item) = back_iter.next_back() {
                    return Some(item);
                }
                self.back_iter = None;
            }

            if let Some(next_back_inner) = self.outer.next_back() {
                self.back_iter = Some(next_back_inner.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        // Since `empty` returns an empty iterator, we should explicitly declare the type.
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![0, 1]].into_iter().our_flatten().count(), 2);
    }
}
