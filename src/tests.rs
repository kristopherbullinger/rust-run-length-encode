#[cfg(test)]
#[derive(Debug)]
struct Item {
    a: usize,
    b: usize,
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(&other.a)
    }
}
impl Eq for Item {}

use super::*;
#[test]
fn returns_none_on_empty_source() {
    let mut rle = "".chars().run_length_encode();
    assert!(rle.next().is_none());
}

#[test]
fn counts_chars_in_sorted_str() {
    let observed = "122333444455555"
        .chars()
        .run_length_encode()
        .collect::<Vec<_>>();
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
    let observed = "122333444455555"
        .chars()
        .rev()
        .run_length_encode()
        .collect::<Vec<_>>();
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
        let next = if forward { rle.next() } else { rle.next_back() };
        forward = !forward;
        match next {
            None => break,
            Some(x) => observed.push(x),
        };
    }
    let expected = vec![(1, '1'), (5, '5'), (2, '2'), (4, '4'), (3, '3')];
    assert_eq!(observed, expected);
}

#[test]
fn can_encode_forwards_and_backwards_alternating_starting_backward() {
    let mut rle = "122333444455555".chars().run_length_encode();
    let mut forward = false;
    let mut observed = Vec::new();
    loop {
        let next = if forward { rle.next() } else { rle.next_back() };
        forward = !forward;
        match next {
            None => break,
            Some(x) => observed.push(x),
        };
    }
    let expected = vec![(5, '5'), (1, '1'), (4, '4'), (2, '2'), (3, '3')];
    assert_eq!(observed, expected);
}

#[test]
#[allow(non_snake_case)]
fn iterate_forwards_T_is_first_in_subsequence() {
    let mut items: Vec<Item> = Vec::with_capacity(100);
    for i in 0..100 {
        let item = Item {
            a: i / 10,
            b: i % 10,
        };
        items.push(item);
    }
    for (_, item) in items.into_iter().run_length_encode() {
        assert_eq!(item.b, 0);
    }
}

#[test]
#[allow(non_snake_case)]
fn iterate_backwards_T_is_last_in_subsequence() {
    let mut items: Vec<Item> = Vec::with_capacity(100);
    for i in 0..100 {
        let item = Item {
            a: i / 10,
            b: i % 10,
        };
        items.push(item);
    }
    for (_, item) in items.into_iter().run_length_encode().rev() {
        assert_eq!(item.b, 9);
    }
}

#[test]
#[allow(non_snake_case)]
fn iterate_alternating_forwards_backwards_T_alternates_first_last_in_subsequence() {
    let mut items: Vec<Item> = Vec::with_capacity(100);
    for i in 0..100 {
        let item = Item {
            a: i / 10,
            b: i % 10,
        };
        items.push(item);
    }
    let mut rle = items.into_iter().run_length_encode();
    let mut forward = true;
    loop {
        let item = if forward { rle.next() } else { rle.next_back() };
        match item {
            Some((_, item)) => {
                let expected = if forward { 0 } else { 9 };
                assert_eq!(item.b, expected);
            }
            None => break,
        };
        forward = !forward;
    }
}

#[test]
fn can_fold() {
    let mut rle = "122333444455555".chars().run_length_encode();
    let expected = 15;
    let observed = rle.fold(0, |acc, item| {
        acc + item.0
    });
    assert_eq!(expected, observed);
}

#[test]
fn longest_subsequence() {
    let mut rle = "1223336666666666444455555".chars().run_length_encode();
    let expected = Some((10, '6'));
    let observed = rle.max_by_key(|item| item.0);
    assert_eq!(expected, observed);
}
