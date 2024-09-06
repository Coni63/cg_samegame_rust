use crate::board::Board;
use crate::solvers::bfs::bfs_solve;
use crate::solvers::dfs::dfs_solve;

pub fn solve(initial_state: &Board) -> String {
    dfs_solve(initial_state)
}
