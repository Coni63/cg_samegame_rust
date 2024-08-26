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
}
