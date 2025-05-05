use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>, // Number of viruses
    pub m_range: RangeInclusive<usize>, // Length of RNA sequence
    pub at_symbol_prob: f64,            // Probability of '@' in RNA sequence
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
            m_range,
            at_symbol_prob,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);
        assert!(*m_range.start() > 0);
        assert!(at_symbol_prob >= 0.0 && at_symbol_prob <= 1.0);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range.clone()); // Number of viruses
        let m = rng.random_range(m_range.clone()); // Length of RNA sequence

        // Format input header
        let mut input = format!("{} {}\n", n, m);

        // Generate RNA sequences for each virus
        let mut sequences = Vec::with_capacity(n);

        // Generate tree structure - pick parent for each virus
        let mut parents = vec![0; n];

        // Virus 1 is always the root
        parents[0] = 0;

        // Generate the evolution tree (for viruses 2 to n)
        for i in 1..n {
            // Pick a random parent from previously defined viruses
            parents[i] = rng.random_range(0..i);
        }

        // Create RNA sequences
        for i in 0..n {
            let parent_idx = parents[i];

            // Initialize a new sequence or base it on parent
            let sequence = if i == 0 || rng.random_bool(0.5) {
                // Create a new random sequence
                let mut seq = String::with_capacity(m);
                for _ in 0..m {
                    let nucleotide = if rng.random_bool(at_symbol_prob) {
                        '@' // Unknown nucleotide
                    } else {
                        // Random nucleotide: A, U, C, G
                        *['A', 'U', 'C', 'G'].choose(&mut rng).unwrap()
                    };
                    seq.push(nucleotide);
                }
                seq
            } else {
                // Base on parent with some mutations
                let parent_seq: &String = &sequences[parent_idx];
                let mut seq = parent_seq.clone();

                // Apply some mutations (0-3 mutations)
                let mutations = rng.random_range(0..=3);
                for _ in 0..mutations {
                    let pos = rng.random_range(0..m);
                    let nucleotide = if rng.random_bool(at_symbol_prob) {
                        '@' // Unknown nucleotide
                    } else {
                        // Random nucleotide: A, U, C, G
                        *['A', 'U', 'C', 'G'].choose(&mut rng).unwrap()
                    };
                    seq.replace_range(pos..pos + 1, &nucleotide.to_string());
                }
                seq
            };

            sequences.push(sequence);
        }

        // Add virus information to input
        for i in 0..n {
            let virus_id = i + 1;
            let parent_id = parents[i] + 1;
            input.push_str(&format!("{} {} {}\n", virus_id, parent_id, sequences[i]));
        }

        // Calculate minimum Hamming distance
        // This is a simplified version - a real solver would need to optimize @ placements
        let mut total_min_distance = 0;

        for i in 0..n {
            if i == parents[i] {
                continue; // Skip the root
            }

            let parent_seq = &sequences[parents[i]];
            let current_seq = &sequences[i];

            // Calculate minimum Hamming distance
            let mut min_distance = 0;
            for j in 0..m {
                let parent_char = parent_seq.chars().nth(j).unwrap();
                let current_char = current_seq.chars().nth(j).unwrap();

                if parent_char != '@' && current_char != '@' && parent_char != current_char {
                    min_distance += 1;
                }
                // If either is @, we can make them match by appropriate substitution
            }

            total_min_distance += min_distance;
        }

        // Format output
        let output = format!("{}", total_min_distance);

        TestCase { input, output }
    }
}
