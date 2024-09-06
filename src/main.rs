use std::env;

use board::Board;

mod board;
mod input;
mod solver;
mod solvers;

fn main() {
    let args: Vec<String> = env::args().collect();
    let testcase = input::load_json(&args[1]);

    let board = Board::new(testcase.board);
    eprintln!("{:?}", board);

    let ans = solver::solve(&board);
    println!("{}", ans);
}
