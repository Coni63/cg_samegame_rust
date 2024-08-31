use std::{
    collections::{HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::board::Board;

// pub fn compute_all_regions(board: &Board) -> Vec<Vec<(usize, usize)>> {
//     if let Some(all_region) = &self.all_region {
//         return all_region.clone();
//     }
//     let mut visited = [[false; 15]; 15];
//     let mut all_regions: Vec<Vec<(usize, usize)>> = Vec::new();
//     for r in 0..15 {
//         for c in 0..15 {
//             if !visited[r][c] && board.get(r, c) >= 0 {
//                 let region = board.compute_region(r, c);
//                 for (r2, c2) in region.iter() {
//                     visited[*r2][*c2] = true;
//                 }
//                 if region.len() < 2 {
//                     continue;
//                 }
//                 all_regions.push(region);
//             }
//         }
//     }
//     all_regions
// }

pub fn solve(initial_state: &Board) -> String {
    // let mut visited: HashSet<u64> = HashSet::new();
    // let mut Q: VecDeque<(String, Board)> = VecDeque::new();
    // Q.push_back((String::new(), initial_state.clone()));

    // let mut best_solution: (String, Board) = (String::new(), initial_state.clone());
    // while !Q.is_empty() {
    //     let (actions, board) = Q.pop_front().unwrap();
    //     // if Q.len() % 100 == 0 {
    //     //     eprintln!("{:?}", board);
    //     // }
    //     let all_regions = board.compute_all_regions();
    //     if all_regions.is_empty() && board.get_score() > best_solution.1.get_score() {
    //         best_solution = (actions.clone(), board.clone());
    //         eprintln!("New Best Socre {} : {}", board.get_score(), actions);
    //     }

    //     for region in all_regions {
    //         let mut copy = board.clone();
    //         let (r, c) = region.first().unwrap();
    //         copy.play(*r, *c);

    //         let mut hasher = DefaultHasher::new();
    //         copy.hash(&mut hasher);
    //         let hash_value = hasher.finish();
    //         if visited.contains(&hash_value) {
    //             continue;
    //         }
    //         visited.insert(hash_value);

    //         let mut copy_actions = actions.clone();
    //         copy_actions.push_str(&format!("{} {};", r, c));
    //         // eprintln!("{:?}", copy);
    //         Q.push_back((copy_actions, copy));
    //     }

    //     // break;
    // }

    // best_solution.0
    String::new()
}
