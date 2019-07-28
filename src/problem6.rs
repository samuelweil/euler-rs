pub fn problem6() -> i64 {
    let s: i64 = 1;
    let e: i64 = 100;

    square_of_sum(s..=e) - sum_of_squares(s..=e)
}

fn sum_of_squares<T>(itr: T) -> i64
where
    T: Iterator<Item = i64>,
{
    itr.map(|x| x.pow(2)).sum()
}

fn square_of_sum<T>(itr: T) -> i64
where
    T: Iterator<Item = i64>,
{
    let x: i64 = itr.sum();
    x.pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(vec![2, 3].into_iter()), 13);
        assert_eq!(sum_of_squares(1i64..=10i64), 385);
    }

    #[test]
    fn test_square_of_sum() {
        assert_eq!(square_of_sum(vec![2, 3].into_iter()), 25);
        assert_eq!(square_of_sum(1i64..=10i64), 3025);
    }

    #[test]
    fn test_problem_6() {
        let start: i64 = 1;
        let end: i64 = 10;
        assert_eq!(
            square_of_sum(start..=end) - sum_of_squares(start..=end),
            2640
        );
    }
}
