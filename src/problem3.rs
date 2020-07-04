use crate::factor::factor;

pub fn problem3() -> u64 {
    let val: u64 = 600851475143;

    *factor(val).factor_list().iter().max().unwrap()
}
