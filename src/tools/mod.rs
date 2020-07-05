use std::collections::BTreeMap;
use std::cmp::max;

pub mod future;

pub fn merge_max<'a>(
    m_in: &'a BTreeMap<u64, u64>,
    m2: &'a BTreeMap<u64, u64>,
) -> BTreeMap<u64, u64> {
    let mut m = m_in.clone();

    for (k, v) in m2 {
        let max_val = match m.get(k) {
            Some(i) => max(*i, *v),
            None => *v,
        };

        let e = m.entry(*k).or_insert(0);
        *e = max_val;
    }

    m
}