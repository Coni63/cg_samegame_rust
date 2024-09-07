use std::collections::VecDeque;

use crate::board::Board;

pub struct Solution {
    pub actions: VecDeque<String>,
    pub board: Board,
}

impl Solution {
    pub fn new(board: Board) -> Solution {
        Solution {
            actions: VecDeque::new(),
            board,
        }
    }
}
