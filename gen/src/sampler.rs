use crate::config::LevelList;
use rayon::prelude::*;
use std::ops::RangeInclusive;

pub trait TestSampler: Sync + Send {
    fn new() -> Self
    where
        Self: Sized;

    fn sample(&self, config: &serde_json::Value) -> TestCase;

    fn sample_many(
        &self,
        configs: &LevelList,
        difficulty_range: RangeInclusive<f32>,
        n: u32,
    ) -> Vec<TestCase> {
        if n == 1 {
            let testcase = self.sample(&configs.level.last().unwrap().config);
            return vec![testcase];
        }

        let samples: Vec<_> = (0..n)
            .into_par_iter()
            .map(|ix| {
                let lw = (n - ix - 1) as f32;
                let rw = ix as f32;
                (lw * difficulty_range.start() + rw * difficulty_range.end()) / (n - 1) as f32
            })
            .map(|difficulty| configs.select(difficulty).unwrap())
            .map(|config| self.sample(config))
            .collect();
        samples
    }
}

pub struct TestCase {
    pub input: String,
    pub output: String,
}
