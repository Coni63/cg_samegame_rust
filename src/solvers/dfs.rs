use crate::board::Board;
use itertools;
use std::{
    collections::{HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
};

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

pub fn dfs_solve(initial_state: &Board) -> String {
    let mut best_solution: Solution = Solution::new(initial_state.clone());
    let mut visited: HashSet<u64> = HashSet::new();
    let mut actions: VecDeque<String> = VecDeque::new();

    inner_dfs(
        &mut visited,
        &mut actions,
        &mut best_solution,
        initial_state,
    );

    eprintln!("{}", best_solution.board.get_score());
    itertools::join(best_solution.actions, ";")
}

fn inner_dfs(
    visited: &mut HashSet<u64>,
    actions: &mut VecDeque<String>,
    best_solution: &mut Solution,
    initial_state: &Board,
) {
    let all_regions = initial_state.compute_all_regions();

    if initial_state.is_over() && initial_state.get_score() > best_solution.board.get_score() {
        best_solution.board = initial_state.clone();
        best_solution.actions = actions.clone();

        eprintln!("New best score: {}", best_solution.board.get_score());
    }

    for region in all_regions {
        let mut copy = initial_state.clone();

        copy.play_region(&region);

        let mut hasher = DefaultHasher::new();
        copy.hash(&mut hasher);
        let hash_value = hasher.finish();
        if visited.contains(&hash_value) {
            continue;
        }
        visited.insert(hash_value);

        let idx = region.first().unwrap();
        let (x, y) = Board::to_coordinates(idx);
        actions.push_back(format!("{} {}", x, y));

        inner_dfs(visited, actions, best_solution, &copy);

        actions.pop_back();
    }
}
