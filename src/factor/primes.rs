pub struct PrimeIterator {
    primes: Vec<i64>,
    n: usize,
}

impl PrimeIterator {
    pub fn new() -> PrimeIterator {
        PrimeIterator {
            primes: vec![2, 3, 5, 7],
            n: 0,
        }
    }

    pub fn new_iter(&self) -> PrimeIterator {
        PrimeIterator {
            primes: self.primes.clone(),
            n: 0,
        }
    }

    fn is_prime(&self, n: i64) -> bool {
        for prime in &self.primes {
            if n % prime == 0 {
                return false;
            }
        }

        true
    }

    fn next_prime(&self) -> i64 {
        let mut value = *self.primes.last().unwrap() + 2;
        while !self.is_prime(value) {
            value += 2;
        }

        value
    }
}

impl Iterator for PrimeIterator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let out: Option<i64>;

        if self.n < self.primes.len() {
            out = Some(self.primes[self.n]);
        } else {
            let next_prime = self.next_prime();
            self.primes.push(next_prime);
            out = Some(next_prime);
        }

        self.n += 1;
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primes() {
        let prime_gen = PrimeIterator::new();
        let first10: Vec<i64> = prime_gen.take(10).collect();

        assert_eq!(first10, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
}
