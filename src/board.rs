use std::collections::VecDeque;

#[derive(Debug)]
pub struct Board {
    pub board: [[u8; 15]; 15],
    pub score: u32,
    pub color: [u8; 5],
}

impl Board {
    pub fn new(board: [[u8; 15]; 15]) -> Board {
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

    pub fn compute_region(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let color = self.board[row][col];
        let mut visited = [[false; 15]; 15];
        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
        stack.push_back((row, col));

        while !stack.is_empty() {
            let (row, col) = stack.pop_front().unwrap();
            eprintln!("{} {}", row, col);
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
