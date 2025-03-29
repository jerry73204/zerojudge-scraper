use crate::sampler::{TestCase, TestSampler};
use itertools::Itertools;
use rand::Rng;
use serde::Deserialize;
use std::{borrow::Cow, marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<u32>,
    pub case: Case,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum Case {
    Best,
    Worst,
    Any,
}

pub struct Sampler {
    _private: PhantomData<()>,
}

impl TestSampler for Sampler {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn sample(&self, config: &serde_json::Value) -> TestCase {
        let Config { n_range, case } = serde_json::from_value(config.clone()).unwrap();
        assert!(*n_range.start() >= 1);

        let mut rng = rand::rng();
        let n_people = rng.random_range(n_range);

        let score_range = match case {
            Case::Best => 60..=100,
            Case::Worst => 0..=59,
            Case::Any => 0..=100,
        };

        let mut scores: Vec<u32> = (0..n_people)
            .into_iter()
            .map(|_| rng.random_range(score_range.clone()))
            .collect();

        let min_pass: Cow<str> = match scores.iter().filter(|&&sc| sc >= 60).min() {
            Some(sc) => sc.to_string().into(),
            None => "worst case".into(),
        };
        let max_fail: Cow<str> = match scores.iter().filter(|&&sc| sc < 60).max() {
            Some(sc) => sc.to_string().into(),
            None => "best case".into(),
        };

        let input = format!(
            "\
{n_people}
{}
",
            scores.iter().join(" ")
        );

        scores.sort_unstable();
        let output = format!(
            "\
{}
{max_fail}
{min_pass}
",
            scores.iter().join(" ")
        );

        TestCase { input, output }
    }
}
