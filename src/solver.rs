use std::time::Instant;

use crate::board::Board;
use crate::solvers::beam::{beam, Params};
use crate::solvers::nested::nested;
use crate::solvers::fast::{actions_to_string, string_to_actions, verify, State};

pub struct Options {
    pub width: usize,
    pub threads: usize,
    pub seed: u64,
    pub group_weight: f32,
    pub single_penalty: f32,
    pub target_weight: f32,
    pub other_weight: f32,
    pub noise: f32,
    pub targets: Vec<u8>,
    /// playout width of the nested search, 0 = plain beam search only
    pub nested_width: usize,
    /// start from the best solution stored in the database
    pub from_db: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            width: 2000,
            threads: std::thread::available_parallelism().map_or(1, |n| n.get()),
            seed: 0,
            group_weight: 0.1,
            single_penalty: 20.0,
            target_weight: 1.0,
            other_weight: 1.0,
            noise: 0.0,
            targets: vec![1, 2, 3, 4, 5],
            nested_width: 0,
            from_db: false,
        }
    }
}

/// `known`: an already known solution (e.g. the best one of the database) the search starts from.
pub fn solve(initial_state: &Board, opts: &Options, known: Option<&str>) -> (String, u32) {
    let root = State::from_board(initial_state);
    let mut best: (Vec<u8>, u32) = (vec![], 0);
    if let Some(actions) = known {
        let moves = string_to_actions(actions);
        let score = verify(initial_state, &moves);
        eprintln!("known solution: {}", score);
        best = (moves, score);
    }
    for &target in opts.targets.iter() {
        let start = Instant::now();
        let p = Params {
            width: opts.width,
            target,
            group_weight: opts.group_weight,
            single_penalty: opts.single_penalty,
            target_weight: opts.target_weight,
            other_weight: opts.other_weight,
            noise: opts.noise,
            seed: opts.seed.wrapping_add(target as u64),
            threads: opts.threads,
        };
        let (mut moves, mut score) = beam(&root, &p);
        if opts.nested_width > 0 {
            eprintln!("target {} beam -> {}", target, score);
            let mut playout = p;
            playout.width = opts.nested_width;
            let init = if score >= best.1 { (moves, score) } else { best.clone() };
            (moves, score) = nested(&root, &playout, opts.threads, Some(init));
        }
        eprintln!(
            "target {} -> {} ({} moves, {:.1}s)",
            target,
            score,
            moves.len(),
            start.elapsed().as_secs_f32()
        );
        if score > best.1 || best.0.is_empty() {
            best = (moves, score);
        }
    }
    let checked = verify(initial_state, &best.0);
    assert_eq!(checked, best.1, "fast engine and reference engine disagree");
    (actions_to_string(&best.0), checked)
}
