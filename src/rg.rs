use std::process::Stdio;

use anyhow::Context;
use tokio::process::{Child, ChildStdout, Command};

use crate::config::Symbol;

pub struct Rg {
  pub symbol: Symbol,

  pub child: Child,
  pub stdout: ChildStdout,
}

impl Rg {
  pub fn new(symbol: Symbol) -> Result<Self, anyhow::Error> {
    let mut child = Command::new("rg")
      .args(["--vimgrep", "--only-matching", "--replace", "$item"])
      .arg(&symbol.regex)
      .stdout(Stdio::piped())
      .spawn()
      .context("spawn")?;
    let stdout = child.stdout.take().context("stdout")?;

    Ok(Self { symbol, child, stdout })
  }
}
