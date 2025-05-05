use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,      // Number of fence posts
    pub height_range: RangeInclusive<usize>, // Range for fence heights
    pub broken_pct: f64,                     // Percentage of broken fences (0)
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
            height_range,
            broken_pct,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() >= 3); // Minimum of 3 fence posts
        assert!(*height_range.start() > 0);
        assert!(broken_pct >= 0.0 && broken_pct <= 1.0);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range); // Number of fence posts

        // Generate fence heights
        let mut heights = Vec::with_capacity(n);
        for _ in 0..n {
            heights.push(rng.random_range(height_range.clone()));
        }

        // Determine which fences are broken (will be set to 0)
        let mut broken_indices = Vec::new();

        // Create a list of potential fence indices to break
        let mut potential_indices: Vec<usize> = (0..n).collect();

        // Shuffle the indices
        potential_indices.shuffle(&mut rng);

        // Determine how many fences to break
        let num_broken = (n as f64 * broken_pct).floor() as usize;

        // Ensure non-adjacent broken fences
        for _ in 0..num_broken {
            if potential_indices.is_empty() {
                break;
            }

            let idx = potential_indices.pop().unwrap();
            broken_indices.push(idx);

            // Remove adjacent indices to ensure no adjacent broken fences
            potential_indices.retain(|&i| i != idx - 1 && i != idx + 1);
        }

        // Set broken fences to height 0
        for &idx in &broken_indices {
            heights[idx] = 0;
        }

        // Format input
        let mut input = format!("{}\n", n);

        let heights_str = heights
            .iter()
            .map(|&h| h.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        input.push_str(&format!("{}\n", heights_str));

        // Calculate the total cost to repair broken fences
        let mut total_cost = 0;

        for &broken_idx in &broken_indices {
            // Find nearest non-broken fence to the left
            let mut left_height = usize::MAX;
            for i in (0..broken_idx).rev() {
                if heights[i] > 0 {
                    left_height = heights[i];
                    break;
                }
            }

            // Find nearest non-broken fence to the right
            let mut right_height = usize::MAX;
            for i in broken_idx + 1..n {
                if heights[i] > 0 {
                    right_height = heights[i];
                    break;
                }
            }

            // If no fence found on one side, use the other side
            if left_height == usize::MAX && right_height != usize::MAX {
                left_height = right_height;
            } else if right_height == usize::MAX && left_height != usize::MAX {
                right_height = left_height;
            }

            // Calculate repair cost (min of left and right)
            let new_height = left_height.min(right_height);
            total_cost += new_height;
        }

        // Format output
        let output = format!("{}", total_cost);

        TestCase { input, output }
    }
}
