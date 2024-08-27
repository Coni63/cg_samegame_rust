use std::env;

use board::Board;

mod board;
mod input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let testcase = input::load_json(&args[1]);

    eprintln!("{:?}", testcase);

    let mut board = Board::new(testcase.board);
    eprintln!("{:?}", board);

    board.play(1, 0);
    eprintln!("{:?}", board);
}
