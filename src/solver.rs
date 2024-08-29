use std::collections::VecDeque;

use crate::board::Board;

pub fn solve(initial_state: &Board) -> String {
    let mut Q: VecDeque<(String, Board)> = VecDeque::new();
    Q.push_back((String::new(), initial_state.clone()));

    let mut best_solution: (String, Board) = (String::new(), initial_state.clone());
    while !Q.is_empty() {
        let (actions, board) = Q.pop_front().unwrap();
        // if Q.len() % 100 == 0 {
        //     eprintln!("{:?}", board);
        // }
        let all_regions = board.compute_all_regions();
        if all_regions.is_empty() && board.get_score() > best_solution.1.get_score() {
            best_solution = (actions.clone(), board.clone());
            eprintln!("New Best Socre {}", board.get_score());
        }

        for region in all_regions {
            let mut copy = board.clone();
            let (r, c) = region.first().unwrap();
            copy.play(*r, *c);
            let mut copy_actions = actions.clone();
            copy_actions.push_str(&format!("{} {};", c, r));
            // eprintln!("{:?}", copy);
            Q.push_back((copy_actions, copy));
        }
        // break;
    }

    best_solution.0
}
