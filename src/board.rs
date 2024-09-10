use core::fmt;
use std::{
    collections::VecDeque,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

use crate::region::Region;

const BOARD_SIZE: usize = 16; // Using 16 for bitwise operations
const GAME_SIZE: usize = 15; // Actual game size
const TOTAL_CELLS: usize = BOARD_SIZE * BOARD_SIZE;
const ROW_MASK: usize = BOARD_SIZE - 1; // 0b1111 for bitwise AND

pub struct Board {
    board: [i8; TOTAL_CELLS],
    score: u32,
    color_counts: [u8; 5],
    actions: Vec<usize>,
}

impl Board {
    pub fn new(initial_board: [[i8; GAME_SIZE]; GAME_SIZE]) -> Board {
        let mut board = [-1; TOTAL_CELLS]; // Initialize all cells as empty
        let mut color_counts = [0; 5];

        for (y, row) in initial_board.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell >= 0 {
                    let index = Board::get_index(x, y);
                    board[index] = cell;
                    color_counts[cell as usize] += 1;
                }
            }
        }

        Board {
            board,
            score: 0,
            color_counts,
            actions: Vec::new(),
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_actions_str(&self) -> String {
        let strings: Vec<String> = self
            .actions
            .iter()
            .map(|idx| {
                let (x, y) = Board::to_coordinates(idx);
                format!("{} {}", x, y)
            })
            .collect();

        itertools::join(strings, ";")
    }

    pub fn get_color_of_index(&self, idx: &usize) -> &i8 {
        &self.board[*idx]
    }

    pub fn play_index(&mut self, index: usize) {
        let picked_color = self.board[index];
        if picked_color < 0 {
            return;
        }

        let region = self.compute_region_index(index);
        if region.len() < 2 {
            return;
        }

        self.play_region(&region);
    }

    pub fn play(&mut self, x: usize, y: usize) {
        let index = Board::get_index(x, y);
        self.play_index(index)
    }

    pub fn play_region(&mut self, region: &Region) {
        self.actions.push(region.first_position);

        for i in region.positions.iter() {
            self.board[*i] = -1;
        }

        self.score += region.score();
        self.color_counts[region.color as usize] -= region.len() as u8;

        let (start_x, start_y, end_x) = self.get_region_boundaries(&region);
        self.apply_gravity(start_x, start_y, end_x);
        self.remove_empty_columns(start_x);

        // if the board is fully empty, add 1000 points
        if self.is_empty() {
            self.score += 1000;
        }
    }

    fn is_empty(&self) -> bool {
        self.board[Board::get_index(0, 0)] == -1
    }

    fn get_region_boundaries(&self, region_removed: &Region) -> (usize, usize, usize) {
        let mut start_x = GAME_SIZE;
        let mut end_x = 0;
        let mut start_y = GAME_SIZE;

        for &index in region_removed.positions.iter() {
            let (x, y) = Board::to_coordinates(&index);
            if y < start_y {
                start_y = y;
            }
            if x < start_x {
                start_x = x;
            }
            if x > end_x {
                end_x = x;
            }
        }

        (start_x, start_y, end_x)
    }

    fn apply_gravity(&mut self, start_x: usize, start_y: usize, end_x: usize) {
        for x in start_x..=end_x {
            let mut cursor = Board::get_index(x, start_y);
            for y in start_y..15 {
                if self.board[cursor] >= 0 {
                    cursor += BOARD_SIZE;
                    continue;
                }
                let idx = Board::get_index(x, y);
                if self.board[idx] >= 0 {
                    self.board[cursor] = self.board[idx];
                    self.board[idx] = -1;
                    cursor += BOARD_SIZE;
                }
            }
        }
    }

    fn remove_empty_columns(&mut self, start_x: usize) {
        let mut cursor_1_x = start_x;
        for cursor_2_x in start_x..GAME_SIZE {
            if self.get(cursor_2_x, 0) >= 0 {
                if cursor_2_x > cursor_1_x {
                    for y in 0..15 {
                        let idx_cur1 = Board::get_index(cursor_1_x, y);
                        let idx_cur2 = Board::get_index(cursor_2_x, y);
                        self.board[idx_cur1] = self.board[idx_cur2];
                        self.board[idx_cur2] = -1;
                    }
                }
                cursor_1_x += 1;
            }
        }
    }

    pub fn compute_region_index(&self, start_index: usize) -> Region {
        let mut visited = [false; TOTAL_CELLS];
        self.inner_compute_region(start_index, &mut visited)
    }

    fn inner_compute_region(
        &self,
        start_index: usize,
        visited: &mut [bool; TOTAL_CELLS],
    ) -> Region {
        let mut region = Vec::new();
        let mut stack = VecDeque::new();
        let color = self.board[start_index];

        stack.push_back(start_index);

        while let Some(index) = stack.pop_front() {
            if visited[index] {
                continue;
            }
            visited[index] = true;
            region.push(index);

            let (x, y) = Board::to_coordinates(&index);

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < GAME_SIZE as i32 && ny >= 0 && ny < GAME_SIZE as i32 {
                    let neighbor = Board::get_index(nx as usize, ny as usize);
                    if self.board[neighbor] == color {
                        stack.push_back(neighbor);
                    }
                }
            }
        }

        Region {
            positions: region,
            color,
            first_position: start_index,
        }
    }

    pub fn compute_all_regions(&self) -> Vec<Region> {
        let mut visited = [false; TOTAL_CELLS];
        let mut all_regions: Vec<Region> = Vec::new();

        for x in 0..15 {
            for y in 0..15 {
                let index = Board::get_index(x, y);
                if visited[index] {
                    continue;
                }

                let color = self.board[index];
                if color < 0 {
                    break;
                }

                let region = self.inner_compute_region(index, &mut visited);
                if region.len() < 2 {
                    continue;
                }
                all_regions.push(region);
            }
        }
        all_regions
    }

    fn get_index(x: usize, y: usize) -> usize {
        (y << 4) | x // row * 16 + col
    }

    pub fn to_coordinates(index: &usize) -> (usize, usize) {
        let y = index >> 4;
        let x = index & ROW_MASK;
        (x, y)
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        self.board[Board::get_index(x, y)]
    }

    pub fn compute_region(&self, x: usize, y: usize) -> Region {
        let start_index = Board::get_index(x, y);
        self.compute_region_index(start_index)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "Score: {} - {:?}", self.score, self.color_counts)?;
        for y in 0..15 {
            for x in 0..15 {
                let char = if self.get(x, 14 - y) < 0 {
                    String::from('-')
                } else {
                    self.get(x, 14 - y).to_string()
                };
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            score: self.score,
            color_counts: self.color_counts,
            board: self.board,
            actions: self.actions.clone(),
        }
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.score.hash(state);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.board == other.board
    }
}

