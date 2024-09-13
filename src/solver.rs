use crate::board::Board;
use crate::solvers::tabucolor::_solve;

pub fn solve(initial_state: &Board) -> (String, u32) {
    _solve(initial_state)
}
