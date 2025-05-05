use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>, // Number of customers
    pub a_b_range: RangeInclusive<i32>, // Range for product IDs a and b
    pub transactions_range: RangeInclusive<usize>, // Range for number of transactions per customer
    pub actions_range: RangeInclusive<usize>, // Range for number of actions per transaction
    pub remove_actions_pct: f64,        // Percentage of remove actions (negative numbers)
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
            a_b_range,
            transactions_range,
            actions_range,
            remove_actions_pct,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() > 0);
        assert!(*a_b_range.start() > 0);
        assert!(*transactions_range.start() > 0);
        assert!(*actions_range.start() > 0);
        assert!(remove_actions_pct >= 0.0 && remove_actions_pct <= 1.0);

        let mut rng = rand::rng();

        // Generate a and b (target products to track)
        let a = rng.random_range(a_b_range.clone());
        let mut b = rng.random_range(a_b_range.clone());
        while b == a {
            b = rng.random_range(a_b_range.clone());
        }

        // Generate number of customers
        let n = rng.random_range(n_range);

        // Format input header
        let mut input = format!("{} {}\n{}\n", a, b, n);

        // Track how many customers bought both a and b
        let mut customers_with_both = 0;

        // Generate shopping records for each customer
        for _ in 0..n {
            let num_transactions = rng.random_range(transactions_range.clone());

            // Initialize counters for products a and b
            let mut a_count = 0;
            let mut b_count = 0;

            // Generate transactions for this customer
            let mut transactions = Vec::new();

            // Decide if customer will buy both a and b
            let will_buy_both = rng.random_bool(0.5);

            for _ in 0..num_transactions {
                if will_buy_both && (a_count == 0 || b_count == 0) {
                    // Ensure this customer buys at least one a and one b if they are meant to buy both
                    if a_count == 0 {
                        transactions.push(a);
                        a_count += 1;
                    } else {
                        transactions.push(b);
                        b_count += 1;
                    }
                } else {
                    // Random product ID between 1 and 100
                    let product_id = rng.random_range(1..=100);

                    // Decide if this is a remove action or add action
                    let is_remove = rng.random_bool(remove_actions_pct);

                    if is_remove {
                        if product_id == a && a_count > 0 {
                            transactions.push(-a);
                            a_count -= 1;
                        } else if product_id == b && b_count > 0 {
                            transactions.push(-b);
                            b_count -= 1;
                        } else {
                            transactions.push(-product_id);
                        }
                    } else {
                        transactions.push(product_id);
                        if product_id == a {
                            a_count += 1;
                        } else if product_id == b {
                            b_count += 1;
                        }
                    }
                }
            }

            // Add terminal 0 to mark end of transactions
            transactions.push(0);

            // Format customer's transactions
            let transaction_str = transactions
                .iter()
                .map(|&t| t.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            input.push_str(&transaction_str);
            input.push('\n');

            // Check if customer bought both products a and b
            if a_count > 0 && b_count > 0 {
                customers_with_both += 1;
            }
        }

        // Format output
        let output = format!("{}", customers_with_both);

        TestCase { input, output }
    }
}
