mod factor;
mod problem1;
mod problem10;
mod problem11;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;
mod sieve;
mod tools;

macro_rules! prob {
    ($problem:ident) => {
        $problem::$problem
    };
}

const PROBLEMS: [fn() -> u64; 11]  = [
    prob!(problem1),
    prob!(problem2),
    prob!(problem3),
    prob!(problem4),
    prob!(problem5),
    prob!(problem6),
    prob!(problem7),
    prob!(problem8),
    prob!(problem9),
    prob!(problem10),
    prob!(problem11),
];

fn solve(n: usize) {
    println!("Solving problem {}: {}", n, PROBLEMS[n - 1]());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let n_problem = args[1].parse::<usize>().unwrap();
        solve(n_problem);
    } else {
        for i in 0..PROBLEMS.len() {
            solve(i + 1);
        }
    }
}
