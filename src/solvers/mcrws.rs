// https://liacs.leidenuniv.nl/~takesfw/pdf/samegame.pdf

use rand::Rng;

use crate::board::Board;

pub fn _solve(initial_state: &Board) -> (String, u32) {
    let mut board = initial_state.clone();
    let k = 1000;

    let mut depth = 1;
    while !board.is_over() {
        let all_regions = board.compute_all_regions();

        eprintln!("Depth: {}", depth);

        if all_regions.len() == 1 {
            let region = all_regions.first().unwrap();
            board.play_region(region);
        } else {
            let mut highest_average_score = 0;
            let mut local_best_board = board.clone();
            for region in board.compute_all_regions() {
                let mut copy = board.clone();
                copy.play_region(&region);

                let mut average_score = 0;
                for _ in 0..k {
                    average_score += rollout(&copy)
                }

                if average_score > highest_average_score {
                    highest_average_score = average_score;
                    local_best_board = copy;
                }
            }

            eprintln!("Highest average score: {}", highest_average_score);
            board = local_best_board;
        }
        eprintln!("{:?}", board);

        depth += 1;
    }

    (board.get_actions_str(), board.get_score())
}

fn rollout(board: &Board) -> u32 {
    let mut copy = board.clone();
    let mut rng = rand::thread_rng();

    while !copy.is_over() {
        let all_regions = copy.compute_all_regions();
        let mut count_color = [0u8; 5];
        for region in all_regions.iter() {
            let first_idx = region.first().unwrap();
            let color_region = copy.get_color_of_index(first_idx);

            count_color[*color_region as usize] += region.len() as u8;
        }

        // let p = get_probs(copy.get_color_count());
        let p = get_probs(&count_color);

        let color_to_pick = pick_index(&p);

        let all_region_of_color: Vec<Vec<usize>> = all_regions
            .iter()
            .filter(|&region| {
                let first_idx = region.first().unwrap();
                let color_region = copy.get_color_of_index(first_idx);
                color_to_pick == *color_region
            })
            .cloned()
            .collect();

        let picked_region = rng.gen_range(0..all_region_of_color.len());

        copy.play_region(&all_region_of_color[picked_region]);
    }

    copy.get_score()
}

fn get_probs(colors: &[u8; 5]) -> [f32; 5] {
    let mut ans = [0f32; 5];

    let color_float: Vec<(usize, f32)> = colors
        .iter()
        .enumerate()
        .map(|(i, &c)| (i, c as f32))
        .filter(|(_, x)| *x > 0.0)
        .collect();

    if color_float.len() == 1 {
        let (i, __iterator_get_unchecked) = color_float[0];
        ans[i] = 1.0;
        return ans;
    }

    let beta: f32 = 4.0;
    let alpha: f32 = 1.0_f32 + (beta / 225.0) * color_float.iter().map(|(_, x)| *x).sum::<f32>();
    let theta = *colors.iter().min().unwrap() as f32 / 2.0;

    let j: Vec<(usize, f32)> = color_float
        .iter()
        .map(|(i, x)| (*i, f32::powf(*x - theta, alpha)))
        .collect();

    let denom = j.iter().map(|(_, x)| *x).sum::<f32>();

    for (i, x) in j {
        ans[i] = (1.0 - x / denom) / (color_float.len() - 1) as f32;
    }

    ans
}

fn pick_index(probabilities: &[f32]) -> i8 {
    // Step 1: Generate a random number between 0 and 1
    let mut rng = rand::thread_rng();
    let random_value: f32 = rng.gen(); // Generates a float between 0 and 1

    // Step 2: Create the cumulative distribution
    let mut cumulative_sum = 0.0;
    for (i, &prob) in probabilities.iter().enumerate() {
        cumulative_sum += prob;
        // Step 3: Return the index where the random value falls
        if random_value < cumulative_sum {
            return i as i8;
        }
    }

    // Fallback, this should rarely happen if the probabilities sum to 1
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probs() {
        let colors: [u8; 5] = [45, 45, 45, 45, 45];

        let p = get_probs(&colors);

        assert_eq!(p, [0.2, 0.2, 0.2, 0.2, 0.2]);

        eprintln!("{:?}", p);
    }

    #[test]
    fn test_probs2() {
        let colors: [u8; 5] = [225, 0, 0, 0, 0];

        let p = get_probs(&colors);

        eprintln!("{:?}", p);

        assert_eq!(p, [1.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_probs6() {
        let colors: [u8; 5] = [90, 90, 0, 0, 0];

        let p = get_probs(&colors);

        eprintln!("{:?}", p);

        assert_eq!(p, [0.5, 0.5, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_probs3() {
        let colors: [u8; 5] = [60, 30, 30, 30, 75];

        let p = get_probs(&colors);

        assert_eq!(p.iter().sum::<f32>(), 1.0);

        eprintln!("{:?}", p);
    }

    #[test]
    fn test_probs4() {
        let colors: [u8; 5] = [6, 3, 3, 3, 8];

        let p = get_probs(&colors);

        assert_eq!(p.iter().sum::<f32>(), 1.0);

        eprintln!("{:?}", p);
    }

    #[test]
    fn test_probs5() {
        let colors: [u8; 5] = [42, 0, 0, 0, 0];

        let p = get_probs(&colors);

        eprintln!("{:?}", p);

        assert_eq!(p, [1.0, 0.0, 0.0, 0.0, 0.0]);
    }
}
