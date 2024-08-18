mod cache;
mod config;
mod ext;
mod fd;
mod fzf;
mod parser;
mod text;
mod utils;
mod worker;

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use crate::{cache::Cache, config::Config, fd::Fd, fzf::Fzf, worker::Worker};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// A configuration TOML string.
  #[arg(short, long)]
  config: Option<String>,
  /// Directory to cache parsed symbols.
  ///
  /// Files are reparsed if their cached mtime differs from than their current mtime.
  /// The cache is only usable if the previously generated relative paths are still valid.
  /// This would normally only be the case when the binary is called from the same
  /// directory multiple times.
  ///
  /// This directory is created if it does not exist.
  #[arg(short, long)]
  cache_dir: Option<PathBuf>,
}

impl Args {
  /// Returns the parsed provided config or the default one.
  pub fn config(&self) -> Result<Config, anyhow::Error> {
    if let Some(config) = &self.config {
      toml::from_str(config).context("from_str")
    } else {
      Ok(Config::default())
    }
  }

  /// Returns the provided cache or an empty one.
  pub fn cache(&self) -> Result<Cache, anyhow::Error> {
    if let Some(cache_dir) = &self.cache_dir {
      Cache::from_dir(cache_dir).context("from_str")
    } else {
      Ok(Cache::default())
    }
  }
}

fn main() -> Result<(), anyhow::Error> {
  let args = Args::parse();

  let config = args.config().context("config")?;
  let cache = args.cache().context("cache")?;

  let fzf = Fzf::new(&config.fzf_settings).context("fzf")?;
  let fd = Fd::new(config.extensions()).context("fd")?;

  for _ in 0..crate::utils::num_threads() {
    Worker::new(&config, &cache, fd.files(), &fzf).run();
  }

  // the cache is saved on drop
  drop(cache);

  let selection = fzf.wait().context("wait")?;
  println!("{selection}");

  Ok(())
}
