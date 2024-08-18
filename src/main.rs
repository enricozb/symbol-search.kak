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

use crate::{config::Config, fd::Fd, fzf::Fzf, worker::Worker};

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
  cache: Option<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
  let args = Args::parse();

  let config = if let Some(config) = args.config {
    toml::from_str(&config)?
  } else {
    Config::default()
  };

  let fzf = Fzf::new(&config.fzf_settings).context("fzf")?;
  let fd = Fd::new(config.extensions()).context("fd")?;

  for _ in 0..crate::utils::num_threads() {
    Worker::new(&config, fd.files(), &fzf).run();
  }

  let selection = fzf.wait().context("wait")?;
  println!("{selection}");

  Ok(())
}
