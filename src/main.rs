use std::env;

use board::Board;

mod board;
mod input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let testcase = input::load_json(&args[1]);

    eprintln!("{:?}", testcase);

    let board = Board::new(testcase.board);
    eprintln!("{:?}", board);

    eprintln!("{:?}", board.compute_region(0, 0));
}
