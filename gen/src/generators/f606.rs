use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,     // Number of servers
    pub m_range: RangeInclusive<usize>,     // Number of cities
    pub k_range: RangeInclusive<usize>,     // Number of placement strategies
    pub traffic_range: RangeInclusive<u32>, // Traffic range
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
            k_range,
            traffic_range,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);
        assert!(*m_range.start() > 0);
        assert!(*k_range.start() > 0);
        assert!(*traffic_range.start() > 0);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range); // Number of servers
        let m = rng.random_range(m_range); // Number of cities
        let k = rng.random_range(k_range); // Number of placement strategies

        // Generate traffic data for each server to each city
        let mut traffic_data = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                traffic_data[i][j] = rng.random_range(traffic_range.clone());
            }
        }

        // Generate placement strategies
        let mut placements = vec![vec![0; n]; k];
        for i in 0..k {
            for j in 0..n {
                placements[i][j] = rng.random_range(0..m);
            }
        }

        // Format input
        let mut input = format!("{} {} {}\n", n, m, k);

        // Add traffic data
        for i in 0..n {
            for j in 0..m {
                input.push_str(&traffic_data[i][j].to_string());
                if j < m - 1 {
                    input.push(' ');
                }
            }
            input.push('\n');
        }

        // Add placement strategies
        for i in 0..k {
            for j in 0..n {
                input.push_str(&placements[i][j].to_string());
                if j < n - 1 {
                    input.push(' ');
                }
            }
            input.push('\n');
        }

        // Calculate the minimum cost
        let mut min_cost = i64::MAX;

        // Evaluate each placement strategy
        for strategy in 0..k {
            let mut total_cost = 0;

            // Initialize traffic matrix between cities
            let mut city_traffic = vec![vec![0_i64; m]; m];

            // Calculate traffic flows between cities for this strategy
            for server in 0..n {
                let server_city = placements[strategy][server]; // City where server is placed

                // Add the traffic from this server to each destination city
                for dest_city in 0..m {
                    city_traffic[server_city][dest_city] += traffic_data[server][dest_city] as i64;
                }
            }

            // Calculate the cost for each city-to-city traffic
            for from_city in 0..m {
                for to_city in 0..m {
                    if city_traffic[from_city][to_city] > 0 {
                        if from_city == to_city {
                            total_cost += city_traffic[from_city][to_city]; // Cost is 1 per unit within same city
                        } else {
                            if city_traffic[from_city][to_city] <= 1000 {
                                total_cost += city_traffic[from_city][to_city] * 3; // Cost is 3 per unit if <= 1000
                            } else {
                                total_cost +=
                                    1000 * 3 + (city_traffic[from_city][to_city] - 1000) * 2; // 3 for first 1000, 2 for rest
                            }
                        }
                    }
                }
            }

            // Update minimum cost
            if total_cost < min_cost {
                min_cost = total_cost;
            }
        }

        // Format output
        let output = format!("{}", min_cost);

        TestCase { input, output }
    }
}
