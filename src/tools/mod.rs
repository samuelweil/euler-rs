use std::iter::FromIterator;

fn map_from_arr<T, K, V>(arr: Vec<(K, V)>) -> T
where
T: IntoIterator<Item = (K, V)> + FromIterator<(K, V)>
{
    arr.into_iter().collect()
}

#[macro_export]
macro_rules! merge {
    ($x:expr) => ($x);

    ($x:expr, $( $y:expr ), *) => {
        merge!($x, $($y), *)
    }
}

pub fn merge<T, K, V>(m1: T, m2: T) -> T
where
    T: IntoIterator<Item = (K, V)> + FromIterator<(K, V)>,
{
    let out: T = m1.into_iter().chain(m2.into_iter()).collect();
    out
}

use std::collections::BTreeMap;
use std::cmp::max;

pub fn merge_max<'a>(m_in: &'a BTreeMap<i64, i64>, m2: &'a BTreeMap<i64, i64>) -> BTreeMap<i64, i64> {
    let mut m = m_in.clone();

    for (k, v) in m2 {
        let max_val = match m.get(k) {
            Some(i) => max(*i, *v),
            None => *v
        };

        let e = m.entry(*k).or_insert(0);
        *e = max_val;
    }

    m
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_merge() {
        let k1: HashMap<i32, i32> = map_from_arr(vec![(1, 2), (3, 4)]);
        let k2: HashMap<i32, i32> = map_from_arr(vec![(5, 6), (7, 8)]);

        let m = merge(k1, k2);
        assert_eq!(m[&1], 2);
        assert_eq!(m[&3], 4);
        assert_eq!(m[&5], 6);
        assert_eq!(m[&7], 8);
    }

    fn test_multi_merge() {

    }
}
