use crate::sampler::{TestCase, TestSampler};
use itertools::Itertools;
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub len_range: RangeInclusive<usize>,
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

    fn sample(&self, config: &serde_json::Value) -> TestCase {
        let Config { len_range } = serde_json::from_value(config.clone()).unwrap();
        assert!(*len_range.start() > 0);

        let mut rng = rand::rng();
        let len = rng.random_range(len_range);

        let digits: String = (0..len).map(|_| rng.random_range('0'..='9')).collect();

        let diff = digits
            .as_bytes()
            .iter()
            .map(|&d| (d - b'0') as i32)
            .chunks(2)
            .into_iter()
            .map(|mut chunk| {
                let d1 = chunk.next().unwrap();
                let d2 = chunk.next().unwrap_or(0);
                d1 - d2
            })
            .sum::<i32>()
            .abs();

        let input = format!("{digits}\n");
        let output = format!("{diff}\n");
        TestCase { input, output }
    }
}
