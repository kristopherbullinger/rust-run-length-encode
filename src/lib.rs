
use std::iter::Iterator;
use std::iter::{Fuse, DoubleEndedIterator};
pub struct RunLengthEncode<I: Iterator<Item = T>, T: Eq> {
    iter: Fuse<I>,
    count: usize,
    current_front: Option<T>,
    current_back: Option<T>,
}

impl <I: Iterator<Item = T>, T: Eq> RunLengthEncode<I,T> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.fuse(),
            count: 0,
            current_front: None,
            current_back: None,
        }
    }
}

pub trait IterExt: Iterator {
    fn run_length_encode(self) -> RunLengthEncode<Self, <Self as Iterator>::Item>
        where
            Self: Iterator + Sized,
            <Self as Iterator>::Item: Eq,
    {
        RunLengthEncode::new(self)
    }
}

impl<T> IterExt for T where T: Iterator + ?Sized { }

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
                }
                None => match self.current_front.take() {
                    Some(front_item) => match self.current_back.take() {
                        Some(back_item) if front_item == back_item => return Some((self.count + 1, front_item)),
                        Some(back_item) => {
                            self.current_back = Some(back_item);
                            return Some((self.count, front_item));
                        }
                        None => return Some((self.count, front_item)),
                    }
                    None => return self.current_back.take().map(|item| (1, item))
                }
            }
        }
    }
}

impl <I, T> DoubleEndedIterator for RunLengthEncode<I, T>
    where
        I: Iterator<Item = T> + DoubleEndedIterator,
        T: Eq
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
                }
                None => match self.current_back.take() {
                    Some(back_item) => match self.current_front.take() {
                        Some(front_item) if front_item == back_item => return Some((self.count + 1, back_item)),
                        Some(front_item) => {
                            self.current_front = Some(front_item);
                            return Some((self.count, back_item));
                        }
                        None => return Some((self.count, back_item)),
                    }
                    None => return self.current_front.take().map(|item| (1, item))
                }
            }
        }
    }   
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_none_on_empty_source() {
        let mut rle = "".chars().run_length_encode();
        assert!(rle.next().is_none());
    }

    #[test]
    fn counts_chars_in_sorted_str() {
        let observed = "122333444455555".chars().run_length_encode().collect::<Vec<_>>();
        let expected = vec![(1, '1'), (2, '2'), (3, '3'), (4, '4'), (5, '5')];
        assert_eq!(observed, expected);
    }

    #[test]
    fn counts_chars_in_unsorted_str() {
        let observed = "501hexdead".chars().run_length_encode().collect::<Vec<_>>();
        let expected = vec![
            (1, '5'),
            (1, '0'),
            (1, '1'),
            (1, 'h'),
            (1, 'e'),
            (1, 'x'),
            (1, 'd'),
            (1, 'e'),
            (1, 'a'),
            (1, 'd'),
        ];
        assert_eq!(observed, expected);
    }

    #[test]
    fn extra_calls_continue_to_yield_none() {
        let mut observed = "5".chars().run_length_encode();
        for i in 0..100 {
            if i == 0 {
                assert!(observed.next().is_some());
            } else {
                assert!(observed.next().is_none());
            }
        }
    }

    #[test]
    fn can_encode_backwards() {
        let observed = "122333444455555".chars().rev().run_length_encode().collect::<Vec<_>>();
        let mut expected = vec![(1, '1'), (2, '2'), (3, '3'), (4, '4'), (5, '5')];
        expected.reverse();
        assert_eq!(observed, expected);
    }

    #[test]
    fn can_encode_forwards_and_backwards_alternating_starting_forward() {
        let mut rle = "122333444455555".chars().run_length_encode();
        let mut forward = true;
        let mut observed = Vec::new();
        loop {
            let next = if forward {
                rle.next()
            } else {
                rle.next_back()
            };
            forward = !forward;
            match next {
                None => break,
                Some(x) => observed.push(x),
            };
        }
        let mut expected = vec![
            (1, '1'),
            (5, '5'),
            (2, '2'),
            (4, '4'),
            (3, '3'),
        ];
        assert_eq!(observed, expected);
    }

    #[test]
    fn can_encode_forwards_and_backwards_alternating_starting_backward() {
        let mut rle = "122333444455555".chars().run_length_encode();
        let mut forward = false;
        let mut observed = Vec::new();
        loop {
            let next = if forward {
                rle.next()
            } else {
                rle.next_back()
            };
            forward = !forward;
            match next {
                None => break,
                Some(x) => observed.push(x),
            };
        }
        let mut expected = vec![
            (5, '5'),
            (1, '1'),
            (4, '4'),
            (2, '2'),
            (3, '3'),
        ];
        assert_eq!(observed, expected);
    }
}
