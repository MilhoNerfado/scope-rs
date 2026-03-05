use std::{path::PathBuf, str::FromStr};

use config::Config;
use dirs::config_dir;
use serde::Deserialize;

/// ScopeConfig is the root configuration struct for the application,
/// containing all the necessary settings for scope-monitor.
#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
#[cfg_attr(
    feature = "gen-docs",
    derive(serde::Serialize, toml_scaffold::TomlScaffold, schemars::JsonSchema)
)]
pub struct ScopeConfig {
    pub general: GeneralConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[cfg_attr(
    feature = "gen-docs",
    derive(serde::Serialize, toml_scaffold::TomlScaffold, schemars::JsonSchema)
)]
pub struct GeneralConfig {
    /// Max number of messages kept in memory
    pub capacity: usize,

    /// Relative path for tag file (user commands)
    pub relative_tag_file: std::path::PathBuf,

    /// Max latency in milliseconds for processing messages before they are dropped
    pub latency: u64,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            capacity: 2000,
            relative_tag_file: PathBuf::from_str("tags.yaml").unwrap_or_default(),
            latency: 100,
        }
    }
}

fn config_path() -> Option<String> {
    let dir = config_dir()?;

    let path = dir.join("scope").join("config.toml");

    let path_str = path.to_str()?;

    Some(path_str.to_string())
}

impl ScopeConfig {
    pub fn load_config() -> Self {
        let Some(path) = config_path() else {
            return Self::default();
        };

        let Some(settings) = Config::builder()
            .add_source(config::File::with_name(&path).required(false))
            .build()
            .ok()
        else {
            return Self::default();
        };

        settings.try_deserialize().unwrap_or_default()
    }
}
