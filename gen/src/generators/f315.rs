use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>, // n value (number of different integers)
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
        let Config { n_range } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range);

        // Create an array where each number from 1 to n appears exactly twice
        let mut arr = Vec::with_capacity(2 * n);
        for i in 1..=n {
            arr.push(i);
            arr.push(i);
        }

        // Shuffle the array
        arr.shuffle(&mut rng);

        // Calculate depression values and their sum
        let mut depression_values_sum = 0;
        let mut first_occurrence = vec![-1; n + 1];

        for (i, &num) in arr.iter().enumerate() {
            if first_occurrence[num] == -1 {
                // First occurrence
                first_occurrence[num] = i as i32;
            } else {
                // Second occurrence, calculate depression value
                let mut smaller_count = 0;
                for j in (first_occurrence[num] as usize + 1)..i {
                    if arr[j] < num {
                        smaller_count += 1;
                    }
                }
                depression_values_sum += smaller_count;
            }
        }

        // Format input and output
        let input = format!(
            "{}\n{}",
            n,
            arr.iter()
                .map(|&val| val.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        let output = format!("{}", depression_values_sum);

        TestCase { input, output }
    }
}
