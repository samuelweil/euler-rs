use crate::sieve::prime_sieve;

pub fn problem10() -> u64 {
    sum_of_primes(2000000)
}

fn sum_of_primes(max: u64) -> u64 {
    let mut sum: u64 = 0;

    let primes = prime_sieve(max);

    for prime in primes {
        sum += prime;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_sum() {
        assert_eq!(sum_of_primes(10), 17);

        assert_eq!(sum_of_primes(20), 77);
    }
}
