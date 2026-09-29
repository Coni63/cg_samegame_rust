// Compact, column-major game state used by the search algorithms.
// Cell index is x * 16 + y, colors are stored as color + 1 and 0 means empty.
// Invariants: cells above a column height and columns >= ncols are empty.

use crate::board::Board;

pub const N: usize = 15;

#[derive(Clone, Copy)]
pub struct State {
    pub c: [u8; 256],
    pub h: [u8; N],
    pub ncols: u8,
    pub cnt: [u8; 6],
    pub score: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Reg {
    pub cell: u8,
    pub color: u8,
    pub size: u8,
}

impl State {
    pub fn from_board(board: &Board) -> State {
        let mut s = State {
            c: [0; 256],
            h: [0; N],
            ncols: 0,
            cnt: [0; 6],
            score: 0,
        };
        let mut nx = 0;
        for x in 0..N {
            let mut ny = 0;
            for y in 0..N {
                let v = board.get(x, y);
                if v < 0 {
                    break;
                }
                s.c[nx * 16 + ny] = v as u8 + 1;
                s.cnt[v as usize + 1] += 1;
                ny += 1;
            }
            if ny > 0 {
                s.h[nx] = ny as u8;
                nx += 1;
            }
        }
        s.ncols = nx as u8;
        s
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ncols == 0
    }

    #[inline(always)]
    fn flood(&self, start: usize, vis: &mut [u8; 256], mark: u8, stack: &mut [u8; 256]) -> usize {
        let color = self.c[start];
        let mut sp = 0;
        let mut size = 0;
        stack[sp] = start as u8;
        sp += 1;
        vis[start] = mark;
        while sp > 0 {
            sp -= 1;
            let i = stack[sp] as usize;
            size += 1;
            let y = i & 15;
            let x = i >> 4;
            if y < 14 && self.c[i + 1] == color && vis[i + 1] != mark {
                vis[i + 1] = mark;
                stack[sp] = (i + 1) as u8;
                sp += 1;
            }
            if y > 0 && self.c[i - 1] == color && vis[i - 1] != mark {
                vis[i - 1] = mark;
                stack[sp] = (i - 1) as u8;
                sp += 1;
            }
            if x < 14 && self.c[i + 16] == color && vis[i + 16] != mark {
                vis[i + 16] = mark;
                stack[sp] = (i + 16) as u8;
                sp += 1;
            }
            if x > 0 && self.c[i - 16] == color && vis[i - 16] != mark {
                vis[i - 16] = mark;
                stack[sp] = (i - 16) as u8;
                sp += 1;
            }
        }
        size
    }

    /// Enumerate all groups. Groups of size >= 2 go to `out`, singletons are counted per color.
    pub fn regions(&self, out: &mut Vec<Reg>, singles: &mut [u8; 6]) {
        out.clear();
        *singles = [0; 6];
        let mut vis = [0u8; 256];
        let mut stack = [0u8; 256];
        for x in 0..self.ncols as usize {
            for y in 0..self.h[x] as usize {
                let i = x * 16 + y;
                if vis[i] != 0 {
                    continue;
                }
                let size = self.flood(i, &mut vis, 1, &mut stack);
                let color = self.c[i];
                if size >= 2 {
                    out.push(Reg {
                        cell: i as u8,
                        color,
                        size: size as u8,
                    });
                } else {
                    singles[color as usize] += 1;
                }
            }
        }
    }

    pub fn has_move(&self) -> bool {
        for x in 0..self.ncols as usize {
            for y in 0..self.h[x] as usize {
                let i = x * 16 + y;
                let col = self.c[i];
                if (y < 14 && self.c[i + 1] == col) || (x < 14 && self.c[i + 16] == col) {
                    return true;
                }
            }
        }
        false
    }

