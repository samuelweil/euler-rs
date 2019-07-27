mod primes; 

use primes::PrimeIterator;

use std::collections::BTreeMap;

pub struct Factors {
    // Use a BTreeMap instead of a HashMap to ensure
    // that the prime numbers are returned in order
    factor_map: BTreeMap<i64, i64>
}

impl Factors {
    pub fn factor_list(&self) -> Vec<i64> {
        let mut out = Vec::new();

        for i in self.factor_map.keys() {
            out.push(*i);
        }

        out
    }

    pub fn factors(&self) -> BTreeMap<i64, i64> {
        self.factor_map.clone()
    }

    pub fn new() -> Factors {
        Factors{
            factor_map: BTreeMap::new()
        }
    }

    pub fn push(&mut self, n: i64) {
        let count = self.factor_map.entry(n).or_insert(0);
        *count += 1;
    }
}

pub fn factor(n: i64) -> Factors {
    let mut factors = Factors::new();
    let mut prime_list = PrimeIterator::new();

    let mut val = n;
    while val > 1 {
        let f = next_factor(val, &mut prime_list);
        factors.push(f);
        val = val / f;

        prime_list = prime_list.new_iter();
    }

    factors
}

fn next_factor(n: i64, primes: &mut PrimeIterator) -> i64 {
    for prime in primes {
        if n % prime == 0 {
            return prime
        }

        if prime > n {
            return n
        }
    }
    
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn factor_test() {
        assert_eq!(factor(6).factor_list(), vec![2, 3]);
    }
}