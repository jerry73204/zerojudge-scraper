use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,         // Number of employees
    pub coefficient_range: RangeInclusive<i64>, // Range for coefficients A, B, C
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
            coefficient_range,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range); // Number of employees

        // Generate coefficients for factory 1
        let a1 = rng.random_range(coefficient_range.clone());
        let b1 = rng.random_range(coefficient_range.clone());
        let c1 = rng.random_range(coefficient_range.clone());

        // Generate coefficients for factory 2
        let a2 = rng.random_range(coefficient_range.clone());
        let b2 = rng.random_range(coefficient_range.clone());
        let c2 = rng.random_range(coefficient_range.clone());

        // Format input
        let mut input = format!("{} {} {}\n", a1, b1, c1);
        input.push_str(&format!("{} {} {}\n", a2, b2, c2));
        input.push_str(&format!("{}\n", n));

        // Calculate the maximum revenue
        // We need to find the optimal allocation that maximizes:
        // Y1 = a1*x1^2 + b1*x1 + c1 and Y2 = a2*x2^2 + b2*x2 + c2
        // where x1 + x2 = n

        // This forms a quadratic function in terms of x1, with x2 = n - x1
        // We don't actually need these coefficients for brute force solution
        let _k2 = a1 + a2;
        let _k1 = b1 - b2 - 2 * a2 * (n as i64);
        let _k0 = a2 * (n as i64) * (n as i64) + b2 * (n as i64) + c1 + c2;

        let mut max_revenue = i64::MIN;

        // Check all possible allocations
        for x1 in 0..=n {
            let x2 = n - x1;
            let y1 = a1 * (x1 as i64) * (x1 as i64) + b1 * (x1 as i64) + c1;
            let y2 = a2 * (x2 as i64) * (x2 as i64) + b2 * (x2 as i64) + c2;
            let total = y1 + y2;

            if total > max_revenue {
                max_revenue = total;
            }
        }

        // Format output
        let output = format!("{}", max_revenue);

        TestCase { input, output }
    }
}
