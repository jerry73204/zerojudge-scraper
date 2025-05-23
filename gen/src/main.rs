pub mod config;
pub mod generators;
pub mod opts;
pub mod sampler;
pub mod utils;

use crate::sampler::TestSampler;
use clap::Parser;
use config::LevelList;
use eyre::ensure;
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
			Box::new(crate::generators::$id::Sampler::new())
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

static SAMPLER_TAB: LazyLock<HashMap<&str, BuilderFn>> = LazyLock::new(|| {
    tab!(
        e288, b266, b294, b965, b966, b967, c290, c291, c575, e283, f312, f313, f315, f579, f580,
        f581, f582, f606, f607, f608, f637, f638, f640, g275, g276, g277, g595
    )
});

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    let Opts {
        problem_id,
        num_tests,
        difficulty,
        config: config_dir,
    } = opts;

    let configs: LevelList = {
        ensure!(
            config_dir.exists(),
            "expect the configuration directory '{}' to exist. \
	     please create one or specfiy the path using --config",
            config_dir.display()
        );

        let config_file = config_dir.join(format!("{problem_id}.json5"));
        LevelList::open(config_file)?
    };

    let difficulty_range = {
        let start = *difficulty.start() as f32 / 100.0;
        let end = *difficulty.end() as f32 / 100.0;
        start..=end
    };
    let builder_fn = &SAMPLER_TAB[problem_id.as_str()];
    let sampler = builder_fn();
    let samples = sampler.sample_many(&configs, difficulty_range, num_tests);

    let output_dir = Path::new(&problem_id);

    if output_dir.exists() {
        eprintln!(
            "the output directory '{}' already exists. the generated output files will be overwritten.",
            output_dir.display()
        );
    }
    fs::create_dir_all(output_dir)?;

    for (ix, sample) in samples.into_iter().enumerate() {
        let input_file = output_dir.join(format!("{ix}.in"));
        fs::write(&input_file, sample.input)?;

        let output_file = output_dir.join(format!("{ix}.out"));
        fs::write(&output_file, sample.output)?;
    }

    Ok(())
}