impl Eq for Board {}

#[cfg(test)]
mod tests {
    use std::hash::DefaultHasher;

    use super::*;

    fn get_board(level: i32) -> Board {
        let board = match level {
            1 => [
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
            ],
            2 => [
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
                [1, 2, 2, 0, 0, 1, 1, 4, 3, 3, 4, 1, 1, 2, 0],
            ],
            3 => [
                [2, 2, 2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2],
                [2, 2, 2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2],
                [2, 2, 2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2],
                [4, 4, 4, 1, 1, 1, 3, 3, 3, 2, 2, 2, 0, 0, 0],
                [4, 4, 4, 1, 1, 1, 3, 3, 3, 2, 2, 2, 0, 0, 0],
                [4, 4, 4, 1, 1, 1, 3, 3, 3, 2, 2, 2, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 4, 4, 4, 1, 1, 1, 4, 4, 4],
                [0, 0, 0, 0, 0, 0, 4, 4, 4, 1, 1, 1, 4, 4, 4],
                [0, 0, 0, 0, 0, 0, 4, 4, 4, 1, 1, 1, 4, 4, 4],
                [1, 1, 1, 3, 3, 3, 2, 2, 2, 3, 3, 3, 0, 0, 0],
                [1, 1, 1, 3, 3, 3, 2, 2, 2, 3, 3, 3, 0, 0, 0],
                [1, 1, 1, 3, 3, 3, 2, 2, 2, 3, 3, 3, 0, 0, 0],
                [4, 4, 4, 0, 0, 0, 3, 3, 3, 1, 1, 1, 2, 2, 2],
                [4, 4, 4, 0, 0, 0, 3, 3, 3, 1, 1, 1, 2, 2, 2],
                [4, 4, 4, 0, 0, 0, 3, 3, 3, 1, 1, 1, 2, 2, 2],
            ],
            _ => panic!("Invalid"),
        };

        Board::new(board)
    }

