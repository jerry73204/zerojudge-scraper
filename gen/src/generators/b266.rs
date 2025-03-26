use crate::sampler::{TestCase, TestSampler};
use itertools::Itertools;
use rand::prelude::*;
use serde::Deserialize;
use std::{
    fmt::{self, Display},
    marker::PhantomData,
    ops::RangeInclusive,
};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub row_range: RangeInclusive<usize>,
    pub column_range: RangeInclusive<usize>,
    pub op_range: RangeInclusive<usize>,
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
        let Config {
            row_range,
            column_range,
            op_range,
        } = serde_json::from_value(config.clone()).unwrap();

        let mut rng = rand::rng();
        let nr = rng.random_range(row_range);
        let nc = rng.random_range(column_range);
        let nk = rng.random_range(op_range);

        let ops: Vec<bool> = rng.random_iter().take(nk).collect();
        let start_matrix = Matrix::new(nr, nc);
        let end_matrix = {
            let mut matrix = start_matrix.clone();
            for &op in &ops {
                if op {
                    matrix.vertical_flip();
                } else {
                    matrix.rotate();
                }
            }
            matrix
        };

        let op_seq = ops.iter().map(|&op| if op { '1' } else { '0' }).join(" ");
        let input = format!(
            "\
{nr} {nc} {nk}
{end_matrix}{op_seq}
"
        );
        let output = format!(
            "\
{} {}
{start_matrix}",
            start_matrix.n_rows(),
            start_matrix.n_columns()
        );

        TestCase { input, output }
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    rows: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0 && cols > 0);
        let mut rng = rand::rng();

        // Create a matrix with random numbers between 0 and 9
        let rows: Vec<Vec<_>> = (0..rows)
            .map(|_| (0..cols).map(|_| rng.random_range(0..=9)).collect())
            .collect();

        Self { rows }
    }

    pub fn vertical_flip(&mut self) {
        self.rows = std::mem::take(&mut self.rows).into_iter().rev().collect();
    }

    pub fn rotate(&mut self) {
        let rows = self.rows.len();
        let cols = self.rows[0].len();

        // Create a new matrix with swapped dimensions
        let mut rotated = vec![vec![0; rows]; cols];

        for (r, row) in self.rows.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                rotated[c][rows - 1 - r] = val;
            }
        }

        self.rows = rotated;
    }

    pub fn n_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn n_columns(&self) -> usize {
        self.rows[0].len()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            write!(f, "{}", row[0])?;
            for &val in &row[1..] {
                write!(f, " {}", val)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
