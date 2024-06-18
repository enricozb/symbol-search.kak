mod config;
mod ext;
mod fzf;
mod rg;

use std::{path::PathBuf, process::Stdio};

use anyhow::Context;
use clap::Parser;
use futures::{Stream, StreamExt};
use tokio::process::Command;

use self::{
  config::Config,
  ext::{AsyncReadExt, UniqueExt},
  fzf::Fzf,
  rg::Rg,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// A configuration TOML string.
  #[arg(short, long)]
  config: String,
}

fn unique_extensions() -> Result<impl Stream<Item = String>, anyhow::Error> {
  let mut child = Command::new("fd").stdout(Stdio::piped()).spawn().context("spawn")?;
  let stdout = child.stdout.take().context("stdout")?;

  Ok(
    stdout
      .lines_stream()
      .filter_map(|path| async {
        let path = PathBuf::from(path.ok()?);

        Some(path.extension()?.to_str()?.to_string())
      })
      .unique(),
  )
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  let args = Args::parse();
  let config: Config = toml::from_str(&args.config).context("parse config")?;

  let fzf = Fzf::new(&config.settings).context("fzf")?;

  let extensions = unique_extensions().context("extensions")?;
  futures::pin_mut!(extensions);

  while let Some(extension) = extensions.next().await {
    for (language, language_config) in config.languages_with_extension(&extension) {
      for symbol in language_config.symbols.values() {
        fzf.insert_all(Rg::new(language, symbol.clone()).context("rg")?);
      }
    }
  }

  let selection = fzf.wait().context("wait")?;

  println!("{selection}");

  Ok(())
}
