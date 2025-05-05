use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,   // Size of the array
    pub k_range: RangeInclusive<usize>,   // Maximum cutting level
    pub value_range: RangeInclusive<i32>, // Range of array values
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
            value_range,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() >= 3); // Array needs at least 3 elements for cutting
        assert!(*k_range.start() >= 1); // At least 1 level of cutting

        let mut rng = rand::rng();
        let n = rng.random_range(n_range);
        // Ensure k is not larger than 29 (as specified in the problem)
        let max_k = std::cmp::min(*k_range.end(), 29);
        let k_range_limited = *k_range.start()..=max_k;
        let k = rng.random_range(k_range_limited);

        // Generate the array with positive integers
        let mut array = Vec::with_capacity(n);
        for _ in 0..n {
            array.push(rng.random_range(value_range.clone()));
        }

        // Format input
        let input = format!(
            "{} {}\n{}",
            n,
            k,
            array
                .iter()
                .map(|&val| val.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        // Calculate the output (sum of cut points)
        let cut_points = find_cut_points(&array, k);
        let output = format!("{}", cut_points.iter().sum::<i32>());

        TestCase { input, output }
    }
}

// Find the optimal cut points and return their values
fn find_cut_points(array: &[i32], max_level: usize) -> Vec<i32> {
    let mut cut_points = Vec::new();
    let mut segments = vec![(0, array.len() - 1)]; // Initial segment [0, n-1]

    // Process segments level by level
    for _level in 0..max_level {
        if segments.is_empty() {
            break;
        }

        let mut new_segments = Vec::new();

        // Process each segment at the current level
        for &(start, end) in &segments {
            // Skip segments that are too small for cutting
            if end - start + 1 < 3 {
                continue;
            }

            // Find the best cut point within this segment
            let cut_idx = find_best_cut_point(array, start, end);

            // Add the value at the cut point to our result
            cut_points.push(array[cut_idx]);

            // Split into two new segments
            new_segments.push((start, cut_idx - 1));
            new_segments.push((cut_idx + 1, end));
        }

        segments = new_segments;
    }

    cut_points
}

// Find the best cut point in the array segment [start, end]
fn find_best_cut_point(array: &[i32], start: usize, end: usize) -> usize {
    let mut best_idx = start + 1;
    let mut min_diff = i64::MAX;

    // Try each potential cut point (excluding endpoints)
    for m in (start + 1)..end {
        let mut left_sum: i64 = 0;
        let mut right_sum: i64 = 0;

        // Calculate left sum: sum(|array[i] * (m - i)|) for i from start to m-1
        for i in start..m {
            left_sum += (array[i] as i64) * ((m - i) as i64);
        }

        // Calculate right sum: sum(|array[i] * (i - m)|) for i from m+1 to end
        for i in (m + 1)..=end {
            right_sum += (array[i] as i64) * ((i - m) as i64);
        }

        // Calculate absolute difference
        let diff = (left_sum - right_sum).abs();

        // Update best cut point if this is better (or same but with smaller index)
        if diff < min_diff {
            min_diff = diff;
            best_idx = m;
        }
    }

    best_idx
}
