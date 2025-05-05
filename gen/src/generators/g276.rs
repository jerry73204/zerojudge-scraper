use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{collections::HashSet, marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,  // Board height
    pub m_range: RangeInclusive<usize>,  // Board width
    pub k_range: RangeInclusive<usize>,  // Number of demon kings
    pub move_range: RangeInclusive<i32>, // Range for movement values
    pub one_dim: Option<bool>,           // If true, make it a 1-dimensional problem (n=1)
}

pub struct Sampler {
    _private: PhantomData<()>,
}

impl TestSampler for Sampler {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            _private: PhantomData,
        }
    }

    fn sample(&self, config: &serde_json::Value) -> crate::sampler::TestCase {
        let Config {
            n_range,
            m_range,
            k_range,
            move_range,
            one_dim,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() >= 1);
        assert!(*m_range.start() >= 1);
        assert!(*k_range.start() >= 1);

        let mut rng = rand::rng();

        // Determine board dimensions
        let n = if one_dim.unwrap_or(false) {
            1
        } else {
            rng.random_range(n_range)
        };

        let m = rng.random_range(m_range);
        let k = rng.random_range(k_range);

        // Generate unique demon kings
        let mut demon_kings = Vec::new();
        let mut positions = HashSet::new();

        for _ in 0..k {
            // Generate a unique starting position
            let mut r = rng.random_range(0..n as i32);
            let mut c = rng.random_range(0..m as i32);

            while positions.contains(&(r, c)) {
                r = rng.random_range(0..n as i32);
                c = rng.random_range(0..m as i32);
            }

            positions.insert((r, c));

            // Generate movement values
            let s = if one_dim.unwrap_or(false) {
                0 // For 1D, no vertical movement
            } else {
                rng.random_range(move_range.clone())
            };

            let t = rng.random_range(move_range.clone());

            // Make sure at least one of s or t is non-zero
            if s == 0 && t == 0 {
                if one_dim.unwrap_or(false) {
                    // For 1D case, only modify t
                    if rng.random_range(0..=1) == 0 {
                        let t_values = vec![-1, 1];
                        let t_idx = rng.random_range(0..t_values.len());
                        demon_kings.push((r, c, s, t_values[t_idx]));
                    } else {
                        // Use larger values for more interesting movement
                        let t_values = vec![-2, -1, 1, 2];
                        let t_idx = rng.random_range(0..t_values.len());
                        demon_kings.push((r, c, s, t_values[t_idx]));
                    }
                } else {
                    // For 2D case, modify either s or t
                    if rng.random_range(0..=1) == 0 {
                        let s_values = vec![-1, 1];
                        let s_idx = rng.random_range(0..s_values.len());
                        demon_kings.push((r, c, s_values[s_idx], t));
                    } else {
                        let t_values = vec![-1, 1];
                        let t_idx = rng.random_range(0..t_values.len());
                        demon_kings.push((r, c, s, t_values[t_idx]));
                    }
                }
            } else {
                demon_kings.push((r, c, s, t));
            }
        }

        // Count remaining bombs
        let bomb_count = count_remaining_bombs(n as i32, m as i32, &demon_kings);

        // Format input and output
        let input = format!(
            "{} {} {}\n{}",
            n,
            m,
            demon_kings.len(),
            demon_kings
                .iter()
                .map(|&(r, c, s, t)| format!("{} {} {} {}", r, c, s, t))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let output = format!("{}", bomb_count);

        TestCase { input, output }
    }
}

// Count the number of remaining bombs when all demon kings are gone
fn count_remaining_bombs(n: i32, m: i32, demon_kings: &[(i32, i32, i32, i32)]) -> usize {
    // Track positions of each demon king and bombs
    let mut kings_positions = Vec::new();
    let mut bombs = HashSet::new();
    let mut active_kings = vec![true; demon_kings.len()];

    // Initialize kings at their starting positions
    for &(r, c, _, _) in demon_kings {
        kings_positions.push((r, c));
    }

    // Simulate until all kings are gone
    loop {
        let mut active_count = 0;

        // Place bombs at current positions
        for i in 0..demon_kings.len() {
            if active_kings[i] {
                bombs.insert(kings_positions[i]);
            }
        }

        // Move kings
        for i in 0..demon_kings.len() {
            if active_kings[i] {
                let (r, c) = kings_positions[i];
                let (_, _, s, t) = demon_kings[i];

                let new_r = r + s;
                let new_c = c + t;

                // Check if king moved out of bounds
                if new_r < 0 || new_r >= n || new_c < 0 || new_c >= m {
                    active_kings[i] = false;
                } else {
                    kings_positions[i] = (new_r, new_c);
                }
            }
        }

        // Check for explosions
        for i in 0..demon_kings.len() {
            if active_kings[i] && bombs.contains(&kings_positions[i]) {
                active_kings[i] = false;
            }
        }

        // Count active kings
        for &active in &active_kings {
            if active {
                active_count += 1;
            }
        }

        if active_count == 0 {
            break;
        }
    }

    bombs.len()
}
