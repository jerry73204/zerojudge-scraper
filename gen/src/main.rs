pub mod config;
pub mod e288;
pub mod opts;
pub mod sampler;

use crate::sampler::TestSampler;
use clap::Parser;
use config::LevelList;
use opts::Opts;
use std::{collections::HashMap, fs, path::Path, sync::LazyLock};

type BuilderFn = Box<dyn Fn() -> Box<dyn TestSampler> + Sync + Send>;

macro_rules! tab {
    ($($id:ident),*) => {
	{
	    use crate::sampler::{TestSampler};
	    use std::collections::HashMap;

            let list = [$(
		{
		    let builder_fn = || -> Box<dyn TestSampler> {
			Box::new($id::Sampler::new())
		    };
		    let builder_fn: BuilderFn = Box::new(builder_fn);
		    (stringify!($id), builder_fn)
		}
	    ),*];
	    let tab: HashMap<&str, BuilderFn> = list.into_iter().collect();
	    tab
	}
    };
}

static SAMPLER_TAB: LazyLock<HashMap<&str, BuilderFn>> = LazyLock::new(|| tab!(e288));

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    let Opts {
        problem_id,
        num_tests,
        difficulty,
    } = opts;

    let configs: LevelList = {
        let config_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("config");
        let config_file = config_dir.join(format!("{problem_id}.json5"));
        let text = std::fs::read_to_string(config_file)?;
        json5::from_str(&text)?
    };

    let difficulty_range = {
        let start = *difficulty.start() as f32 / 100.0;
        let end = *difficulty.end() as f32 / 100.0;
        start..=end
    };
    let builder_fn = &SAMPLER_TAB[problem_id.as_str()];
    let mut sampler = builder_fn();
    let samples = sampler.sample_many(&configs, difficulty_range, num_tests);

    let output_dir = Path::new(&problem_id);
    fs::create_dir(output_dir)?;

    for (ix, sample) in samples.into_iter().enumerate() {
        let input_file = output_dir.join(format!("{ix}.in"));
        fs::write(&input_file, sample.input)?;

        let output_file = output_dir.join(format!("{ix}.out"));
        fs::write(&output_file, sample.output)?;
    }

    Ok(())
}
