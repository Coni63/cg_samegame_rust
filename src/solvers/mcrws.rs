// https://liacs.leidenuniv.nl/~takesfw/pdf/samegame.pdf

use rand::Rng;

use crate::board::Board;
use crate::solvers::solution::Solution;

pub fn _solve(initial_state: &Board) -> String {
    let mut board = initial_state.clone();
    let mut actions: Vec<String> = Vec::new();
    let K = 1000;

    while !board.is_over() {
        let mut highest_average_score = 0;
        let mut local_best_board = board.clone();
        let mut local_best_action = String::new();

        for region in board.compute_all_regions() {
            let mut copy = board.clone();
            copy.play_region(&region);

            let mut average_score = 0;
            for _ in 0..K {
                average_score += rollout(&copy)
            }

            if average_score > highest_average_score {
                highest_average_score = average_score;
                local_best_board = copy.clone();
                let idx = region.first().unwrap();
                let (x, y) = Board::to_coordinates(idx);
                local_best_action = format!("{} {}", x, y);
            }
        }

        board = local_best_board;
        actions.push(local_best_action);
    }

    itertools::join(actions, ";")
}

fn rollout(board: &Board) -> u32 {
    let mut copy = board.clone();
    let mut rng = rand::thread_rng();

    while !copy.is_over() {
        let p = get_probs(copy.get_color_count());
        let all_regions = copy.compute_all_regions();
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

    // Step 1: Find indices of positive values
    let mut positive_count = 0;
    let mut positive_index = None;

    for (i, &value) in colors.iter().enumerate() {
        if value > 0 {
            positive_count += 1;
            if positive_count > 1 {
                break; // No need to continue once we know there are more than one positive values
            }
            positive_index = Some(i);
        }
    }

    // Step 2: If exactly one positive value is found, set its probability to 1.0
    if positive_count == 1 {
        ans[positive_index.unwrap()] = 1.0;
        return ans;
    }

    let color_float: Vec<f32> = colors.iter().map(|&c| c as f32).collect();
    let beta: f32 = 4.0;
    let alpha: f32 = 1.0_f32 + (beta / 225.0) * color_float.iter().sum::<f32>();
    let theta = *colors.iter().min().unwrap() as f32 / 2.0;

    let j: Vec<f32> = colors
        .iter()
        .map(|x| f32::powf(*x as f32 - theta, alpha))
        .collect();

    let denom = j.iter().sum::<f32>();

    for i in 0..5 {
        ans[i] = 0.25 * (1.0 - j[i] / denom);
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

        assert_eq!(p.iter().sum::<f32>(), 1.0);

        eprintln!("{:?}", p);
    }

    #[test]
    fn test_probs2() {
        let colors: [u8; 5] = [225, 0, 0, 0, 0];

        let p = get_probs(&colors);

        eprintln!("{:?}", p);
        assert_eq!(p.iter().sum::<f32>(), 1.0);
        assert_eq!(p[0], 1.0);
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

        assert_eq!(p.iter().sum::<f32>(), 1.0);
        assert_eq!(p[0], 1.0);
    }
}
