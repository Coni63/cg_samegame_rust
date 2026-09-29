// Nested search using the beam as playout (NMCS-like):
// at each step every move is tried and completed with a small beam, the best complete sequence
// found so far is memorized and the next move of that sequence is played.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use fxhash::FxHashSet;

use super::beam::{beam, Params};
use super::fast::{Reg, State};

pub fn nested(root: &State, p: &Params, threads: usize, initial: Option<(Vec<u8>, u32)>) -> (Vec<u8>, u32) {
    let start = Instant::now();
    let mut playout = *p;
    playout.threads = 1;

    let mut best: (Vec<u8>, u32) = initial.unwrap_or((vec![], 0));
    let mut state = *root;
    let mut prefix: Vec<u8> = Vec::new();
    let mut regs: Vec<Reg> = Vec::new();
    let mut singles = [0u8; 6];

    loop {
        state.regions(&mut regs, &mut singles);
        if regs.is_empty() {
            break;
        }
        // children, deduplicated (different groups can't give the same state, but keep it cheap anyway)
        let mut seen = FxHashSet::default();
        let children: Vec<(u8, State)> = regs
            .iter()
            .filter_map(|r| {
                let mut c = state;
                c.play(r.cell as usize);
                seen.insert(c.hash()).then_some((r.cell, c))
            })
            .collect();

        let next = AtomicUsize::new(0);
        let results: Mutex<Vec<(Vec<u8>, u32)>> = Mutex::new(Vec::new());
        std::thread::scope(|sc| {
            for t in 0..threads.min(children.len()) {
                let children = &children;
                let next = &next;
                let results = &results;
                let prefix = &prefix;
                let mut pp = playout;
                pp.seed = p.seed ^ (prefix.len() as u64 * 7919 + t as u64);
                sc.spawn(move || loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    if i >= children.len() {
                        break;
                    }
                    let (cell, child) = &children[i];
                    let (moves, score) = beam(child, &pp);
                    let mut seq = prefix.clone();
                    seq.push(*cell);
                    seq.extend(moves);
                    results.lock().unwrap().push((seq, score));
                });
            }
        });

        for (seq, score) in results.into_inner().unwrap() {
            if score > best.1 || best.0.len() <= prefix.len() {
                best = (seq, score);
            }
        }
        eprintln!(
            "  step {:2} ({:2} moves): best {} ({:.0}s)",
            prefix.len(),
            children.len(),
            best.1,
            start.elapsed().as_secs_f32()
        );

        let mv = best.0[prefix.len()];
        state.play(mv as usize);
        prefix.push(mv);
    }
    best
}
