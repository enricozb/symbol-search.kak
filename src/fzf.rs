use std::{
  fmt::Display,
  io::Write,
  path::PathBuf,
  process::{Child, ChildStdin, Command, Stdio},
  sync::Arc,
};

use anyhow::Context;
use parking_lot::Mutex;

const SPACE: char = '\u{2008}';

use crate::config::FzfSettings;

pub struct Fzf {
  child: Child,
  stdin: Arc<Mutex<ChildStdin>>,
}

pub struct Sink {
  stdin: Arc<Mutex<ChildStdin>>,
}

pub struct Entry {
  path: PathBuf,
  loc: Loc,
  symbol: String,
  kind: SymbolKind,
}

pub struct Loc {
  line: usize,
  column: usize,
}

impl Loc {
  pub fn new(line: usize, column: usize) -> Self {
    Self { line, column }
  }
}

#[derive(Clone, Copy)]
pub enum SymbolKind {
  Class,
  Function,
  Global,
}

impl Fzf {
  /// Spawns `fzf` process that expects stdin entries of the form
  /// `<path> <line> <column> <symbol> <kind>` separated by [`SPACE`].
  pub fn new(settings: &FzfSettings) -> Result<Fzf, anyhow::Error> {
    let mut child = Command::new("fzf")
      .args([
        "--ansi",
        "--delimiter=\u{2008}",
        "--with-nth=-1,-2",
        "--nth=2",
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

  pub fn send(&self, entry: Entry) -> Result<(), std::io::Error> {
    self.stdin.lock().write(format!("{entry}\n").as_bytes())?;

    Ok(())
  }
}

impl Display for Entry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{path}{SPACE}{line}{SPACE}{column}{SPACE}{symbol}{SPACE}{kind}",
      path = self.path.to_string_lossy(),
      line = self.loc.line,
      column = self.loc.column,
      symbol = self.symbol,
      kind = self.kind.short(),
    )
  }
}

impl SymbolKind {
  pub fn short(self) -> &'static str {
    // these strings must all have the same printable length
    match self {
      Self::Class => "\x1b[36m(cls)\x1b[0m",
      Self::Function => "\x1b[35m(fun)\x1b[0m",
      Self::Global => "\x1b[33m(gbl)\x1b[0m",
    }
  }
}
