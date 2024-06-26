use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  #[serde(default)]
  pub settings: Settings,

  #[serde(flatten)]
  pub languages: HashMap<String, LanguageConfig>,
}

impl Config {
  pub fn languages_with_extension<'a, 'b: 'a>(
    &'a self,
    extension: &'b String,
  ) -> impl Iterator<Item = (&'a str, &'a LanguageConfig)> + 'a {
    self.languages.iter().filter_map(|(language, config)| {
      if config.extensions.contains(extension) {
        Some((language.as_str(), config))
      } else {
        None
      }
    })
  }
}

#[derive(Deserialize)]
pub struct Settings {
  #[serde(default = "default_preview_window")]
  pub preview_window: String,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      preview_window: default_preview_window(),
    }
  }
}

fn default_preview_window() -> String {
  "70%".to_string()
}

#[derive(Deserialize)]
pub struct LanguageConfig {
  pub extensions: Vec<String>,
  pub symbols: HashMap<String, Symbol>,
}

#[derive(Clone, Deserialize)]
pub struct Symbol {
  #[serde(rename = "type")]
  pub kind: SymbolKind,
  pub regex: String,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
  Class,
  Function,
  Global,
}

impl SymbolKind {
  pub fn short(self) -> &'static str {
    // it is relied on that these strings all have the same printable length
    match self {
      Self::Class => "\x1b[36m(cls)\x1b[0m",
      Self::Function => "\x1b[35m(fun)\x1b[0m",
      Self::Global => "\x1b[33m(gbl)\x1b[0m",
    }
  }
}
