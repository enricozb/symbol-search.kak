use std::{
  fmt::Display,
  io::Write,
  path::Path,
  process::{Child, ChildStdin, Command, Stdio},
  sync::Arc,
};

use anyhow::Context;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

const SPACE: char = '\u{2008}';

use crate::{config::FzfSettings, text::Loc};

pub struct Fzf {
  child: Child,
  stdin: Arc<Mutex<ChildStdin>>,
}

pub struct Sink {
  stdin: Arc<Mutex<ChildStdin>>,
}

#[derive(Serialize, Deserialize)]
pub struct Entry<P, S> {
  pub path: P,
  pub loc: Loc,
  pub symbol: S,
  pub kind: SymbolKind,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SymbolKind {
  Module,
  Macro,
  Global,
  Constant,
  Define,

  Class,
  Struct,
  Enum,
  Union,

  Alias,
  Interface,
  Trait,
  Type,

  Function,
  Impl,

  Unknown,
}

impl Fzf {
  /// Spawns `fzf` process that expects stdin entries of the form
  /// `<path> <line> <column> <symbol> <kind>` separated by [`SPACE`].
  pub fn new(settings: &FzfSettings) -> Result<Fzf, anyhow::Error> {
    let mut child = Command::new("fzf")
      .args([
        "--ansi",
        &format!("--delimiter={SPACE}"),
        "--nth=-1",
        "--with-nth=5,4",
        "--reverse",
        "--preview=bat {1} --color always --style=numbers,snip,header --highlight-line {2} --line-range {2}:+100",
        "--bind=tab:down,shift-tab:up",
      ])
      .args([format!("--preview-window={}", settings.preview_window)])
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .context("spawn")?;

    let stdin = child.stdin.take().context("stdin")?;

    Ok(Fzf {
      child,
      stdin: Arc::new(Mutex::new(stdin)),
    })
  }

  pub fn sink(&self) -> Sink {
    Sink::new(self.stdin.clone())
  }

  pub fn wait(self) -> Result<String, anyhow::Error> {
    // when all references to `stdin` are dropped, the spinner will stop.
    drop(self.stdin);

    let output = self.child.wait_with_output().context("wait")?;

    let selection = String::from_utf8_lossy(&output.stdout)
      .split(SPACE)
      .take(3)
      .collect::<Vec<_>>()
      .join(" ");

    Ok(selection)
  }
}

impl Sink {
  pub fn new(stdin: Arc<Mutex<ChildStdin>>) -> Self {
    Self { stdin }
  }

  pub fn send<P: AsRef<Path>, S: Display>(&self, entry: &Entry<P, S>) -> Result<(), std::io::Error> {
    self.stdin.lock().write_all(format!("{entry}\n").as_bytes())?;

    Ok(())
  }
}

impl<P, S> Entry<P, S> {
  pub fn new(path: P, loc: Loc, symbol: S, kind: SymbolKind) -> Self {
    Self {
      path,
      loc,
      symbol,
      kind,
    }
  }
}

impl<P: AsRef<Path>, S: Display> Display for Entry<P, S> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{path}{SPACE}{line}{SPACE}{column}{SPACE}{symbol}{SPACE}{kind}{SPACE}",
      path = self.path.as_ref().to_string_lossy(),
      line = self.loc.line,
      column = self.loc.column,
      symbol = self.symbol,
      kind = self.kind.short(),
    )
  }
}

impl SymbolKind {
  #[rustfmt::skip]
  pub fn short(self) -> &'static str {
    // these strings must all have the same printable length
    match self {
      Self::Module    => "\x1b[33m(mod)   \x1b[0m",
      Self::Macro     => "\x1b[33m(macro) \x1b[0m",
      Self::Global    => "\x1b[33m(global)\x1b[0m",
      Self::Constant  => "\x1b[33m(const) \x1b[0m",
      Self::Define    => "\x1b[33m(const) \x1b[0m",

      Self::Class     => "\x1b[36m(class) \x1b[0m",
      Self::Struct    => "\x1b[36m(struct)\x1b[0m",
      Self::Enum      => "\x1b[36m(enum)  \x1b[0m",
      Self::Union     => "\x1b[36m(union) \x1b[0m",

      Self::Alias     => "\x1b[34m(alias) \x1b[0m",
      Self::Interface => "\x1b[34m(inter) \x1b[0m",
      Self::Trait     => "\x1b[34m(trait) \x1b[0m",
      Self::Type      => "\x1b[34m(type)  \x1b[0m",

      Self::Function  => "\x1b[35m(func)  \x1b[0m",
      Self::Impl      => "\x1b[35m(impl)  \x1b[0m",

      Self::Unknown   => "\x1b[31m(??????)\x1b[0m",    }
  }
}
