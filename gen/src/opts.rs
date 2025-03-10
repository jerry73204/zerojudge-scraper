use std::ops::RangeInclusive;

use clap::Parser;
use eyre::{Context, ensure};

#[derive(Parser)]
pub struct Opts {
    pub problem_id: String,

    #[clap(default_value = "1")]
    pub num_tests: u32,

    #[clap(
        default_value = "default_difficulty",
        value_parser = parse_difficulty
    )]
    pub difficulty: RangeInclusive<u32>,
}

fn parse_difficulty(arg: &str) -> eyre::Result<RangeInclusive<u32>> {
    let invalid_format =
        || format!("{arg} is not a valid difficulty specification. It should be 100 or 0-100.");

    let range = match arg.split_once("-") {
        Some((lhs, rhs)) => {
            let lhs = lhs.parse().wrap_err_with(invalid_format)?;
            let rhs = rhs.parse().wrap_err_with(invalid_format)?;
            ensure!(
                (0..=100).contains(&lhs) && (0..=100).contains(&rhs) && lhs <= rhs,
                "Invalid difficulty value range. It should be within 0 and 100."
            );
            lhs..=rhs
        }
        None => {
            let val = arg.parse().wrap_err_with(invalid_format)?;
            val..=val
        }
    };
    Ok(range)
}
