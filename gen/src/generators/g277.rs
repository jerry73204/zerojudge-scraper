use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{collections::HashSet, marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,   // Number of people
    pub value_range: RangeInclusive<i32>, // Range for values
    pub limited_range: Option<bool>,      // If true, values are limited to 1..n
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
            value_range,
            limited_range,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() >= 1);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range);

        // Generate n unique numbers
        let mut numbers = Vec::with_capacity(n);
        let mut used_values = HashSet::new();

        for _ in 0..n {
            let mut value = if limited_range.unwrap_or(false) {
                rng.random_range(1..=n as i32)
            } else {
                rng.random_range(value_range.clone())
            };

            // Ensure uniqueness
            while used_values.contains(&value) {
                value = if limited_range.unwrap_or(false) {
                    rng.random_range(1..=n as i32)
                } else {
                    rng.random_range(value_range.clone())
                };
            }

            used_values.insert(value);
            numbers.push(value);
        }

        // Calculate the lucky number
        let lucky_number = find_lucky_number(&numbers);

        // Format input and output
        let input = format!(
            "{}\n{}",
            n,
            numbers
                .iter()
                .map(|&val| val.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        let output = format!("{}", lucky_number);

        TestCase { input, output }
    }
}

// Find the lucky number in the array
fn find_lucky_number(arr: &[i32]) -> i32 {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l < r {
        // Find the position of the minimum element
        let mut min_pos = l;
        for i in (l + 1)..=r {
            if arr[i] < arr[min_pos] {
                min_pos = i;
            }
        }

        // If min_pos is at the beginning or end, move to the other side
        if min_pos == l {
            return arr[l];
        }

        if min_pos == r {
            return arr[r];
        }

        // Calculate sum of left and right partitions
        let mut left_sum = 0;
        for i in l..min_pos {
            left_sum += arr[i];
        }

        let mut right_sum = 0;
        for i in (min_pos + 1)..=r {
            right_sum += arr[i];
        }

        // Select the partition with higher sum
        if left_sum > right_sum {
            r = min_pos - 1;
        } else if left_sum < right_sum {
            l = min_pos + 1;
        } else {
            // If equal, select the right partition
            l = min_pos + 1;
        }
    }

    arr[l] // When l == r, there's only one element left
}
