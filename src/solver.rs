use crate::board::Board;
use crate::solvers::mcrws::_solve;

pub fn solve(initial_state: &Board) -> (String, u32) {
    _solve(initial_state)
}
