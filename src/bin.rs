mod factor;
mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod tools;

macro_rules! solve {
    ($problem:ident) => {
        println!("{}: {}", stringify!($problem), $problem::$problem());
    };
}

fn main() {
    solve!(problem1);
    solve!(problem2);
    solve!(problem3);
    solve!(problem4);
    solve!(problem5);
    solve!(problem6);
}
