use crate::board::Board;
use fxhash::{FxHashSet, FxHasher64};
use itertools;
use std::{
    collections::{HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
    time::Instant,
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
    let mut visited: HashSet<u64> = HashSet::default();
    let mut actions: VecDeque<String> = VecDeque::new();
    let mut count = 0;
    let timer = Instant::now();

    inner_dfs(
        &mut visited,
        &mut actions,
        &mut best_solution,
        initial_state,
        &mut count,
    );

    eprintln!(
        "Final best score: {} - {} plays in {:?}",
        best_solution.board.get_score(),
        count,
        timer.elapsed()
    );
    itertools::join(best_solution.actions, ";")
}

fn inner_dfs(
    visited: &mut HashSet<u64>,
    actions: &mut VecDeque<String>,
    best_solution: &mut Solution,
    initial_state: &Board,
    count: &mut i32,
) {
    let all_regions = initial_state.compute_all_regions();

    if initial_state.is_over() {
        if initial_state.get_score() > best_solution.board.get_score() {
            best_solution.board = initial_state.clone();
            best_solution.actions = actions.clone();

            eprintln!(
                "New best score: {} - {} plays",
                best_solution.board.get_score(),
                count
            );
        }
        *count += 1;
    }

    for region in all_regions {
        let mut copy = initial_state.clone();

        copy.play_region(&region);

        let mut hasher = DefaultHasher::default();
        copy.hash(&mut hasher);
        let hash_value = hasher.finish();
        if visited.contains(&hash_value) {
            continue;
        }
        visited.insert(hash_value);

        let idx = region.first().unwrap();
        let (x, y) = Board::to_coordinates(idx);
        actions.push_back(format!("{} {}", x, y));

        inner_dfs(visited, actions, best_solution, &copy, count);

        actions.pop_back();
    }
}
