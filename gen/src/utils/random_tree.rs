use crate::utils::range_set::RangeSet;
use rand::Rng;
use std::collections::{HashMap, VecDeque};

pub fn random_tree(mut rng: impl Rng, n: usize) -> Vec<(usize, usize)> {
    match n {
        0 => panic!("tree size must be nonzero"),
        1 => return vec![],
        _ => {}
    }

    let prüfer_seq: VecDeque<_> = (0..(n - 2)).map(|_| rng.random_range(0..n)).collect();
    let mut counts: HashMap<_, u32> = HashMap::with_capacity(n - 2);

    for &val in &prüfer_seq {
        *counts.entry(val).or_default() += 1;
    }

    // eprintln!("INIT");
    let mut range_set = RangeSet::default();
    for &val in &prüfer_seq {
        range_set.insert(val);
    }

    let mut edges: Vec<_> = prüfer_seq
        .iter()
        .map(|&to| {
            let count = counts.get_mut(&to).unwrap();
            *count -= 1;

            let from = match range_set.first() {
                Some(first) => {
                    if first.start == 0 {
                        first.end
                    } else {
                        0
                    }
                }
                None => 0,
            };

            range_set.insert(from);

            if *count == 0 {
                range_set.remove(to);
            }

            (from, to)
        })
        .collect();

    let ranges: Vec<_> = range_set.iter().collect();

    let last_edge = match ranges.as_slice() {
        [] => {
            assert_eq!(n, 2);
            (0, 1)
        }
        [r1] => match r1.start {
            0 => {
                assert_eq!(r1.end, n - 2);
                (n - 2, n - 1)
            }
            1 => {
                assert_eq!(r1.end, n - 1);
                (0, n - 1)
            }
            _ => unreachable!(),
        },
        [r1, r2] => {
            assert_eq!(r1.start, 0);
            assert_eq!(r1.end + 1, r2.start);
            assert_eq!(r2.end, n - 1);
            (r1.end, n - 1)
        }
        _ => unreachable!(),
    };
    edges.push(last_edge);

    edges
}

#[cfg(test)]
mod tests {
    use super::random_tree;
    use rand::Rng;
    use rayon::{iter::repeat, prelude::*};
    use std::collections::HashMap;

    #[test]
    fn random_tree_test() {
        const N1: usize = 100;
        const N2: usize = 10_000;
        const REPEAT1: usize = 100;
        const REPEAT2: usize = 100;

        (1..=N1)
            .into_par_iter()
            .flat_map(|n| repeat(n).take(REPEAT1))
            .for_each(|n| {
                let mut rng = rand::rng();
                let edges = random_tree(&mut rng, n);
                check_tree(n, &edges);
            });

        repeat(()).take(REPEAT2).for_each(|_| {
            let mut rng = rand::rng();
            let n = rng.random_range((N1 + 1)..=N2);
            let edges = random_tree(&mut rng, n);
            check_tree(n, &edges);
        });
    }

    /// Check if the input edge set forms a tree. It forms a tree iff
    /// the no. of edges is n - 1 and is connected.
    fn check_tree(n: usize, edges: &[(usize, usize)]) {
        assert!(n > 0);
        assert_eq!(n - 1, edges.len());

        if n == 1 {
            return;
        }

        let adj = {
            let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
            edges
                .iter()
                .flat_map(|&(a, b)| [(a, b), (b, a)])
                .for_each(|(from, to)| {
                    assert!((0..n).contains(&from));
                    adj.entry(from).or_default().push(to);
                });
            adj
        };

        let mut visited = vec![false; n];
        let mut n_visited = 0;
        let mut frontier = Vec::with_capacity(n - 1);
        frontier.push(0);

        while let Some(curr) = frontier.pop() {
            if visited[curr] {
                continue;
            }
            visited[curr] = true;
            n_visited += 1;

            frontier.extend(adj[&curr].iter().copied());
        }

        assert_eq!(n_visited, n);
    }
}
