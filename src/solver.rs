use crate::board::Board;
// use crate::solvers::bfs::_solve;
// use crate::solvers::dfs::_solve;
use crate::solvers::mcrws::_solve;

pub fn solve(initial_state: &Board) -> String {
    _solve(initial_state)
}
