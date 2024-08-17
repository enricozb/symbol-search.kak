use std::{
  collections::{HashMap, HashSet},
  path::Path,
};

use once_cell::sync::Lazy;
use serde::Deserialize;
use syntect::parsing::{Scope, SyntaxSet};

use crate::parser::{scope::ScopeExpr, Parser};

pub static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| syntect::dumps::from_binary(SYNTAX_SET_BIN));

static DEFAULT_CONFIG: &str = include_str!("../default-config.toml");
static SYNTAX_SET_BIN: &[u8] = include_bytes!("../syntax/bin/syntax-set.bin");

#[derive(Clone, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub fzf_settings: FzfSettings,
  #[serde(flatten)]
  pub languages: HashMap<String, LanguageConfig>,
}

impl Config {
  pub fn parser_for_file<P: AsRef<Path>>(&self, file: P) -> Option<Parser> {
    let file = file.as_ref();
    let extension = file.extension()?.to_str()?;
    let language_config = self.languages.values().find(|l| l.extensions.contains(extension))?;
    let syntax_reference = SYNTAX_SET.find_syntax_by_extension(extension)?;

    Some(Parser::new(
      &language_config.include,
      &language_config.restrict,
      &language_config.exclude,
      syntax_reference,
    ))
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

/// A configuration stanza. This structure does not exactly reflect the TOML configuration.
/// It has this shape for efficiency during file parsing.
#[derive(Clone)]
pub struct LanguageConfig {
  /// Extensions that should trigger this language's parser.
  pub extensions: HashSet<String>,
  /// Symbols that should be included in the symbol list.
  ///
  /// Scopes can individually exclude a parent scope. For example, in Go, one of the
  /// symbols to be indexed is `variable.other.constant.declaration.go - meta.block.go`,
  /// which is all instances of constant declarations _not in_ a `meta` block. This
  /// map maps scopes to be included to an optional manual exclusion.
  pub include: HashMap<Scope, Option<Scope>>,
  /// Symbols that restrict some scopes.
  ///
  /// These are all non-`None` values in the `include` field.
  pub restrict: HashSet<Scope>,
  /// Symbols that should be excluded in the symbol list.
  ///
  /// For example, in TypeScript, function definitions _and calls_ have the same scope,
  /// but we only want function definitions to be searched. TypeScript's default config
  /// excludes the function body scope.
  pub exclude: HashSet<Scope>,
}

impl<'de> Deserialize<'de> for LanguageConfig {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct Helper {
      extensions: HashSet<String>,
      include: HashSet<ScopeExpr>,
      #[serde(default)]
      exclude: HashSet<Scope>,
    }

    let Helper {
      extensions,
      include,
      exclude,
    } = Helper::deserialize(deserializer)?;

    Ok(Self {
      extensions,
      restrict: include.iter().flat_map(|expr| expr.exclude).clone().collect(),
      include: include.into_iter().map(|expr| (expr.scope, expr.exclude)).collect(),
      exclude,
    })
  }
}
