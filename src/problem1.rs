use std::collections::HashSet;

pub fn problem1() -> i64 {
    natural_numbers(1000, vec![3, 5].into_iter())
}

fn natural_numbers<T>(n_max: i64, multiples: T) -> i64
where
    T: Iterator<Item = i64>,
{
    let mut nat_nums = HashSet::new();

    for m in multiples {
        for i in (0..n_max).step_by(m as usize) {
            nat_nums.insert(i);
        }
    }

    nat_nums.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_natural_numbers() {
        let mults = vec![3, 5];
        assert_eq!(natural_numbers(10, mults.into_iter()), 23);
    }
}