    #[test]
    fn test_board_1() {
        let mut board = get_board(1);

        assert_eq!(board.score, 0);
        assert_eq!(board.color_counts, [45, 90, 60, 15, 15]);

        for _ in 0..12 {
            board.play(0, 0);
        }

        assert_eq!(board.score, 4873);
        assert_eq!(board.color_counts, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_board_2() {
        let mut board = get_board(1);
        assert_eq!(board.score, 0);
        assert_eq!(board.color_counts, [45, 90, 60, 15, 15]);

        board.play(0, 1);
        board.play(0, 4);
        board.play(0, 3);
        board.play(0, 5);
        board.play(0, 5);
        board.play(0, 5);
        board.play(0, 0);
        board.play(0, 0);

        assert_eq!(board.score, 11607);
        assert_eq!(board.color_counts, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_board_3() {
        let mut board = get_board(2);
        assert_eq!(board.score, 0);
        assert_eq!(board.color_counts, [45, 75, 45, 30, 30]);

        board.play(5, 0);
        board.play(1, 0);
        board.play(1, 0);
        board.play(2, 0);
        board.play(2, 0);
        board.play(0, 0);
        board.play(0, 0);
        board.play(0, 0);

        assert_eq!(board.score, 7107);
        assert_eq!(board.color_counts, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_board_4() {
        let mut board = get_board(3);
        assert_eq!(board.score, 0);
        assert_eq!(board.color_counts, [63, 45, 45, 36, 36]);

        board.play(0, 14);
        board.play(6, 11);
        board.play(6, 11);
        board.play(3, 5);
        board.play(0, 8);
        board.play(6, 5);
        board.play(6, 8);
        board.play(6, 5);
        board.play(0, 5);
        board.play(0, 5);
        board.play(6, 5);
        board.play(6, 5);
        board.play(0, 2);
        board.play(0, 2);

        assert_eq!(board.score, 5421);
        assert_eq!(board.color_counts, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_all_regions() {
        let board = get_board(3);

        assert_eq!(board.compute_all_regions().len(), 23);
    }

    #[test]
    fn test_play() {
        let mut board = get_board(3);

        board.play(6, 11);

        assert_eq!(board.get(6, 0), 0);
        assert_eq!(board.get(6, 3), 3);
        assert_eq!(board.get(6, 6), 4);
        assert_eq!(board.get(6, 9), 3);
        assert_eq!(board.get(6, 12), -1);
    }

    #[test]
    fn test_conversion() {
        for x in 0..16 {
            for y in 0..16 {
                let idx = Board::get_index(x, y);

                assert_eq!(idx, x + y * 16);

                let (x2, y2) = Board::to_coordinates(&idx);
                assert_eq!(x, x2);
                assert_eq!(y, y2);
            }
        }
    }

    #[test]
    fn test_hash() {
        let board1 = get_board(1);
        let board2 = get_board(1);
        let board3 = get_board(3);

        let mut hasher = DefaultHasher::new();
        board1.hash(&mut hasher);
        let hash_value1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        board2.hash(&mut hasher);
        let hash_value2 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        board3.hash(&mut hasher);
        let hash_value3 = hasher.finish();

        assert_eq!(hash_value1, hash_value2);
        assert_ne!(hash_value1, hash_value3);
    }
}
