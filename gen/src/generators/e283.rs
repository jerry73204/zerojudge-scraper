use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub string_count: RangeInclusive<usize>, // Number of strings
    pub max_string_length: RangeInclusive<usize>, // Maximum length of each string
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
            string_count,
            max_string_length,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*string_count.start() > 0);
        assert!(*max_string_length.start() > 0);

        let mut rng = rand::rng();
        let num_strings = rng.random_range(string_count);

        // Valid characters are A through F
        let char_set = vec!['A', 'B', 'C', 'D', 'E', 'F'];
        // Binary encoding for each character
        let binary_codes = vec![
            "0 1 0 1", // A
            "0 1 1 1", // B
            "0 0 1 0", // C
            "1 1 0 1", // D
            "1 0 0 0", // E
            "1 1 0 0", // F
        ];

        let mut input = String::new();
        let mut output = String::new();

        for i in 0..num_strings {
            // Generate a random string length
            let string_length = rng.random_range(max_string_length.clone());

            input.push_str(&format!("{}\n", string_length));

            // Generate a random string of characters from A-F
            let mut current_string = String::new();
            for _ in 0..string_length {
                let char_index = rng.random_range(0..char_set.len());
                let ch = char_set[char_index];
                current_string.push(ch);

                // Add binary encoding for this character
                input.push_str(binary_codes[char_index]);
                input.push('\n');
            }

            // Add the string to the output
            output.push_str(&current_string);

            // Add newline between strings (except for the last one)
            if i < num_strings - 1 {
                output.push('\n');
            }
        }

        TestCase { input, output }
    }
}
