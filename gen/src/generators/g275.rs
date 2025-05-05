use crate::sampler::{TestCase, TestSampler};
use rand::prelude::*;
use serde::Deserialize;
use std::{marker::PhantomData, ops::RangeInclusive};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub n_range: RangeInclusive<usize>,    // Number of couplets
    pub valid_percent: RangeInclusive<u8>, // Percentage of valid couplets
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
            valid_percent,
        } = serde_json::from_value(config.clone()).unwrap();

        assert!(*n_range.start() >= 1);
        assert!(*valid_percent.start() <= 100);
        assert!(*valid_percent.end() <= 100);

        let mut rng = rand::rng();
        let n = rng.random_range(n_range);

        let mut couplets = Vec::new();
        let mut outputs = Vec::new();

        // Get the range for valid percentages
        let min_valid = *valid_percent.start();
        let max_valid = *valid_percent.end();

        for _ in 0..n {
            // Decide whether this couplet should be valid
            let threshold = rng.random_range(min_valid..=max_valid);
            let should_be_valid = rng.random_range(0..100) < threshold;

            if should_be_valid {
                // Generate a valid couplet
                let (output, couplet) = generate_valid_couplet(&mut rng);
                couplets.push(couplet);
                outputs.push(output);
            } else {
                // Generate an invalid couplet with random rule violations
                let (output, couplet) = generate_invalid_couplet(&mut rng);
                couplets.push(couplet);
                outputs.push(output);
            }
        }

        // Format input and output
        let input = format!("{}\n{}", n, couplets.join("\n"));

        let output = outputs.join("\n");

        TestCase { input, output }
    }
}

// Generate a valid Chinese couplet
fn generate_valid_couplet(rng: &mut impl Rng) -> (String, String) {
    // First line template: Rule A: 2nd and 4th different, 2nd and 6th same
    // Let's set 2nd position to 1
    let mut line1 = vec![0; 7];
    line1[1] = 1; // 2nd position
    line1[3] = 0; // 4th position (different from 2nd)
    line1[5] = 1; // 6th position (same as 2nd)
    line1[6] = 1; // Rule B: First line ends with 仄声 (1)

    // Randomly generate other positions
    line1[0] = rng.random_range(0..=1);
    line1[2] = rng.random_range(0..=1);
    line1[4] = rng.random_range(0..=1);

    // Second line: Rule C: 2nd, 4th, 6th positions must be opposite of first line
    let mut line2 = vec![0; 7];
    line2[1] = 1 - line1[1]; // 2nd position (opposite)
    line2[3] = 1 - line1[3]; // 4th position (opposite)
    line2[5] = 1 - line1[5]; // 6th position (opposite)
    line2[6] = 0; // Rule B: Second line ends with 平声 (0)

    // Randomly generate other positions
    line2[0] = rng.random_range(0..=1);
    line2[2] = rng.random_range(0..=1);
    line2[4] = rng.random_range(0..=1);

    let couplet = format!(
        "{}\n{}",
        line1
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        line2
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    ("None".to_string(), couplet)
}

// Generate an invalid Chinese couplet with random rule violations
fn generate_invalid_couplet(rng: &mut impl Rng) -> (String, String) {
    // Start with a valid couplet
    let mut line1 = vec![0; 7];
    let mut line2 = vec![0; 7];

    // Randomly fill all positions
    for i in 0..7 {
        line1[i] = rng.random_range(0..=1);
        line2[i] = rng.random_range(0..=1);
    }

    // Choose which rules to violate
    let violated_a = rng.random_range(0..=1) == 1;
    let violated_b = rng.random_range(0..=1) == 1;
    let violated_c = rng.random_range(0..=1) == 1;

    // Make sure at least one rule is violated
    let all_valid = !violated_a && !violated_b && !violated_c;
    let violated_a = violated_a || (all_valid && rng.random_range(0..=2) == 0);
    let violated_b = violated_b || (all_valid && rng.random_range(0..=1) == 0);
    let violated_c = violated_c || all_valid;

    // Rule A: 2nd and 4th different, 2nd and 6th same
    if !violated_a {
        if line1[1] == line1[3] {
            line1[3] = 1 - line1[1]; // Make 2nd and 4th different
        }
        line1[5] = line1[1]; // Make 2nd and 6th same
    }

    // Rule B: First line ends with 仄声 (1), second line ends with 平声 (0)
    if !violated_b {
        line1[6] = 1;
        line2[6] = 0;
    }

    // Rule C: 2nd, 4th, 6th positions in two lines must be opposite
    if !violated_c {
        line2[1] = 1 - line1[1];
        line2[3] = 1 - line1[3];
        line2[5] = 1 - line1[5];
    }

    // Determine which rules are violated
    let mut violations = Vec::new();

    // Check Rule A
    let rule_a_violated = line1[1] == line1[3] || line1[1] != line1[5];
    if rule_a_violated {
        violations.push('A');
    }

    // Check Rule B
    let rule_b_violated = line1[6] != 1 || line2[6] != 0;
    if rule_b_violated {
        violations.push('B');
    }

    // Check Rule C
    let rule_c_violated =
        line1[1] + line2[1] != 1 || line1[3] + line2[3] != 1 || line1[5] + line2[5] != 1;
    if rule_c_violated {
        violations.push('C');
    }

    let output = if violations.is_empty() {
        "None".to_string()
    } else {
        violations.iter().collect::<String>()
    };

    let couplet = format!(
        "{}\n{}",
        line1
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        line2
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    (output, couplet)
}
