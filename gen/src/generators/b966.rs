use crate::sampler::{TestCase, TestSampler};
use itertools::Itertools;
use rand::{Rng, rngs::ThreadRng};
use serde::Deserialize;
use std::{cmp::Ordering, collections::HashSet, marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<u32>,
    pub segment_range: RangeInclusive<i32>,
    pub overlap: bool,
}

pub struct Sampler {
    _private: PhantomData<()>,
}

impl TestSampler for Sampler {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn sample(&self, config: &serde_json::Value) -> TestCase {
        let Config {
            n_range,
            segment_range,
            overlap,
        } = serde_json::from_value(config.clone()).unwrap();

        let n_start = *n_range.start();
        let n_end = *n_range.end();
        assert!(n_start >= 1);

        let n_max = (segment_range.end() - segment_range.start()) as u32;
        assert!(n_end <= n_max);

        let mut rng = rand::rng();
        let n_seg = rng.random_range(n_range);

        let random_segment = |rng: &mut ThreadRng, range: RangeInclusive<i32>| {
            loop {
                let a = rng.random_range(range.clone());
                let b = rng.random_range(range.clone());

                break match a.cmp(&b) {
                    Ordering::Less => [a, b],
                    Ordering::Equal => continue,
                    Ordering::Greater => [b, a],
                };
            }
        };

        let segments: Vec<_> = if overlap {
            (0..n_seg)
                .map(|_| random_segment(&mut rng, segment_range.clone()))
                .collect()
        } else {
            let mut ends: HashSet<i32> = HashSet::new();

            for _ in 0..n_seg {
                loop {
                    let end = rng.random_range(segment_range.clone());
                    if !ends.contains(&end) {
                        ends.insert(end);
                        break;
                    }
                }
            }

            let mut bound_ends: Vec<_> = ends.into_iter().collect();
            bound_ends.sort_unstable();

            let mut bound_start = *segment_range.start();

            let segments: Vec<_> = bound_ends
                .into_iter()
                .map(|bound_end| {
                    let segment = random_segment(&mut rng, bound_start..=bound_end);
                    bound_start = bound_end;
                    segment
                })
                .collect();

            segments
        };

        let len = {
            let mut edges: Vec<_> = segments
                .iter()
                .flat_map(|&[a, b]| [(a, 1), (b, -1)])
                .collect();
            edges.sort_unstable();

            let mut len = 0;
            let mut iter = edges.into_iter();
            let (mut prev, mut count) = iter.next().unwrap();

            for (curr, inc) in iter {
                if count > 0 {
                    len += curr - prev;
                }
                count += inc;
                prev = curr;
            }

            len
        };

        let input = format!(
            "\
{n_seg}	    
{}",
            segments.iter().map(|[a, b]| format!("{a} {b}")).join("\n")
        );
        let output = format!("{len}\n");

        TestCase { input, output }
    }
}