    /// Remove the group containing `cell` (must be of size >= 2).
    pub fn play(&mut self, cell: usize) {
        let color = self.c[cell];
        let mut vis = [0u8; 256];
        let mut stack = [0u8; 256];
        // collect cells
        let mut masks = [0u16; N];
        let mut sp = 0;
        let mut size: u32 = 0;
        stack[sp] = cell as u8;
        sp += 1;
        vis[cell] = 1;
        let mut minx = 15;
        let mut maxx = 0;
        while sp > 0 {
            sp -= 1;
            let i = stack[sp] as usize;
            size += 1;
            let y = i & 15;
            let x = i >> 4;
            masks[x] |= 1 << y;
            if x < minx {
                minx = x;
            }
            if x > maxx {
                maxx = x;
            }
            if y < 14 && self.c[i + 1] == color && vis[i + 1] == 0 {
                vis[i + 1] = 1;
                stack[sp] = (i + 1) as u8;
                sp += 1;
            }
            if y > 0 && self.c[i - 1] == color && vis[i - 1] == 0 {
                vis[i - 1] = 1;
                stack[sp] = (i - 1) as u8;
                sp += 1;
            }
            if x < 14 && self.c[i + 16] == color && vis[i + 16] == 0 {
                vis[i + 16] = 1;
                stack[sp] = (i + 16) as u8;
                sp += 1;
            }
            if x > 0 && self.c[i - 16] == color && vis[i - 16] == 0 {
                vis[i - 16] = 1;
                stack[sp] = (i - 16) as u8;
                sp += 1;
            }
        }
        debug_assert!(size >= 2);
        self.score += (size - 2) * (size - 2);
        self.cnt[color as usize] -= size as u8;

        let mut empty_col = false;
        for x in minx..=maxx {
            let m = masks[x];
            if m == 0 {
                continue;
            }
            let base = x * 16;
            let h = self.h[x] as usize;
            let lowest = m.trailing_zeros() as usize;
            let mut w = lowest;
            for y in lowest..h {
                if m & (1 << y) == 0 {
                    self.c[base + w] = self.c[base + y];
                    w += 1;
                }
            }
            for y in w..h {
                self.c[base + y] = 0;
            }
            self.h[x] = w as u8;
            if w == 0 {
                empty_col = true;
            }
        }

        if empty_col {
            let n = self.ncols as usize;
            let mut w = minx;
            for x in minx..n {
                if self.h[x] > 0 {
                    if w != x {
                        let (a, b) = self.c.split_at_mut(x * 16);
                        a[w * 16..w * 16 + 16].copy_from_slice(&b[..16]);
                        b[..16].fill(0);
                        self.h[w] = self.h[x];
                        self.h[x] = 0;
                    }
                    w += 1;
                }
            }
            self.ncols = w as u8;
            if w == 0 {
                self.score += 1000;
            }
        }
    }

    pub fn hash(&self) -> u64 {
        let mut hh: u64 = 0xcbf29ce484222325;
        let n = self.ncols as usize * 16;
        for chunk in self.c[..n].chunks_exact(8) {
            let v = u64::from_le_bytes(chunk.try_into().unwrap());
            hh = (hh ^ v).wrapping_mul(0x100000001b3).rotate_left(29);
        }
        hh ^ (self.ncols as u64)
    }
}

pub fn cell_to_xy(cell: usize) -> (usize, usize) {
    (cell >> 4, cell & 15)
}

pub fn actions_to_string(cells: &[u8]) -> String {
    cells
        .iter()
        .map(|&c| {
            let (x, y) = cell_to_xy(c as usize);
            format!("{} {}", x, y)
        })
        .collect::<Vec<_>>()
        .join(";")
}

/// Replay a sequence on the reference engine and return its score.
pub fn verify(initial: &Board, cells: &[u8]) -> u32 {
    let mut b = initial.clone();
    for &c in cells {
        let (x, y) = cell_to_xy(c as usize);
        let before = b.get_score();
        let r = b.compute_region(x, y);
        assert!(r.len() >= 2, "illegal move {} {}", x, y);
        b.play(x, y);
        let _ = before;
    }
    b.get_score()
}

pub fn string_to_actions(actions: &str) -> Vec<u8> {
    actions
        .split(';')
        .filter(|a| !a.trim().is_empty())
        .map(|a| {
            let mut it = a.split_whitespace().map(|v| v.parse::<usize>().unwrap());
            let x = it.next().unwrap();
            let y = it.next().unwrap();
            (x * 16 + y) as u8
        })
        .collect()
}
