mod grid;

use grid::GRID;
use std::cmp::max;

pub fn problem11() -> u64 {
    0
}

fn max_prod(slice: &[u32]) -> u64 { 
    let mut max_p = 0;
    for i in 0..slice.len() - 4 {
        max_p = max(max_p, prod(&slice[i..i+4]));
    }

    max_p
}

fn prod(slice: &[u32]) -> u64 {
    slice.iter().fold(1, |a, v| a * (*v as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prod() {
        assert_eq!(prod(&[1, 2, 3, 4]), 24);
    }
}
