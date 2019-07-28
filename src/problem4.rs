pub fn problem4() -> i64 {
    use std::cmp::max;

    let mut max_p = 0;
    let range = || (100..1000).rev();

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

fn is_palindrome(n: i64) -> bool {
    let as_str = format!("{}", n);

    for (c1, c2) in as_str.chars().zip(as_str.chars().rev()) {
        if c1 != c2 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn palindrome() {
        assert!(is_palindrome(101));
        assert!(is_palindrome(123321));
        assert!(is_palindrome(1231321));
        assert!(!is_palindrome(1234));
    }
}
