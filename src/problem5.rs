use std::collections::BTreeMap;

use crate::factor::factor;
use crate::tools::merge_max;

pub fn problem5() -> u64 {
    let v: Vec<u64> = (1..=20).collect();
    lowest_multiple(v)
}

fn lowest_multiple(nums: Vec<u64>) -> u64 {
    let mut fcs: BTreeMap<u64, u64> = BTreeMap::new();

    for i in nums {
        fcs = merge_max(&fcs, &factor(i).factors());
    }

    let mut prod = 1;
    for (k, v) in fcs {
        prod *= k.pow(v as u32);
    }
    prod
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lowest_multiple() {
        use std::iter::FromIterator;

        assert_eq!(lowest_multiple(vec![6, 8]), 24);
        assert_eq!(lowest_multiple(vec![3, 7]), 21);
        assert_eq!(lowest_multiple(Vec::from_iter(1u64..=10u64)), 2520);
    }
}
