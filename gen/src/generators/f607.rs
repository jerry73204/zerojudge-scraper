use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>, // Number of cuts
    pub l_range: RangeInclusive<i32>,   // Length of the stick
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
        let Config { n_range, l_range } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);
        assert!(*l_range.start() > 0);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range); // Number of cuts
        let l = rng.random_range(l_range); // Length of the stick

        // Generate cut positions (between 0 and L)
        let mut cut_positions = Vec::with_capacity(n);
        for _ in 0..n {
            cut_positions.push(rng.random_range(1..l)); // Avoid cutting at 0 or L
        }

        // Make sure positions are unique
        cut_positions.sort();
        cut_positions.dedup();

        // Generate order of cuts (1 to n)
        let mut cut_order: Vec<usize> = (1..=cut_positions.len()).collect();
        cut_order.shuffle(&mut rng);

        // Format input
        let mut input = format!("{} {}\n", cut_positions.len(), l);
        for (i, &pos) in cut_positions.iter().enumerate() {
            input.push_str(&format!("{} {}\n", pos, cut_order[i]));
        }

        // Calculate the cost
        // First sort cuts by order
        let mut cuts = Vec::with_capacity(cut_positions.len());
        for (i, &pos) in cut_positions.iter().enumerate() {
            cuts.push((pos, cut_order[i]));
        }
        cuts.sort_by_key(|&(_, order)| order);

        let mut segments = vec![(0, l)]; // Initial segment: [0, L]
        let mut total_cost = 0_i64;

        // Process each cut in order
        for &(position, _) in &cuts {
            // Find which segment contains this position
            let mut segment_idx = None;
            for (i, &(start, end)) in segments.iter().enumerate() {
                if position >= start && position <= end {
                    segment_idx = Some(i);
                    break;
                }
            }

            if let Some(idx) = segment_idx {
                let (start, end) = segments[idx];

                // Calculate the cost (length of the current segment)
                total_cost += (end - start) as i64;

                // Create two new segments
                segments[idx] = (start, position); // Update the current segment
                segments.push((position, end)); // Add a new segment
            }
        }

        // Format output
        let output = format!("{}", total_cost);

        TestCase { input, output }
    }
}
