mod grid;

use crate::tools::future::{run_async, Future};
use grid::{Grid, GRID};
use std::cmp::max;

pub fn problem11() -> u64 {
    let row_max = run_async(|| max_of_arrs(GRID.rows()));
    let col_max = run_async(|| max_of_arrs(GRID.cols()));
    let diag_max = run_async(|| max_of_arrs(GRID.diags().filter(|r| r.len() > 3)));

    max(row_max.result(), max(col_max.result(), diag_max.result()))
}

fn max_of_arrs(i: impl Iterator<Item = Vec<u32>>) -> u64 {
    i.map(max_prod).fold(0, max)
}

fn max_prod(slice: Vec<u32>) -> u64 {
    let mut max_p = 0;
    for i in 0..slice.len() - 4 {
        max_p = max(max_p, prod(&slice[i..i + 4]));
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
