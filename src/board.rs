use core::fmt;
use std::{
    collections::VecDeque,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

const BOARD_SIZE: usize = 16; // Using 16 for bitwise operations
const GAME_SIZE: usize = 15; // Actual game size
const TOTAL_CELLS: usize = BOARD_SIZE * BOARD_SIZE;
const ROW_MASK: usize = BOARD_SIZE - 1; // 0b1111 for bitwise AND

pub struct Board {
    board: [i8; TOTAL_CELLS],
    score: u32,
    color_counts: [u8; 5],
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
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn play(&mut self, x: usize, y: usize) {
        let index = Board::get_index(x, y);
        let picked_color = self.board[index];
        if picked_color < 0 {
            return;
        }

        let region = self.compute_region_index(index);
        if region.len() < 2 {
            return;
        }

        for &i in &region {
            self.board[i] = -1;
        }

        self.score += u32::pow((region.len() - 2) as u32, 2);
        self.color_counts[picked_color as usize] -= region.len() as u8;

        self.apply_gravity(region);

        // if the board is fully empty, add 1000 points
        if self.color_counts.iter().sum::<u8>() == 0 {
            self.score += 1000;
        }
    }

    pub fn is_over(&self) -> bool {
        // this is not really True, we need to check that there is no group of 2 or more
        self.color_counts.iter().sum::<u8>() == 0
    }

    fn apply_gravity(&mut self, region_removed: Vec<usize>) {
        let mut start_x = GAME_SIZE;
        let mut end_x = 0;
        let mut start_y = GAME_SIZE;

        for &index in &region_removed {
            let (x, y) = Board::get_x_y(index);
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

        for x in start_x..=end_x {
            let mut cursor = Board::get_index(x, start_y);
            for y in start_y..15 {
                let idx = Board::get_index(x, y);
                if self.board[idx] >= 0 {
                    self.board[cursor] = self.board[idx];
                    self.board[idx] = -1;
                    cursor += BOARD_SIZE;
                }
            }
        }

        self.remove_empty_columns(start_x);
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
                    cursor_1_x += 1;
                } else {
                    cursor_1_x += 1;
                }
            }
        }
    }

    pub fn compute_region_index(&self, start_index: usize) -> Vec<usize> {
        let mut region = Vec::new();
        let mut stack = VecDeque::new();
        let mut visited = [false; TOTAL_CELLS];
        let color = self.board[start_index];

        stack.push_back(start_index);

        while let Some(index) = stack.pop_front() {
            if visited[index] || self.board[index] != color {
                continue;
            }

            visited[index] = true;
            region.push(index);

            let row = index >> 4; // Equivalent to index / 16
            let col = index & ROW_MASK; // Equivalent to index % 16

            if row > 0 {
                stack.push_back(index - BOARD_SIZE);
            }
            if row < GAME_SIZE - 1 {
                stack.push_back(index + BOARD_SIZE);
            }
            if col > 0 {
                stack.push_back(index - 1);
            }
            if col < GAME_SIZE - 1 {
                stack.push_back(index + 1);
            }
        }

        region
    }

    fn get_index(x: usize, y: usize) -> usize {
        (y << 4) | x // row * 16 + col
    }

    pub fn get_x_y(index: usize) -> (usize, usize) {
        let y = index >> 4;
        let x = index & 0b1111;
        (x, y)
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        self.board[Board::get_index(x, y)]
    }

    pub fn compute_region(&self, x: usize, y: usize) -> Vec<usize> {
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
        }
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.score.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_1() {
        let grid = [
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
        ];

        let mut board = Board::new(grid);

        // eprintln!("{:?}", board);

        assert_eq!(board.score, 0);
        assert_eq!(board.color_counts, [45, 90, 60, 15, 15]);
        assert!(!board.is_over());

        for _ in 0..12 {
            board.play(0, 0);
        }
        eprintln!("{:?}", board);

        assert_eq!(board.score, 4873);
        assert_eq!(board.color_counts, [0, 0, 0, 0, 0]);
        assert!(board.is_over());
    }

    #[test]
    fn test_board_2() {
        let grid = [
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
        ];

        let mut board = Board::new(grid);
        assert_eq!(board.score, 0);
        assert_eq!(board.color_counts, [45, 90, 60, 15, 15]);
        assert!(!board.is_over());

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
        assert!(board.is_over());
    }

    #[test]
    fn test_board_3() {
        let grid = [
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
        ];

        let mut board = Board::new(grid);
        assert_eq!(board.score, 0);
        assert_eq!(board.color_counts, [45, 75, 45, 30, 30]);
        assert!(!board.is_over());

        board.play(5, 0);
        board.play(1, 0);
        board.play(1, 0);
        board.play(2, 0);
        board.play(2, 0);
        board.play(0, 0);
        board.play(0, 0);
        board.play(0, 0);

        eprintln!("{:?}", board);

        assert_eq!(board.score, 7107);
        assert_eq!(board.color_counts, [0, 0, 0, 0, 0]);
        assert!(board.is_over());
    }

    #[test]
    fn test_board_4() {
        let grid = [
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
        ];

        let mut board = Board::new(grid);
        assert_eq!(board.score, 0);
        assert_eq!(board.color_counts, [63, 45, 45, 36, 36]);
        assert!(!board.is_over());

        board.play(6, 3);
        board.play(12, 6);
        board.play(12, 3);
        board.play(3, 9);
        board.play(3, 3);
        board.play(3, 0);
        board.play(9, 0);
        board.play(3, 3);
        board.play(0, 3);

        assert_eq!(board.score, 4033);
        assert_eq!(board.color_counts, [0, 0, 0, 0, 0]);
        assert!(board.is_over());
    }
}
