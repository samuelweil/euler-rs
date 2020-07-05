pub fn problem12() -> u64 {
    for n in triangle_numbers() {
        if find_divisors(n).len() > 500 {
            return n;
        }
    }

    panic!("Failed to find a value");
}

struct TriangleNumberIterator {
    value: u64,
    i: u64,
}

impl Iterator for TriangleNumberIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.value);

        self.i = self.i + 1;
        self.value = self.value + self.i;

        result
    }
}

fn triangle_numbers() -> TriangleNumberIterator {
    TriangleNumberIterator { value: 1, i: 1 }
}

fn integer_sqrt(n: u64) -> u64 {
    let sq_root = (n as f64).sqrt();
    let floored = sq_root.floor();
    floored as u64
}

fn find_divisors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    for d in 1..=integer_sqrt(n) {
        if n % d == 0 {
            result.push(d);
            let other = n / d;
            if other != d {
                result.push(n / d);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_number_iter() {
        let numbers: Vec<u64> = triangle_numbers().take(7).collect();
        assert_eq!(numbers, vec![1, 3, 6, 10, 15, 21, 28]);
    }

    #[test]
    fn test_find_divisors() {
        assert_eq!(find_divisors(12), vec![1, 12, 2, 6, 3, 4]);
        assert_eq!(find_divisors(100), vec![1, 100, 2, 50, 4, 25, 5, 20, 10])
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(integer_sqrt(4), 2);
        assert_eq!(integer_sqrt(16), 4);
        assert_eq!(integer_sqrt(49), 7);
        assert_eq!(integer_sqrt(50), 7);
        assert_eq!(integer_sqrt(63), 7);
        assert_eq!(integer_sqrt(64), 8);
    }
}
