use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct LevelList {
    pub level: Vec<Level>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Level {
    pub level: f32,
    #[serde(flatten)]
    pub config: serde_json::Value,
}
