use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>, // Number of dice
    pub m_range: RangeInclusive<usize>, // Number of operations
    pub swap_ratio: f64,                // Probability of swap operations vs rotations
}

pub struct Sampler {
    _private: PhantomData<()>,
}

// Map for dice rotations
const FRONT_MAP: [usize; 6] = [2, 1, 5, 0, 4, 3]; // Rotate forward
const RIGHT_MAP: [usize; 6] = [4, 0, 2, 3, 5, 1]; // Rotate right

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
            swap_ratio,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);
        assert!(*m_range.start() > 0);
        assert!(swap_ratio >= 0.0 && swap_ratio <= 1.0);

        let mut rng = rand::rng();

        // Cap the values to reasonable ranges to prevent performance issues
        let n = rng.random_range(n_range).min(20); // Reasonable cap for number of dice
        let m = rng.random_range(m_range).min(100); // Reasonable cap for operations

        // Format input header
        let mut input = format!("{} {}\n", n, m);

        // Generate operations more efficiently
        let mut operations = Vec::with_capacity(m);

        for _ in 0..m {
            if rng.random_bool(swap_ratio) {
                // Swap operation: two different dice indices
                // More efficient approach - generate a pair of distinct indices
                let mut indices: Vec<usize> = (1..=n).collect();
                indices.shuffle(&mut rng);
                let a = indices[0];
                let b = indices[1];
                operations.push((a, b as i32));
            } else {
                // Rotation operation
                let a = rng.random_range(1..=n);
                let b = if rng.random_bool(0.5) { -1_i32 } else { -2_i32 }; // -1: forward rotation, -2: right rotation
                operations.push((a, b));
            }
        }

        // Add operations to input
        for (a, b) in &operations {
            input.push_str(&format!("{} {}\n", a, b));
        }

        // Calculate the final state of each die
        // Initially all dice show 1 (index 0) on top
        let mut dice = vec![0; n]; // 0-indexed

        // Apply operations
        for &(a, b) in &operations {
            let a_idx = a - 1; // Convert to 0-indexed

            if b > 0 {
                // Swap operation
                let b_idx = b as usize - 1;
                dice.swap(a_idx, b_idx);
            } else if b == -1 {
                // Forward rotation
                dice[a_idx] = FRONT_MAP[dice[a_idx]];
            } else if b == -2 {
                // Right rotation
                dice[a_idx] = RIGHT_MAP[dice[a_idx]];
            }
        }

        // Convert dice values to display value (add 1 to get 1-6 range)
        let output_values: Vec<String> = dice.iter().map(|&d| format!("{}", d + 1)).collect();

        // Format output
        let output = output_values.join(" ");

        TestCase { input, output }
    }
}
