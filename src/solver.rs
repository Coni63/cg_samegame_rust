use std::{
    collections::{HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::board::Board;

pub fn solve(initial_state: &Board) -> String {
    let mut visited: HashSet<u64> = HashSet::new();
    let mut Q: VecDeque<(String, Board)> = VecDeque::new();
    Q.push_back((String::new(), initial_state.clone()));

    let mut best_solution: (String, Board) = (String::new(), initial_state.clone());
    while !Q.is_empty() {
        let (actions, board) = Q.pop_front().unwrap();

        let all_regions = board.compute_all_regions();
        if all_regions.is_empty() {
            if board.get_score() > best_solution.1.get_score() {
                best_solution = (actions.clone(), board.clone());
                eprintln!("New Best Socre {} : {}", board.get_score(), actions);
            } else {
                eprintln!("New End {} : {}", board.get_score(), actions);
            }
            continue;
        }

        for region in all_regions {
            let mut copy = board.clone();
            let mut copy_actions = actions.clone();

            let idx = region.first().unwrap();
            let (x, y) = Board::to_coordinates(idx);
            copy.play(x, y);
            copy_actions.push_str(&format!("{} {};", x, y));

            let mut hasher = DefaultHasher::new();
            copy.hash(&mut hasher);
            let hash_value = hasher.finish();
            if visited.contains(&hash_value) {
                continue;
            }
            visited.insert(hash_value);

            eprintln!("{} : {}", copy.get_score(), copy_actions);

            // eprintln!("{:?}", copy);
            Q.push_back((copy_actions, copy));
        }

        // break;
    }

    best_solution.0
}
