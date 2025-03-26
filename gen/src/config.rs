use eyre::{Context, bail, ensure};
use serde::Deserialize;
use std::path::Path;

macro_rules! notnan {
    ($val:expr) => {
        ordered_float::NotNan::new($val).unwrap()
    };
}

#[derive(Debug, Clone, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct LevelList {
    pub level: Vec<Level>,
}

impl LevelList {
    pub fn open<P>(path: P) -> eyre::Result<Self>
    where
        P: AsRef<Path>,
    {
        let text = std::fs::read_to_string(&path)
            .wrap_err_with(|| format!("unable to open '{}'", path.as_ref().display()))?;
        let list: Self = json5::from_str(&text)
            .wrap_err_with(|| format!("unable to open '{}'", path.as_ref().display()))?;

        let Some(last) = list.level.last() else {
            bail!("at least one level is required");
        };
        ensure!(last.level == 100, "the last level must be 100");

        for window in list.level.windows(2) {
            let [prev, next] = window else { unreachable!() };
            ensure!(
                prev.level < next.level,
                "the levels must be arranged in increasing order"
            );
        }

        Ok(list)
    }

    pub fn select(&self, difficulty: f32) -> eyre::Result<&serde_json::Value> {
        ensure!(
            (0.0..=1.0).contains(&difficulty),
            "the difficulty must be in within 0.0 and 1.0"
        );

        let result = self
            .level
            .binary_search_by_key(&notnan!(difficulty), |level| {
                notnan!(level.level as f32 / 100.0)
            });
        let ix = match result {
            Ok(ix) => ix,
            Err(ix) => ix,
        };
        let Level { config, .. } = &self.level[ix];
        Ok(config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Level {
    pub level: u32,
    #[serde(flatten)]
    pub config: serde_json::Value,
}
