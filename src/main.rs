mod config;
mod ext;
mod fzf;
mod parser;
mod text;
mod utils;
mod worker;

use std::{
  ffi::OsString,
  io::{BufRead, BufReader},
  os::unix::ffi::OsStringExt,
  path::PathBuf,
  process::{Command, Stdio},
  thread::JoinHandle,
};

use anyhow::Context;
use clap::Parser;
use crossbeam::channel::Receiver;

use crate::{config::Config, ext::AnyExt, fzf::Fzf, worker::Worker};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// A configuration TOML string.
  #[arg(short, long)]
  config: Option<String>,
}

/// Spawns an `fd` process to list all files under a directory.
fn files() -> Result<(Receiver<PathBuf>, JoinHandle<()>), anyhow::Error> {
  let (send, recv) = crossbeam::channel::bounded(crate::utils::num_threads());

  let mut child = Command::new("fd")
    .args(["-t", "f", "-0"])
    .stdout(Stdio::piped())
    .spawn()
    .context("spawn")?;

  let stdout = child.stdout.take().context("stdout")?;

  let handle = std::thread::spawn(move || {
    for line in BufReader::new(stdout).split(b'\0') {
      let line = OsString::from_vec(line.expect("failed to get line"));

      send.send(PathBuf::from(line)).expect("failed to send");
    }
  });

  Ok((recv, handle))
}

fn main() -> Result<(), anyhow::Error> {
  let args = Args::parse();
  let config = args
    .config
    .as_deref()
    .map_or(Config::default().ok(), toml::from_str)
    .context("parse config")?;

  let fzf = Fzf::new(&config.fzf_settings).context("fzf")?;

  let (files, _) = files()?;

  for _ in 0..crate::utils::num_threads() {
    Worker::new(&config, &files, &fzf).run();
  }

  let selection = fzf.wait().context("wait")?;

  println!("{selection}");

  Ok(())
}
