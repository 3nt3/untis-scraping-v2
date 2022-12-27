use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub untis: UntisConfig,
}

#[derive(Debug, Deserialize)]
pub struct UntisConfig {
    pub school: String,
    pub username: String,
    pub password: String,
}

pub fn read_config(path: String) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let parsed: Config = toml::from_str(&content)?;
    Ok(parsed)
}
