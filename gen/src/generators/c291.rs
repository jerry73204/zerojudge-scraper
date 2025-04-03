use crate::sampler::{TestCase, TestSampler};
use itertools::Itertools;
use rand::prelude::*;
use serde::Deserialize;
use std::{cmp::min, marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,
    pub max_group_size: Option<usize>,
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
            max_group_size,
        } = serde_json::from_value(config.clone()).unwrap();
        assert!(*n_range.start() > 0);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range);

        let nums = if let Some(max_group_size) = max_group_size {
            let group_sizes = {
                let mut remain = n;
                let mut group_sizes = vec![];
                while remain > 0 {
                    let group_size = rng.random_range(1..=min(remain, max_group_size));
                    group_sizes.push(group_size);
                    remain -= group_size;
                }
                group_sizes
            };

            let nums = {
                let mut nums: Vec<_> = (0..n).collect();
                nums.shuffle(&mut rng);
                nums
            };

            let mut slice = nums.as_slice();
            let mut shuffled = vec![0; n];

            for size in group_sizes {
                let (taken, remain) = slice.split_at(size);
                slice = remain;

                taken
                    .iter()
                    .copied()
                    .circular_tuple_windows()
                    .take(taken.len())
                    .for_each(|(prev, next)| {
                        shuffled[prev] = next;
                    });
            }

            shuffled
        } else {
            let mut nums: Vec<_> = (0..n).collect();
            nums.shuffle(&mut rng);
            nums
        };

        let num_groups: usize = {
            let mut visited = vec![false; n];
            let mut count = 0;

            for &start in &nums {
                if visited[start] {
                    continue;
                }
                visited[start] = true;
                count += 1;

                let mut prev = start;
                loop {
                    let next = nums[prev];
                    if visited[next] {
                        break;
                    }
                    visited[next] = true;
                    prev = next;
                }
            }

            count
        };

        let input = format!(
            "\
{n}
{}
",
            nums.iter().join(" ")
        );
        let output = format!("{num_groups}");

        TestCase { input, output }
    }
}
