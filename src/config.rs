use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Deserializer};
use tree_sitter::{Language as TreeSitterLanguage, Query};

use crate::{symbol::Kind, utils::OneOrMany};

static DEFAULT_CONFIG: &str = include_str!("../default-config.toml");

#[derive(Deserialize)]
pub struct Config {
  #[serde(flatten, deserialize_with = "deserialize_languages")]
  pub languages: HashMap<Language, LanguageConfig>,

  #[serde(default)]
  pub fzf_settings: FzfSettings,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Language {
  C,
  Go,
  Haskell,
  Python,
  Rust,
  TypeScript,
}

/// A configuration stanza. This structure does not exactly reflect the TOML configuration.
/// It has this shape for efficiency during file parsing.
pub struct LanguageConfig {
  /// Symbols that should be included in the symbol list and which queries match them.
  pub symbol_queries: HashMap<Kind, Vec<Query>>,
}

impl Config {
  pub fn extensions(&self) -> impl Iterator<Item = &'static str> + '_ {
    self.languages.keys().flat_map(Language::extensions).copied()
  }
}

impl Default for Config {
  fn default() -> Self {
    toml::from_str(DEFAULT_CONFIG).unwrap()
  }
}

impl Language {
  pub fn extensions(&self) -> &'static [&'static str] {
    match self {
      Self::C => &["c", "h"],
      Self::Go => &["go"],
      Self::Haskell => &["hs"],
      Self::Python => &["py"],
      Self::Rust => &["rs"],
      Self::TypeScript => &["js", "jsx", "ts", "tsx"],
    }
  }

  pub fn to_tree_sitter(&self) -> TreeSitterLanguage {
    match self {
      Self::C => tree_sitter_c::LANGUAGE.into(),
      Self::Go => tree_sitter_go::LANGUAGE.into(),
      Self::Haskell => tree_sitter_haskell::LANGUAGE.into(),
      Self::Python => tree_sitter_python::LANGUAGE.into(),
      Self::Rust => tree_sitter_rust::LANGUAGE.into(),
      Self::TypeScript => tree_sitter_typescript::LANGUAGE_TSX.into(),
    }
  }

  pub fn from_extension<S: AsRef<str>>(extension: S) -> Option<Self> {
    Language::from_str(extension.as_ref()).ok()
  }
}

impl FromStr for Language {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "c" | "h" => Ok(Self::C),
      "go" => Ok(Self::Go),
      "hs" => Ok(Self::Haskell),
      "py" => Ok(Self::Python),
      "rs" => Ok(Self::Rust),
      "js" | "jsx" | "ts" | "tsx" => Ok(Self::TypeScript),
      _ => Err(()),
    }
  }
}

fn deserialize_languages<'de, D>(deserializer: D) -> Result<HashMap<Language, LanguageConfig>, D::Error>
where
  D: Deserializer<'de>,
{
  let languages = HashMap::<Language, HashMap<Kind, OneOrMany<String>>>::deserialize(deserializer)?;

  Ok(
    languages
      .into_iter()
      .map(|(language, symbol_queries)| {
        let ts_language = language.to_tree_sitter();

        let symbol_queries: HashMap<Kind, Vec<Query>> = symbol_queries
          .into_iter()
          .map(|(symbol_kind, queries)| {
            let queries = Vec::from(queries);
            let queries = queries.into_iter().map(|query| Query::new(&ts_language, &query).unwrap()).collect();

            (symbol_kind, queries)
          })
          .collect();

        (language, LanguageConfig { symbol_queries })
      })
      .collect(),
  )
}

#[derive(Deserialize)]
pub struct FzfSettings {
  #[serde(default = "FzfSettings::default_preview_window")]
  pub preview_window: String,
}

impl FzfSettings {
  fn default_preview_window() -> String {
    "70%".to_string()
  }
}

impl Default for FzfSettings {
  fn default() -> Self {
    Self {
      preview_window: Self::default_preview_window(),
    }
  }
}
