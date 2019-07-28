use crate::factor::factor;

pub fn problem3() -> i64 {
    let val: i64 = 600851475143;

    *factor(val).factor_list().iter().max().unwrap()
}
