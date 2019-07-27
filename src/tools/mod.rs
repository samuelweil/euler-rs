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
