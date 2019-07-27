mod factor;
mod tools;

use factor::factor;
use tools::merge_max;
use std::collections::HashMap;

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

    *factor(val).factor_list().iter().max().unwrap()
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
    println!("problem 5: {}", problem5());
}

use std::collections::BTreeMap;

fn lowest_multiple(nums: Vec<i64>) -> i64 {

    let mut fcs: BTreeMap<i64, i64> = BTreeMap::new();

    for i in nums {
        fcs = merge_max(&fcs, &factor(i).factors());
    }

    let mut prod = 1;
    
    for (k, v) in fcs {
        prod *= k.pow(v as u32);
    }
    
    prod
}

fn problem5() -> i64 {
    let v: Vec<i64> = (1..=10).collect();
    lowest_multiple(v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lowest_multiple() {
        assert_eq!(lowest_multiple(vec![6,8]), 24);
        assert_eq!(lowest_multiple(vec![3, 7]), 21);
    }
}