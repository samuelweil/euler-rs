pub fn problem2() -> u64 {
    FibonnaciIterator::new()
        .take_while(|n| n < &4000000)
        .filter(|n| is_even(n))
        .sum()
}

fn is_even(n: &u64) -> bool {
    n % 2 == 0
}

struct FibonnaciIterator {
    n_1: u64,
    n_2: u64,
}

impl Iterator for FibonnaciIterator {
    fn next(&mut self) -> Option<u64> {
        let next = self.n_1 + self.n_2;
        self.n_1 = self.n_2;
        self.n_2 = next;

        Option::Some(self.n_1)
    }

    type Item = u64;
}

impl FibonnaciIterator {
    pub fn new() -> FibonnaciIterator {
        FibonnaciIterator { n_1: 0, n_2: 1 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fibonnaci_series() {
        let fib_series: Vec<u64> = FibonnaciIterator::new().take(5).collect();
        assert_eq!(fib_series, vec![1, 1, 2, 3, 5]);
    }
}
