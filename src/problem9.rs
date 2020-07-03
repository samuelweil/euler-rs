pub fn problem9() -> i32 {

    for a in 1..1000 {
        for b in 1..1000 {
            let c = 1000 - a - b;
            if check_sum(a,b,c) && check_pythogareas(a,b,c) {
                return a * b * c
            }
        }
    }

    0
}

fn check_sum(a: i32, b: i32, c: i32) -> bool {
    a + b + c == 1000
}

fn check_pythogareas(a: i32, b: i32, c: i32) -> bool {
    a * a + b * b == c * c
}
