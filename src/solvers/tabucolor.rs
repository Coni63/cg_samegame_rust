// https://project.dke.maastrichtuniversity.nl/games/files/bsc/Tak_Bsc-paper.pdf

use lru::LruCache;
use rand::Rng;
use std::num::NonZeroUsize;

use crate::{board::Board, region::Region};

const UCT_CONSTANT: f32 = 5.96;
const DEVIATION_CONSTANT: f32 = 67.98;
const VISITS_BEFORE_EXPAND: f32 = 13.0;
const TOP_SCORE_WEIGHT: f32 = 0.49;
const CHANCE_CHOSEN_COLOR: f32 = 0.0007;

struct Node<'a> {
    visited: f32,
    highest_score: f32,
    sum_score: f32,
    board: &'a Board,
    childs: Vec<&'a Self>,
    parent: Option<&'a mut Self>,
    leaf: bool,
}

impl<'a> Node<'a> {
    pub fn new(board: &'a Board) -> Node {
        Node {
            visited: 0.0,
            highest_score: 0.0,
            sum_score: 0.0,
            board,
            childs: vec![],
            parent: None,
            leaf: false,
        }
    }

    pub fn get_board(&self) -> &Board {
        self.board
    }

    pub fn set_parent(&mut self, parent: &'a mut Self) {
        self.parent = Some(parent);
    }

    pub fn add_child(&mut self, child: &'a Self) {
        self.childs.push(child);
    }

    pub fn is_expanded(&self) -> bool {
        self.childs.is_empty()
    }

    pub fn set_leaf(&mut self) {
        self.leaf = true;
    }

    pub fn backpropagate(&mut self, score: f32) {
        self.sum_score += score;
        self.visited += 1.0;
        if score > self.highest_score {
            self.highest_score = score;
        }

        if let Some(ref mut node) = self.parent {
            node.backpropagate(score);
        }
    }

    pub fn get_child(&self) -> Option<&'a Node> {
        let mut best_child: Option<&Node> = None;
        let mut best_uct = 0.0f32;
        for &child in self.childs.iter() {
            let x_bar = if child.visited > 0.0 {
                child.sum_score / child.visited
            } else {
                0.0f32
            };

            let p1 = (self.visited.log2() / child.visited).sqrt();
            let p2 = if self.is_expanded() {
                let a: f32 = self.childs.iter().map(|x| x.sum_score * x.sum_score).sum();
                let b = self.visited * x_bar * x_bar;
                ((a + b + DEVIATION_CONSTANT) / self.visited).sqrt()
            } else {
                f32::MAX
            };
            let uct = x_bar + p1 + p2;
            if uct > best_uct {
                best_uct = uct;
                best_child = Some(child)
            }
        }

        best_child
    }
}

pub fn _solve(initial_state: &Board) -> (String, u32) {
    let mut best_probe = initial_state.clone();
    let board = initial_state.clone();

    let mut cache_region: LruCache<Board, Vec<Region>> =
        LruCache::new(NonZeroUsize::new(1000000).unwrap());

    let root = Node::new(initial_state);

    for _ in 0..1000000 {
        let mut node = &root;

        // selection
        while node.is_expanded() {
            node = node.get_child().unwrap();
        }

        // rollout
        let probe = rollout(&board, &mut cache_region);

        // backpropagate
        let score = probe.get_score() as f32;
        node.backpropagate(score);

        // keep the best probe in case of better solution than the MCTS
        if probe.get_score() > best_probe.get_score() {
            best_probe = probe.clone();
        }

        // expand
        if node.visited > VISITS_BEFORE_EXPAND {
            let all_regions = node.get_board().compute_all_regions();
            if all_regions.is_empty() {
                node.set_leaf();
            }
            for region in all_regions.iter() {
                let copy = node.get_board().clone();
                copy.play_region(region);
                node.add_child(&Node::new(&copy));
            }
        }
    }

    (best_probe.get_actions_str(), best_probe.get_score())
}

fn rollout(board: &Board, cache: &mut LruCache<Board, Vec<Region>>) -> Board {
    let mut copy = board.clone();
    let mut rng = rand::thread_rng();

    loop {
        let all_regions = cache.get_or_insert(copy.clone(), || copy.compute_all_regions());
        if all_regions.is_empty() {
            break;
        }

        let mut count_color = [0u8; 5];
        for region in all_regions.iter() {
            count_color[region.color as usize] += region.len() as u8;
        }

        let p = get_probs(&count_color);

        let color_to_pick = pick_index(&p);

        let all_region_of_color: Vec<&Region> = all_regions
            .iter()
            .filter(|&region| region.color == color_to_pick)
            .collect();

        if !all_region_of_color.is_empty() {
            let picked_region = rng.gen_range(0..all_region_of_color.len());

            copy.play_region(all_region_of_color[picked_region]);
        }
    }

    copy
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_probs() {}
// }
