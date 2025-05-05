use crate::sampler::{TestCase, TestSampler};
use crate::utils::random_tree::random_tree;
use itertools::Itertools;
use rand::{Rng, seq::SliceRandom};
use serde::Deserialize;
use std::{
    cmp::{max, min},
    collections::HashMap,
    marker::PhantomData,
    ops::RangeInclusive,
};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,
    pub max_degree: Option<usize>,
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
            n_range,
            max_degree: _,
        } = serde_json::from_value(config.clone()).unwrap();
        assert!(*n_range.start() >= 2);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range);
        let mut edges = random_tree(&mut rng, n);
        edges.shuffle(&mut rng);

        let diameter = find_diameter(&edges);

        let edges_display = edges
            .iter()
            .format_with("\n", |(from, to), f| f(&format_args!("{from} {to}")));

        let input = format!(
            "\
{n}
{edges_display}
"
        );
        let output = format!("{diameter}\n");
        TestCase { input, output }
    }
}

fn find_diameter(edges: &[(usize, usize)]) -> usize {
    let n = edges.len() + 1;
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n);

    for &(from, to) in edges {
        adj.entry(from).or_default().push(to);
        adj.entry(to).or_default().push(from);
    }

    let mut visited = vec![false; n];
    let mut diameter = 0;
    visit(0, &adj, &mut visited, &mut diameter);
    diameter
}

fn visit(
    curr: usize,
    adj: &HashMap<usize, Vec<usize>>,
    visited: &mut [bool],
    diameter: &mut usize,
) -> usize {
    visited[curr] = true;

    let heights: Vec<_> = adj[&curr]
        .iter()
        .filter_map(|&next| {
            if visited[next] {
                return None;
            }
            let height = visit(next, adj, visited, diameter);
            Some(height + 1)
        })
        .collect();

    let [max1, max2] = {
        let mut iter = heights.iter().copied();
        let Some(first) = iter.next() else {
            return 0;
        };
        let Some(second) = iter.next() else {
            *diameter = max(*diameter, first);
            return first;
        };

        let mut max1 = max(first, second);
        let mut max2 = min(first, second);

        for val in iter {
            if val > max1 {
                max2 = max1;
                max1 = val;
            } else if val > max2 {
                max2 = val;
            }
        }

        [max1, max2]
    };

    *diameter = max(*diameter, max1 + max2);
    max1
}
