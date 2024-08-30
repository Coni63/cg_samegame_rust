use core::fmt;
use std::{
    collections::VecDeque,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

pub struct Board {
    board: [[i8; 15]; 15],
    score: u32,
    color: [u8; 5],
    all_region: Option<Vec<Vec<(usize, usize)>>>,
}

impl Board {
    pub fn new(board: [[i8; 15]; 15]) -> Board {
        let mut color = [0; 5];
        for i in 0..15 {
            for j in 0..15 {
                color[board[i][j] as usize] += 1;
            }
        }

        Board {
            board,
            score: 0,
            color,
            all_region: None,
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn play(&mut self, row: usize, col: usize) {
        let picked_color = self.board[row][col];
        // eprintln!("{}", picked_color);
        if picked_color < 0 {
            return;
        }

        let region = self.compute_region(row, col);
        for (r, c) in &region {
            self.board[*r][*c] = -1;
        }
        // eprintln!("{:?} {}", self.color, region.len());
        // eprintln!("{:?}", region);
        self.color[picked_color as usize] -= region.len() as u8;
        // eprintln!("{:?} {}", self.color, region.len());

        if region.len() >= 2 {
            self.score += u32::pow((region.len() - 2) as u32, 2);
        }

        self.apply_gravity(region);

        if self.is_over() {
            self.score += 1000;
        }
    }

    pub fn is_over(&self) -> bool {
        self.color.iter().sum::<u8>() == 0
    }

    fn apply_gravity(&mut self, region_removed: Vec<(usize, usize)>) {
        // gravity is toward the top (row 14 is the top row)
        let mut start_col: usize = 15;
        let mut end_col: usize = 0;
        let mut start_row: usize = 15;

        for (r, c) in region_removed {
            if r < start_row {
                start_row = r;
            }
            if c < start_col {
                start_col = c;
            }
            if c > end_col {
                end_col = c;
            }
        }
        // eprintln!(
        //     "start_row: {}, start_col: {}, end_col: {}",
        //     start_row, start_col, end_col
        // );

        for c in start_col..=end_col {
            let mut cursor = start_row;
            for r in start_row..15 {
                if self.board[r][c] >= 0 {
                    self.board[cursor][c] = self.board[r][c];
                    self.board[r][c] = -1;
                    cursor += 1;
                }
            }
        }

        self.remove_column(start_col, end_col);
    }

    fn remove_column(&mut self, start_col: usize, end_col: usize) {
        let mut offset = 0;
        for c in start_col..15 {
            let mut empty = true;
            for r in 0..15 {
                if self.board[r][c] >= 0 {
                    empty = false;
                    break;
                }
            }

            if empty {
                offset += 1;
            } else {
                for r in start_col..15 {
                    let temp = self.board[r][c];
                    self.board[r][c] = self.board[r][c - offset];
                    self.board[r][c - offset] = temp;
                }
            }

            if c == end_col && offset == 0 {
                // if we reach the end of the impacted area and there is no empty space, there is no need to remove anything
                return;
            }
        }
    }

    pub fn compute_all_regions(&self) -> Vec<Vec<(usize, usize)>> {
        if let Some(all_region) = &self.all_region {
            return all_region.clone();
        }
        let mut visited = [[false; 15]; 15];
        let mut all_regions: Vec<Vec<(usize, usize)>> = Vec::new();
        for r in 0..15 {
            for c in 0..15 {
                if !visited[r][c] && self.board[r][c] >= 0 {
                    let region = self.compute_region(r, c);
                    for (r2, c2) in region.iter() {
                        visited[*r2][*c2] = true;
                    }
                    if region.len() < 2 {
                        continue;
                    }
                    all_regions.push(region);
                }
            }
        }
        all_regions
    }

    fn compute_region(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let color = self.board[row][col];
        let mut visited = [[false; 15]; 15];
        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
        stack.push_back((row, col));

        while !stack.is_empty() {
            let (row, col) = stack.pop_front().unwrap();
            if visited[row][col] {
                continue;
            }
            visited[row][col] = true;
            ans.push((row, col));

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = row as i32 + dx;
                let ny = col as i32 + dy;

                if (0..15).contains(&nx)
                    && (0..15).contains(&ny)
                    && self.board[nx as usize][ny as usize] == color
                    && !visited[nx as usize][ny as usize]
                {
                    stack.push_back((nx as usize, ny as usize));
                }
            }
        }
        ans
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "Score: {} - {:?}", self.score, self.color)?;
        for row in 0..15 {
            for col in 0..15 {
                let char = if self.board[14 - row as usize][col as usize] < 0 {
                    String::from('-')
                } else {
                    self.board[14 - row as usize][col as usize].to_string()
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
            color: self.color,
            board: self.board,
            all_region: self.all_region.clone(),
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
        assert_eq!(board.score, 0);
        assert_eq!(board.color, [45, 90, 60, 15, 15]);
        assert!(!board.is_over());

        for _ in 0..12 {
            board.play(0, 0);
        }

        assert_eq!(board.score, 4873);
        assert_eq!(board.color, [0, 0, 0, 0, 0]);

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
        assert_eq!(board.color, [45, 90, 60, 15, 15]);
        assert!(!board.is_over());

        board.play(0, 1);
        board.play(0, 4);
        board.play(0, 3);
        board.play(0, 5);
        board.play(0, 5);
        board.play(0, 5);
        board.play(0, 0);
        board.play(0, 0);

        eprintln!("{:?}", board);
        eprintln!("{:?}", board.compute_all_regions());

        assert_eq!(board.score, 11607);
        assert_eq!(board.color, [0, 0, 0, 0, 0]);
        assert!(board.is_over());
    }
}
