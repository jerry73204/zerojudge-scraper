use ordered_float::NotNan;
use std::ops::RangeInclusive;

macro_rules! notnan {
    ($val:expr) => {
        NotNan::new($val).unwrap()
    };
}

pub trait TestSampler {
    fn new() -> Self
    where
        Self: Sized;

    fn sample(&mut self, config: &serde_json::Value) -> TestCase;

    fn sample_many(
        &mut self,
        configs: &[(f32, serde_json::Value)],
        difficulty_range: RangeInclusive<f32>,
        n: u32,
    ) -> Vec<TestCase> {
        let samples: Vec<_> = (0..n)
            .map(|ix| {
                let lw = (n - ix - 1) as f32;
                let rw = ix as f32;
                (lw * difficulty_range.start() + rw * difficulty_range.end()) / (n - 1) as f32
            })
            .map(|difficulty| {
                let result =
                    configs.binary_search_by_key(&notnan!(difficulty), |(key, _)| notnan!(*key));
                let ix = match result {
                    Ok(ix) => ix,
                    Err(ix) => ix,
                };
                let (_, config) = &configs[ix];
                config
            })
            .map(|config| self.sample(config))
            .collect();
        samples
    }
}

pub struct TestCase {
    pub input: String,
    pub output: String,
}
