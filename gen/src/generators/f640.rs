use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub max_depth: RangeInclusive<usize>, // Maximum nesting depth of functions
    pub max_width: RangeInclusive<usize>, // Maximum number of function calls
    pub value_range: RangeInclusive<i32>, // Range of parameter values
    pub f_only: Option<bool>,             // If true, only use function f
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
            max_depth,
            max_width,
            value_range,
            f_only,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*max_depth.start() >= 1);
        assert!(*max_width.start() >= 1);

        let mut rng = rand::rng();
        let max_depth = rng.random_range(max_depth);
        let max_width = rng.random_range(max_width);

        // Generate the function expression
        let (expression, value) = generate_expression(
            max_depth,
            max_width,
            &value_range,
            f_only.unwrap_or(false),
            &mut rng,
        );

        // Format input and output
        let input = expression;
        let output = format!("{}", value);

        TestCase { input, output }
    }
}

// Function definitions
fn func_f(x: i64) -> i64 {
    2 * x - 3
}

fn func_g(x: i64, y: i64) -> i64 {
    2 * x + y - 7
}

fn func_h(x: i64, y: i64, z: i64) -> i64 {
    3 * x - 2 * y + z
}

// Generate a random function expression with its value
fn generate_expression(
    depth: usize,
    width: usize,
    value_range: &RangeInclusive<i32>,
    f_only: bool,
    rng: &mut impl Rng,
) -> (String, i64) {
    if depth <= 1 || width <= 1 {
        // Base case: just return a value
        let value = rng.random_range(value_range.clone()) as i64;
        return (value.to_string(), value);
    }

    // Choose which function to use
    let func_type = if f_only {
        0 // Only use function f
    } else {
        rng.random_range(0..3) // Randomly choose between f, g, h
    };

    match func_type {
        0 => {
            // Function f: f(x) = 2x - 3
            let (x_expr, x_val) =
                generate_expression(depth - 1, width.saturating_sub(1), value_range, f_only, rng);
            let result = func_f(x_val);
            let expr = format!("f {}", x_expr);
            (expr, result)
        }
        1 => {
            // Function g: g(x, y) = 2x + y - 7
            let (x_expr, x_val) = generate_expression(
                depth - 1,
                width.saturating_sub(2) / 2,
                value_range,
                f_only,
                rng,
            );
            let (y_expr, y_val) = generate_expression(
                depth - 1,
                width.saturating_sub(2) / 2,
                value_range,
                f_only,
                rng,
            );
            let result = func_g(x_val, y_val);
            let expr = format!("g {} {}", x_expr, y_expr);
            (expr, result)
        }
        2 => {
            // Function h: h(x, y, z) = 3x - 2y + z
            let (x_expr, x_val) = generate_expression(
                depth - 1,
                width.saturating_sub(3) / 3,
                value_range,
                f_only,
                rng,
            );
            let (y_expr, y_val) = generate_expression(
                depth - 1,
                width.saturating_sub(3) / 3,
                value_range,
                f_only,
                rng,
            );
            let (z_expr, z_val) = generate_expression(
                depth - 1,
                width.saturating_sub(3) / 3,
                value_range,
                f_only,
                rng,
            );
            let result = func_h(x_val, y_val, z_val);
            let expr = format!("h {} {} {}", x_expr, y_expr, z_expr);
            (expr, result)
        }
        _ => unreachable!(),
    }
}
