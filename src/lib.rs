#[cfg(test)]
mod tests;

use std::iter::Iterator;
use std::iter::{DoubleEndedIterator, Fuse};

/// An iterator that yields a [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding)
/// of the underlying iterator. This struct is created by the [`IteratorExt::run_length_encode`] method.
/// Check its documentation for more information.
#[derive(Debug, Clone)]
pub struct RunLengthEncode<I: Iterator<Item = T>, T: Eq> {
    iter: Fuse<I>,
    count: usize,
    current_front: Option<T>,
    current_back: Option<T>,
}

impl<I: Iterator<Item = T>, T: Eq> RunLengthEncode<I, T> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.fuse(),
            count: 0,
            current_front: None,
            current_back: None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.iter.size_hint() {
            (_, Some(n)) => (0, Some(n)),
            (_, None) => (0, None),
        }
    }
}

pub trait IteratorExt: Iterator {
    /// An iterator that yields a [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding)
    /// of the underlying iterator. That is, it yields items of type `(usize, T)`, representing
    /// the number of times in a row that an item which compared equal to T was yielded from the source.
    ///
    /// The specific item yielded when calling `next` is the first instance of T seen in a sequence of items that
    /// compare equal according to `T::Eq`. The reverse is true when calling `next_back`. This is important when
    /// `T::Eq` is derived by hand.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use run_length_encode::IteratorExt;
    /// let chars_rle = "122333255555".chars().run_length_encode().collect::<Vec<_>>();
    /// let expected = vec![(1, '1'), (2, '2'), (3, '3'), (1, '2'), (5, '5')];
    /// assert_eq!(expected, chars_rle);
    /// ```
    ///
    /// Custom Eq types:
    ///
    /// ```
    /// # use run_length_encode::IteratorExt;
    /// #[derive(Eq)]
    /// struct Item {
    ///     a: usize,
    ///     b: &'static str,
    /// }
    /// impl PartialEq for Item {
    ///     fn eq(&self, other: &Item) -> bool {
    ///         //field `b` is ignored
    ///         self.a.eq(&other.a)
    ///     }
    /// }
    /// let mut items_rle = vec![
    ///         Item{ a: 0, b: "me" },
    ///         Item{ a: 0, b: "not me" },
    ///         Item{ a: 0, b: "Also not me" },
    ///     ]
    ///     .into_iter()
    ///     .run_length_encode();
    /// assert_eq!(items_rle.next().map(|(c, item)| (c, item.b)), Some((3, "me")));
    /// ```
    fn run_length_encode(self) -> RunLengthEncode<Self, <Self as Iterator>::Item>
    where
        Self: Iterator + Sized,
        <Self as Iterator>::Item: Eq,
    {
        RunLengthEncode::new(self)
    }
}

impl<T> IteratorExt for T where T: Iterator + ?Sized {}

impl<I: Iterator<Item = T>, T: Eq> Iterator for RunLengthEncode<I, T> {
    type Item = (usize, T);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                x @ Some(_) if x == self.current_front => self.count += 1,
                Some(item) => match self.current_front.take() {
                    Some(current) => {
                        let out = (self.count, current);
                        self.current_front = Some(item);
                        self.count = 1;
                        return Some(out);
                    }
                    None => {
                        self.current_front = Some(item);
                        self.count = 1;
                    }
                },
                None => match self.current_front.take() {
                    Some(front_item) => match self.current_back.take() {
                        Some(back_item) if front_item == back_item => {
                            return Some((self.count + 1, front_item))
                        }
                        Some(back_item) => {
                            self.current_back = Some(back_item);
                            return Some((self.count, front_item));
                        }
                        None => return Some((self.count, front_item)),
                    },
                    None => return self.current_back.take().map(|item| (1, item)),
                },
            }
        }
    }
}

impl<I, T> DoubleEndedIterator for RunLengthEncode<I, T>
where
    I: Iterator<Item = T> + DoubleEndedIterator,
    T: Eq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next_back() {
                x @ Some(_) if x == self.current_back => self.count += 1,
                Some(item) => match self.current_back.take() {
                    Some(current) => {
                        let out = (self.count, current);
                        self.current_back = Some(item);
                        self.count = 1;
                        return Some(out);
                    }
                    None => {
                        self.current_back = Some(item);
                        self.count = 1;
                    }
                },
                None => match self.current_back.take() {
                    Some(back_item) => match self.current_front.take() {
                        Some(front_item) if front_item == back_item => {
                            return Some((self.count + 1, back_item))
                        }
                        Some(front_item) => {
                            self.current_front = Some(front_item);
                            return Some((self.count, back_item));
                        }
                        None => return Some((self.count, back_item)),
                    },
                    None => return self.current_front.take().map(|item| (1, item)),
                },
            }
        }
    }
}
