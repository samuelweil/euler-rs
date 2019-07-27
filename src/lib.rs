use std::collections::HashSet;

extern crate num;   

pub fn sum<T, E>(iter: T) -> E where
T: Iterator<Item=E>,
E: std::ops::Add<Output=E> + num::Zero {
    iter.fold(E::zero(), |n1, n2| n1 + n2)
}

pub fn natural_numbers(n: i64) -> i64 {
    let mut nat_nums = HashSet::new();

    for i in (0..n).step_by(3) {
        nat_nums.insert(i);
    }

    for i in (0..n).step_by(5) {
        nat_nums.insert(i);
    }

    sum(nat_nums.into_iter())
}

pub struct FibonnaciIterator {
    n_1: i64,
    n_2: i64,
}

impl Iterator for FibonnaciIterator {
    fn next(&mut self) -> Option<i64> {
        let next = self.n_1 + self.n_2;
        self.n_1 = self.n_2;
        self.n_2 = next;

        Option::Some(self.n_1)
    }

    type Item = i64;
}

impl FibonnaciIterator {
    pub fn new() -> FibonnaciIterator {
        FibonnaciIterator { n_1: 0, n_2: 1 }
    }
}

pub fn is_palindrome(n: i64) -> bool {
    let as_str = format!("{}", n);

    for (c1, c2) in as_str.chars().zip(as_str.chars().rev()) {
        if c1 != c2 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_1() {
        assert_eq!(natural_numbers(10), 23);
    }

    #[test]
    fn fibonnaci_series() {
        let fib_series: Vec<i64> = FibonnaciIterator::new().take(5).collect();
        assert_eq!(fib_series, vec![1, 1, 2, 3, 5]);
    }

    #[test]
    fn palindrome() {
        assert!(is_palindrome(101));
        assert!(is_palindrome(123321));
        assert!(is_palindrome(1231321));
        assert!(!is_palindrome(1234));
    }
}