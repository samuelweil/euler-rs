use crate::factor::primes::PrimeIterator;

pub fn problem7() -> i64 {

    let n = 10001;
    let primes: Vec<i64> = PrimeIterator::new().take(n).collect();

    primes[n-1]

}