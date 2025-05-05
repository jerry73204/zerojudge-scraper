use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub r_range: RangeInclusive<usize>,        // Number of rows
    pub c_range: RangeInclusive<usize>,        // Number of columns
    pub k_range: RangeInclusive<usize>,        // Division factor (4-50)
    pub m_range: RangeInclusive<usize>,        // Number of days
    pub population_range: RangeInclusive<i32>, // Range for initial population
    pub city_probability: f64,                 // Probability of a cell being a city (vs -1)
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
            r_range,
            c_range,
            k_range,
            m_range,
            population_range,
            city_probability,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*r_range.start() > 0);
        assert!(*c_range.start() > 0);
        assert!(*k_range.start() >= 4);
        assert!(*m_range.start() > 0);
        assert!(city_probability >= 0.0 && city_probability <= 1.0);

        let mut rng = rand::rng();
        let r = rng.random_range(r_range); // Number of rows
        let c = rng.random_range(c_range); // Number of columns
        let k = rng.random_range(k_range); // Division factor
        let m = rng.random_range(m_range); // Number of days

        // Format input header
        let mut input = format!("{} {} {} {}\n", r, c, k, m);

        // Generate the grid
        let mut grid = Vec::with_capacity(r);
        for _ in 0..r {
            let mut row = Vec::with_capacity(c);
            for _ in 0..c {
                if rng.random_bool(city_probability) {
                    // This is a city, generate a random population
                    row.push(rng.random_range(population_range.clone()));
                } else {
                    // This is not a city
                    row.push(-1);
                }
            }
            grid.push(row);
        }

        // Ensure there's at least one city
        if grid.iter().flatten().all(|&x| x == -1) {
            let random_row = rng.random_range(0..r);
            let random_col = rng.random_range(0..c);
            grid[random_row][random_col] = rng.random_range(population_range.clone());
        }

        // Add grid to input
        for row in &grid {
            let row_str = row
                .iter()
                .map(|&p| p.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            input.push_str(&format!("{}\n", row_str));
        }

        // Simulate the migration process
        for _ in 0..m {
            // Create a new grid to store updated populations
            let mut new_grid = grid.clone();

            // Process each city
            for i in 0..r {
                for j in 0..c {
                    if grid[i][j] <= 0 {
                        continue; // Skip non-cities or empty cities
                    }

                    let transfer = grid[i][j] / (k as i32);

                    // Transfer population to neighbors if they are cities
                    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up

                    for &(di, dj) in &directions {
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;

                        // Check if the neighbor is within bounds
                        if ni >= 0 && ni < r as i32 && nj >= 0 && nj < c as i32 {
                            let ni = ni as usize;
                            let nj = nj as usize;

                            // Check if the neighbor is a city
                            if grid[ni][nj] != -1 {
                                // Transfer population
                                new_grid[ni][nj] += transfer;
                                new_grid[i][j] -= transfer;
                            }
                        }
                    }
                }
            }

            // Update the grid
            grid = new_grid;
        }

        // Find minimum and maximum populations
        let mut min_population = i32::MAX;
        let mut max_population = i32::MIN;

        for row in &grid {
            for &population in row {
                if population != -1 {
                    min_population = min_population.min(population);
                    max_population = max_population.max(population);
                }
            }
        }

        // Format output
        let output = format!("{}\n{}", min_population, max_population);

        TestCase { input, output }
    }
}
