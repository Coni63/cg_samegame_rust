use core::fmt;
use std::{
    collections::VecDeque,
    fmt::{Debug, Formatter},
};

pub struct Board {
    board: [[i8; 15]; 15],
    score: u32,
    color: [u8; 5],
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
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn play(&mut self, row: usize, col: usize) {
        let picked_color = self.board[row][col];
        if picked_color < 0 {
            return;
        }

        let region = self.compute_region(row, col);
        for (r, c) in &region {
            self.board[*r][*c] = -1;
        }
        self.color[picked_color as usize] -= region.len() as u8;
        self.score += u32::pow((region.len() - 2) as u32, 2);

        self.apply_gravity(region);
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

    fn compute_region(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut ans = vec![(row, col)];
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

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = row as i32 + dx;
                let ny = col as i32 + dy;

                if (0..15).contains(&nx)
                    && (0..15).contains(&ny)
                    && self.board[nx as usize][ny as usize] == color
                    && !visited[nx as usize][ny as usize]
                {
                    stack.push_back((nx as usize, ny as usize));
                    ans.push((nx as usize, ny as usize));
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
                write!(
                    f,
                    "{}",
                    &self.board[14 - row as usize][col as usize].to_string()
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
