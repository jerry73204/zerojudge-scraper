use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,        // Number of fruits
    pub coordinate_range: RangeInclusive<i32>, // Coordinate range for x and y
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
            coordinate_range,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);
        assert!(*coordinate_range.start() > 0);

        let mut rng = rand::rng();

        // For large test cases, limit the size to avoid excessive runtime
        let n_limit = 50000; // Limit for level 100 tests
        let n_orig = rng.random_range(n_range); // Original number from range
        let n = n_orig.min(n_limit); // Capped number of fruits

        // Generate fruit positions in 2D space efficiently
        let mut fruits = Vec::with_capacity(n);

        // Create a set of coordinates to avoid duplicates efficiently
        let mut coordinates = std::collections::HashSet::with_capacity(n);

        // Generate coordinates with reduced range for large n to avoid excessive searching
        let range_scale_factor = if n > 1000 { 10 } else { 1 };
        let scaled_range = 1..=(coordinate_range.end() / range_scale_factor);

        while fruits.len() < n {
            let x = rng.random_range(scaled_range.clone());
            let y = rng.random_range(scaled_range.clone());
            let coord = (x, y);

            if coordinates.insert(coord) {
                fruits.push(coord);
            }
        }

        // Sort by x-coordinate
        fruits.sort_by_key(|&(x, _)| x);

        // Format input
        let mut input = format!("{}\n", fruits.len());
        for &(x, y) in &fruits {
            input.push_str(&format!("{} {}\n", x, y));
        }

        // Calculate the maximum number of fruits that can be eaten
        // Use a more efficient O(n log n) algorithm for Longest Increasing Subsequence
        let max_fruits = longest_increasing_subsequence_optimized(&fruits);

        // Format output
        let output = format!("{}", max_fruits);

        TestCase { input, output }
    }
}

// Efficient O(n log n) algorithm for Longest Increasing Subsequence
fn longest_increasing_subsequence_optimized(fruits: &[(i32, i32)]) -> usize {
    if fruits.is_empty() {
        return 0;
    }

    // Use patience sort algorithm
    let mut tails: Vec<i32> = Vec::new();

    for &(_, y) in fruits {
        match tails.binary_search(&y) {
            Ok(_) => {} // Element already exists
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(y);
                } else {
                    tails[pos] = y;
                }
            }
        }
    }

    tails.len()
}
