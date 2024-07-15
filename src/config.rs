use std::{
  collections::{HashMap, HashSet},
  path::Path,
};

use once_cell::sync::Lazy;
use serde::Deserialize;
use syntect::parsing::{Scope, SyntaxDefinition, SyntaxSet, SyntaxSetBuilder};

use crate::parser::Parser;

pub static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| {
  let mut builder = SyntaxSetBuilder::new();

  builder.add(SyntaxDefinition::load_from_str(RUST_SYNTAX, false, None).unwrap());

  builder.build()
});

static DEFAULT_CONFIG: &str = include_str!("../example-config.toml");
static RUST_SYNTAX: &str = include_str!("../syntaxes/rust.sublime-syntax");

#[derive(Clone, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub fzf_settings: FzfSettings,
  #[serde(flatten)]
  pub languages: HashMap<String, LanguageConfig>,
}

impl Config {
  pub fn parser_for_file<P: AsRef<Path>>(&self, file: P) -> Option<Parser> {
    let extension = file.as_ref().extension()?.to_str()?;
    let language_config = self.languages.values().find(|l| l.extensions.contains(extension))?;
    let syntax_reference = SYNTAX_SET.find_syntax_by_extension(extension)?;

    Some(Parser::new(&language_config.symbols, syntax_reference))
  }
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
  pub extensions: HashSet<String>,
  pub symbols: HashSet<Scope>,
}
