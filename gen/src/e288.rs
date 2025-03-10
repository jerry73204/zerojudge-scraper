use crate::sampler::{TestCase, TestSampler};
use itertools::{Itertools, izip};
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};
use serde::Deserialize;
use std::{collections::HashMap, ops::RangeInclusive};

static ALL_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone, Deserialize)]
struct Config {
    m_range: RangeInclusive<u32>,
    n_range: RangeInclusive<u32>,
    strlen_range: RangeInclusive<u32>,
    repeat: bool,
}

pub struct Sampler {
    rng: ThreadRng,
}

impl TestSampler for Sampler {
    fn new() -> Self {
        Sampler { rng: rand::rng() }
    }

    fn sample(&mut self, config: &serde_json::Value) -> TestCase {
        let Self { rng } = self;
        let Config {
            m_range,
            n_range,
            strlen_range,
            repeat,
        } = serde_json::from_value(config.clone()).unwrap();

        let m = rng.random_range(m_range);
        let n = rng.random_range(n_range);
        let letters: Vec<char> = ALL_LETTERS
            .as_bytes()
            .choose_multiple(rng, m as usize)
            .map(|&b| b as char)
            .collect();

        let sets: Vec<String> = (0..n)
            .map(|_| {
                let len = rng.random_range(strlen_range.clone()) as usize;

                if repeat {
                    (0..len)
                        .map(|_| *letters.choose(rng).unwrap())
                        .take(len)
                        .collect()
                } else {
                    letters.choose_multiple(rng, len).copied().collect()
                }
            })
            .collect();

        let letter_map: HashMap<char, usize> = izip!(letters.iter().copied(), 0..).collect();
        let counts: HashMap<u64, u32> = sets
            .iter()
            .map(|set| {
                let bitset = set
                    .as_bytes()
                    .iter()
                    .map(|&ch| letter_map[&(ch as char)])
                    .fold(0u64, |bits, nth| bits | (1u64 << nth));
                (bitset, 1u32)
            })
            .into_grouping_map()
            .sum();

        let mask = (1u64 << m) - 1;
        let num_pairs = counts
            .iter()
            .map(|(&bitset, &count)| {
                let comp_bitset = !bitset & mask;
                let comp_count = counts.get(&comp_bitset).copied().unwrap_or(0);
                count * comp_count
            })
            .sum::<u32>()
            / 2;

        let input = format!("{m} {n}\n{}", sets.join("\n"));
        let output = num_pairs.to_string();

        TestCase { input, output }
    }
}
