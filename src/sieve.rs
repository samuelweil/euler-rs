pub fn prime_sieve(max: u64) -> Vec<u64> {
    let mut sieve = vec![false; max as usize];
    let mut primes: Vec<usize> = vec![2];
    mark_multiples(&mut sieve, 2);

    while let Some(p) = next_prime(&sieve, *primes.last().unwrap()) {
        primes.push(p);
        mark_multiples(&mut sieve, p);
    }

    primes.into_iter().map(|r| r as u64).collect()
}

fn next_prime(sieve: &Vec<bool>, start: usize) -> Option<usize> {
    for index in start..sieve.len() {
        if !sieve[index] {
            return Some(index + 1)
        }
    }

    None
}

fn mark_multiples(sieve: &mut Vec<bool>, value: usize) {

    let mut multiplier = 1;

    let array_len = sieve.len();

    loop {
        let index = multiplier * value;
        if index > array_len {
            return
        }

        mark(sieve, index);
        multiplier = multiplier + 1;
    }
}

fn mark(sieve: &mut Vec<bool>, index: usize) {
    sieve[index - 1] = true;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_next_prime() {
        let sieve = vec![true, true, true, false, true];
        let p = next_prime(&sieve, 1);
        assert_eq!(p, Some(4));
    }

    #[test]
    fn test_marking() {
        let mut sieve = vec![false; 10];
        mark_multiples(&mut sieve, 3);

        assert!(!sieve[0]);
        assert!(!sieve[1]);
        assert!(sieve[2]);
        assert!(!sieve[3]);
        assert!(!sieve[4]);
        assert!(sieve[5]);
        assert!(!sieve[6]);
        assert!(!sieve[7]);
        assert!(sieve[8]);
        assert!(!sieve[9]);
    }

    #[test]
    fn test_primes() {
        let primes: Vec<u64> = prime_sieve(10);
        assert_eq!(primes.len(), 4);
        assert_eq!(primes[0], 2);
        assert_eq!(primes[1], 3);
        assert_eq!(primes[2], 5);
        assert_eq!(primes[3], 7);
    }
}
