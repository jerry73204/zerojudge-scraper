use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>, // Number of service points
    pub k_range: RangeInclusive<usize>, // Number of base stations
    pub position_range: RangeInclusive<i32>, // Max coordinate value
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
            k_range,
            position_range,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);
        assert!(*k_range.start() > 0);
        assert!(*k_range.start() < *n_range.end()); // K < N constraint

        let mut rng = rand::rng();
        let n = rng.random_range(n_range);
        // Ensure K < N
        let k = rng.random_range(*k_range.start()..std::cmp::min(*k_range.end(), n));

        // Generate N service points (may not be sorted or unique)
        let mut positions = Vec::with_capacity(n);
        for _ in 0..n {
            positions.push(rng.random_range(position_range.clone()));
        }

        // Generate input
        let input = format!(
            "{} {}\n{}",
            n,
            k,
            positions
                .iter()
                .map(|&p| p.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        // Calculate output (minimum diameter)
        // Sort positions
        positions.sort();

        // Binary search for the minimum diameter
        let mut left = 0;
        let mut right = positions[n - 1] - positions[0];
        let mut result = right;

        while left <= right {
            let mid = left + (right - left) / 2;

            if can_cover(&positions, n, k, mid) {
                result = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        let output = format!("{}", result);

        TestCase { input, output }
    }
}

// Check if it's possible to cover all points with at most K base stations with given diameter
fn can_cover(positions: &[i32], n: usize, k: usize, diameter: i32) -> bool {
    if k <= 0 {
        return false;
    }

    let mut stations_used = 1; // Start with 1 station
    let mut current_endpoint = positions[0] + diameter;

    for i in 1..n {
        if positions[i] > current_endpoint {
            stations_used += 1;
            if stations_used > k {
                return false;
            }
            current_endpoint = positions[i] + diameter;
        }
    }

    true
}
