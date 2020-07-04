pub fn problem6() -> u64 {
    let s: u64 = 1;
    let e: u64 = 100;

    square_of_sum(s..=e) - sum_of_squares(s..=e)
}

fn sum_of_squares<T>(itr: T) -> u64
where
    T: Iterator<Item = u64>,
{
    itr.map(|x| x.pow(2)).sum()
}

fn square_of_sum<T>(itr: T) -> u64
where
    T: Iterator<Item = u64>,
{
    let x: u64 = itr.sum();
    x.pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(vec![2, 3].into_iter()), 13);
        assert_eq!(sum_of_squares(1u64..=10u64), 385);
    }

    #[test]
    fn test_square_of_sum() {
        assert_eq!(square_of_sum(vec![2, 3].into_iter()), 25);
        assert_eq!(square_of_sum(1u64..=10u64), 3025);
    }

    #[test]
    fn test_problem_6() {
        let start: u64 = 1;
        let end: u64 = 10;
        assert_eq!(
            square_of_sum(start..=end) - sum_of_squares(start..=end),
            2640
        );
    }
}
