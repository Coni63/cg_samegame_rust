// https://liacs.leidenuniv.nl/~takesfw/pdf/samegame.pdf

use crate::board::Board;
use crate::solvers::solution::Solution;

pub fn _solve(initial_state: &Board) -> String {
    let mut best_solution: Solution = Solution::new(initial_state.clone());

    itertools::join(best_solution.actions, ";")
}
