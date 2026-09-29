// Multithreaded beam search with a "target color" evaluation:
// one color is kept as intact as possible so that it can be removed in one huge group at the end.

use fxhash::FxHashSet;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use super::fast::{Reg, State};

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub width: usize,
    pub target: u8, // 1..=5, 0 = no target
    pub group_weight: f32,
    pub single_penalty: f32,
    pub target_weight: f32,
    pub other_weight: f32,
    pub noise: f32,
    pub seed: u64,
    pub threads: usize,
}

#[derive(Clone, Copy)]
struct Cand {
    eval: f32,
    hash: u64,
    parent: u32,
    cell: u8,
}

pub fn evaluate(s: &State, regs: &[Reg], singles: &[u8; 6], p: &Params) -> f32 {
    let mut e = s.score as f32;
    let t = p.target as usize;
    let mut groups = 0f32;
    let mut sq = [0f32; 6];
    for c in 1..6 {
        sq[c] = singles[c] as f32;
    }
    for r in regs {
        let v = r.size as f32;
        if r.color as usize != t {
            groups += (v - 2.0) * (v - 2.0);
        }
        sq[r.color as usize] += v * v;
    }
    e += p.group_weight * groups;
    for c in 1..6 {
        let n = s.cnt[c] as f32;
        if c != t {
            e -= p.single_penalty * singles[c] as f32;
        }
        if n >= 3.0 {
            // connectivity potential: equals the real gain once the whole color is a single group
            let w = if c == t { p.target_weight } else { p.other_weight };
            e += w * (n - 2.0) * (n - 2.0) * sq[c] / (n * n);
        }
    }
    e
}

/// Generate and evaluate all children of `part` (whose first state has index `base` in the layer).
/// Also returns the best terminal state of `part`, as (score, index).
fn expand(part: &[State], base: usize, p: &Params, seed: u64) -> (Vec<Cand>, Option<(u32, u32)>) {
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut cands = Vec::with_capacity(part.len() * 30);
    let mut regs = Vec::with_capacity(64);
    let mut cregs = Vec::with_capacity(64);
    let mut singles = [0u8; 6];
    let mut term: Option<(u32, u32)> = None;
    for (k, s) in part.iter().enumerate() {
        s.regions(&mut regs, &mut singles);
        if regs.is_empty() {
            if term.map_or(true, |(sc, _)| s.score > sc) {
                term = Some((s.score, (base + k) as u32));
            }
            continue;
        }
        for r in regs.iter() {
            let mut child = *s;
            child.play(r.cell as usize);
            child.regions(&mut cregs, &mut singles);
            let mut e = evaluate(&child, &cregs, &singles, p);
            if p.noise > 0.0 {
                e += rng.gen::<f32>() * p.noise;
            }
            cands.push(Cand {
                eval: e,
                hash: child.hash(),
                parent: (base + k) as u32,
                cell: r.cell,
            });
        }
    }
    (cands, term)
}

/// Returns (best move sequence, score) found by the beam starting from `root`.
pub fn beam(root: &State, p: &Params) -> (Vec<u8>, u32) {
    let mut layer: Vec<State> = vec![*root];
    let mut history: Vec<Vec<(u32, u8)>> = Vec::new();
    let mut best_score = 0u32;
    let mut best_at: Option<(usize, u32)> = None; // (depth, index in layer)
    if !root.has_move() {
        return (vec![], root.score);
    }

    loop {
        let depth = history.len();
        let nthreads = p.threads.max(1).min((layer.len() + 63) / 64);
        let chunk = (layer.len() + nthreads - 1) / nthreads;
        // expansion
        let seed = p.seed ^ ((depth as u64) << 32);
        let results: Vec<(Vec<Cand>, Option<(u32, u32)>)> = if nthreads == 1 {
            vec![expand(&layer, 0, p, seed)]
        } else {
            std::thread::scope(|sc| {
                let handles: Vec<_> = layer
                    .chunks(chunk)
                    .enumerate()
                    .map(|(ti, part)| {
                        let seed = seed ^ (ti as u64).wrapping_mul(0x9E3779B97F4A7C15);
                        sc.spawn(move || expand(part, ti * chunk, p, seed))
                    })
                    .collect();
                handles.into_iter().map(|h| h.join().unwrap()).collect()
            })
        };

        let mut cands: Vec<Cand> = Vec::new();
        for (c, term) in results {
            if let Some((sc, idx)) = term {
                if sc > best_score || best_at.is_none() {
                    best_score = sc;
                    best_at = Some((depth, idx));
                }
            }
            cands.extend(c);
        }
        if cands.is_empty() {
            break;
        }

        // selection
        let keep = (p.width * 2).min(cands.len());
        if keep < cands.len() {
            cands.select_nth_unstable_by(keep - 1, |a, b| b.eval.partial_cmp(&a.eval).unwrap());
            cands.truncate(keep);
        }
        cands.sort_unstable_by(|a, b| b.eval.partial_cmp(&a.eval).unwrap());
        let mut seen: FxHashSet<u64> = FxHashSet::default();
        let mut chosen: Vec<(u32, u8)> = Vec::with_capacity(p.width);
        for c in cands.iter() {
            if chosen.len() >= p.width {
                break;
            }
            if seen.insert(c.hash) {
                chosen.push((c.parent, c.cell));
            }
        }

        // materialization
        let new_layer: Vec<State> = chosen
            .iter()
            .map(|&(parent, cell)| {
                let mut s = layer[parent as usize];
                s.play(cell as usize);
                s
            })
            .collect();
        history.push(chosen);
        layer = new_layer;
    }

    // reconstruct
    let (depth, mut idx) = best_at.unwrap();
    let mut moves = Vec::with_capacity(depth);
    for d in (0..depth).rev() {
        let (parent, cell) = history[d][idx as usize];
        moves.push(cell);
        idx = parent;
    }
    moves.reverse();
    (moves, best_score)
}
