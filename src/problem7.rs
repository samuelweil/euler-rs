use crate::factor::primes::PrimeIterator;

pub fn problem7() -> u64 {

    let n = 10001;
    let primes: Vec<u64> = PrimeIterator::new().take(n).collect();

    primes[n-1]

}