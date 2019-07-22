mod primes; 

use primes::PrimeIterator;

use std::collections::HashMap;
struct Factors {
    factor_map: HashMap<i64, i64>
}

impl Factors {
    fn factor_list(&self) -> Vec<i64> {
        let mut out = Vec::new();

        for i in self.factor_map.keys() {
            out.push(*i);
        }

        out
    }
}



pub fn factor(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
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
        assert_eq!(factor(6), vec![2, 3]);
    }
}