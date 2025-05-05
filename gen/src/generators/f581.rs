use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,      // Number of rooms
    pub m_range: RangeInclusive<usize>,      // Number of tasks
    pub p_range: RangeInclusive<i32>,        // Range for points in each room
    pub q_range_factor: RangeInclusive<f64>, // Factor to determine q range based on total points
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
            p_range,
            q_range_factor,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);
        assert!(*m_range.start() > 0);
        assert!(*p_range.start() > 0);
        assert!(*q_range_factor.start() > 0.0 && *q_range_factor.end() <= 1.0);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range); // Number of rooms
        let m = rng.random_range(m_range); // Number of tasks

        // Generate points for each room
        let mut points = Vec::with_capacity(n);
        for _ in 0..n {
            points.push(rng.random_range(p_range.clone()));
        }

        // Calculate total points
        let total_points: i32 = points.iter().sum();

        // Format input header
        let mut input = format!("{} {}\n", n, m);

        // Add points to input
        let points_str = points
            .iter()
            .map(|&p| p.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        input.push_str(&points_str);
        input.push('\n');

        // Calculate prefix sums for simulation
        let mut prefix_sums = vec![0; n + 1];
        for i in 0..n {
            prefix_sums[i + 1] = prefix_sums[i] + points[i];
        }

        // Generate task requirements
        let mut tasks = Vec::with_capacity(m);
        for _ in 0..m {
            // Generate q based on total points and factor range
            let factor = rng.random_range(q_range_factor.clone());
            let q = (total_points as f64 * factor).round() as i32;
            if q <= 0 {
                tasks.push(1); // Ensure q is at least 1
            } else {
                tasks.push(q);
            }
        }

        // Add tasks to input
        let tasks_str = tasks
            .iter()
            .map(|&q| q.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        input.push_str(&tasks_str);
        input.push('\n');

        // Simulate the process
        let mut current_room = 0;

        for &q in &tasks {
            let target = (prefix_sums[current_room] + q) % total_points;

            // Find the room where we collect the necessary points
            let mut target_room = n;
            for i in 0..=n {
                if prefix_sums[i] > target {
                    target_room = i - 1;
                    break;
                }
            }

            // Move to the next room
            current_room = (target_room + 1) % n;
        }

        // Format output
        let output = format!("{}", current_room);

        TestCase { input, output }
    }
}
