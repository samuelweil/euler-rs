mod factor;

use factor::factor;

use euler::*;

fn problem1() -> i64 {
    natural_numbers(1000)
}

fn is_even(n: &i64) -> bool {
    n % 2 == 0
}

fn problem2() -> i64 {
    let nums = FibonnaciIterator::new()
        .take_while(|n| n < &4000000)
        .filter(|n| is_even(n));
    sum(nums)
}

fn problem3() -> i64 {
    let val: i64 = 600851475143;

    *factor(val).iter().max().unwrap()
}

fn problem4() -> i64 {
    use std::cmp::max;

    let mut max_p = 0;
    let range = || { (100..1000).rev() };

    for n1 in range() {
        for n2 in range() {
            let prod = n1 * n2;
            if is_palindrome(prod) {
                max_p = max(prod, max_p);
                break;
            }
        }
    }

    max_p
}

pub fn main() {
    println!("problem 1: {}", problem1());
    println!("problem 2: {}", problem2());
    println!("problem 3: {}", problem3());
    println!("problem 4: {}", problem4());
}
