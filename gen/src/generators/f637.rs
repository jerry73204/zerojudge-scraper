use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>, // Image size (must be a power of 2)
    pub complexity: RangeInclusive<usize>, // Controls the complexity of the generated image (approx. number of 2s in the encoding)
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
            complexity,
        } = serde_json::from_value(config.clone()).unwrap();

        // n must be a power of 2
        assert!(*n_range.start() >= 1);
        assert!(n_range.start().is_power_of_two());
        assert!(n_range.end().is_power_of_two());

        let mut rng = rand::rng();

        // Choose n (image size) from range, ensuring it's a power of 2
        let n_power = rng.random_range(
            n_range.start().trailing_zeros() as usize..=n_range.end().trailing_zeros() as usize,
        );
        let n = 1 << n_power;

        // Determine complexity of the image (how many 2s in the encoding)
        let num_complex_nodes = rng.random_range(complexity);

        // Generate the DF-expression
        let df_expression = generate_df_expression(n, num_complex_nodes, &mut rng);

        // Count black pixels (1s) in the image
        let black_pixel_count = count_black_pixels(&df_expression, n);

        // Format input and output
        let input = format!("{}\n{}", df_expression, n);
        let output = format!("{}", black_pixel_count);

        TestCase { input, output }
    }
}

// Generate a valid DF-expression string
fn generate_df_expression(size: usize, complexity: usize, rng: &mut impl Rng) -> String {
    let mut expression = String::new();
    let mut remaining_complex = complexity;

    // Helper function to recursively build the expression
    fn build_expression(expr: &mut String, size: usize, remaining: &mut usize, rng: &mut impl Rng) {
        if size == 1 || *remaining == 0 {
            // Base case: uniform color for this quadrant
            expr.push(if rng.random_range(0..=2) == 0 {
                '1'
            } else {
                '0'
            });
            return;
        }

        // Decide if this node should be complex (has mixed colors)
        let make_complex = *remaining > 0 && rng.random_range(0..=3) > 0;

        if make_complex {
            // Mark as complex node (mixed colors)
            expr.push('2');
            *remaining -= 1;

            // Recursively generate the four quadrants
            for _ in 0..4 {
                build_expression(expr, size / 2, remaining, rng);
            }
        } else {
            // Uniform color for this quadrant
            expr.push(if rng.random_range(0..=2) == 0 {
                '1'
            } else {
                '0'
            });
        }
    }

    build_expression(&mut expression, size, &mut remaining_complex, rng);
    expression
}

// Count the number of black pixels (1s) represented by the DF-expression
fn count_black_pixels(s: &str, n: usize) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut index = 0;

    // Helper function to recursively parse the expression
    fn parse_expression(chars: &[char], index: &mut usize, size: usize) -> usize {
        let current = chars[*index];
        *index += 1;

        match current {
            '0' => 0,           // All white
            '1' => size * size, // All black
            '2' => {
                // Mixed, count each quadrant
                let new_size = size / 2;

                let mut count = 0;
                // Process all four quadrants
                for _ in 0..4 {
                    count += parse_expression(chars, index, new_size);
                }
                count
            }
            _ => panic!("Invalid character in DF-expression"),
        }
    }

    parse_expression(&chars, &mut index, n)
}
