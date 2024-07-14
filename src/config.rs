use std::collections::HashMap;

use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../example-config.toml");

#[derive(Clone, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub fzf_settings: FzfSettings,
  #[serde(flatten)]
  pub languages: HashMap<String, LanguageConfig>,
}

impl Default for Config {
  fn default() -> Self {
    toml::from_str(DEFAULT_CONFIG).unwrap()
  }
}

#[derive(Clone, Deserialize)]
pub struct FzfSettings {
  #[serde(default = "default_preview_window")]
  pub preview_window: String,
}

impl Default for FzfSettings {
  fn default() -> Self {
    Self {
      preview_window: default_preview_window(),
    }
  }
}

fn default_preview_window() -> String {
  "70%".to_string()
}

#[derive(Clone, Deserialize)]
pub struct LanguageConfig {
  pub extensions: Vec<String>,
  pub symbols: Vec<String>,
}
