use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub r_range: RangeInclusive<usize>,
    pub c_range: RangeInclusive<usize>,
    pub m_range: RangeInclusive<usize>,
    pub value_range: RangeInclusive<u8>,
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
            r_range,
            c_range,
            m_range,
            value_range,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*r_range.start() >= 1);
        assert!(*c_range.start() >= 1);
        assert!(*m_range.start() >= 1);
        assert!(*value_range.start() <= *value_range.end());
        assert!(*value_range.end() <= 9);

        let mut rng = rand::rng();
        let r = rng.random_range(r_range);
        let c = rng.random_range(c_range);
        let m = rng.random_range(m_range);

        // Generate the original Matrix A
        let mut matrix_a = vec![vec![0; c]; r];
        for i in 0..r {
            for j in 0..c {
                matrix_a[i][j] = rng.random_range(value_range.clone());
            }
        }

        // Generate M random operations (0 for rotate, 1 for flip)
        let mut operations = Vec::new();
        for _ in 0..m {
            operations.push(rng.random_range(0..=1));
        }

        // Apply operations to get Matrix B
        let mut matrix_b = matrix_a.clone();
        for &op in &operations {
            if op == 0 {
                // Rotate clockwise
                let mut new_matrix = vec![vec![0; matrix_b.len()]; matrix_b[0].len()];
                for i in 0..matrix_b.len() {
                    for j in 0..matrix_b[0].len() {
                        new_matrix[j][matrix_b.len() - 1 - i] = matrix_b[i][j];
                    }
                }
                matrix_b = new_matrix;
            } else {
                // Flip
                let rows = matrix_b.len();
                for i in 0..rows / 2 {
                    matrix_b.swap(i, rows - 1 - i);
                }
            }
        }

        // Format the input
        let mut input = format!("{} {} {}\n", matrix_b.len(), matrix_b[0].len(), m);
        for i in 0..matrix_b.len() {
            for j in 0..matrix_b[0].len() {
                input.push_str(&format!("{}", matrix_b[i][j]));
                if j < matrix_b[0].len() - 1 {
                    input.push(' ');
                }
            }
            input.push('\n');
        }

        // Add operations (in reverse order)
        let ops_str = operations
            .iter()
            .map(|&op| op.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        input.push_str(&ops_str);

        // Format the output (original matrix A)
        let mut output = format!("{} {}\n", matrix_a.len(), matrix_a[0].len());
        for i in 0..matrix_a.len() {
            for j in 0..matrix_a[0].len() {
                output.push_str(&format!("{}", matrix_a[i][j]));
                if j < matrix_a[0].len() - 1 {
                    output.push(' ');
                }
            }
            if i < matrix_a.len() - 1 {
                output.push('\n');
            }
        }

        TestCase { input, output }
    }
}
