use std::{
  collections::HashMap,
  io::Write,
  path::PathBuf,
  process::{Child, ChildStdin, Command, Stdio},
  sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
  },
};

use anyhow::Context;
use futures::StreamExt;
use parking_lot::Mutex;

const SPACE: char = '\u{2008}';

use crate::{ext::AsyncReadExt, rg::Rg};

pub struct Fzf {
  child: Child,
  stdin: Arc<Mutex<ChildStdin>>,

  last_entry_id: Arc<AtomicUsize>,
  entries: Arc<Mutex<HashMap<usize, Match>>>,
}

pub struct Match {
  path: PathBuf,
  loc: Loc,
  symbol: String,
}

impl Match {
  pub fn parse(string: &str) -> Result<Self, anyhow::Error> {
    let parts: Vec<_> = string.splitn(4, ':').collect();

    // path:line:column:match
    anyhow::ensure!(parts.len() == 4);

    Ok(Self {
      path: parts[0].into(),
      loc: Loc::new(parts[1].parse()?, parts[2].parse()?),
      symbol: parts[3].to_string(),
    })
  }
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

impl Fzf {
  pub fn new() -> Result<Self, anyhow::Error> {
    let mut child = Command::new("fzf")
      .args([
        "--ansi",
        "--delimiter=\u{2008}",
        "--nth=2",
        "--with-nth=-2,-1",
        "--reverse",
        "--preview=bat {1} --color always --plain --highlight-line {2} --line-range {4}:+100",
        "--bind=tab:down,shift-tab:up",
      ])
      .stdin(Stdio::piped())
      .spawn()
      .context("spawn")?;
    let stdin = child.stdin.take().context("stdin")?;

    Ok(Self {
      child,
      stdin: Arc::new(Mutex::new(stdin)),
      last_entry_id: Arc::default(),
      entries: Arc::default(),
    })
  }

  pub fn insert_all(&self, rg: Rg) {
    let stdin = self.stdin.clone();
    let last_entry_id = self.last_entry_id.clone();
    let entries = self.entries.clone();

    tokio::spawn(async move {
      let lines = rg.stdout.lines_stream();
      futures::pin_mut!(lines);

      while let Some(line) = lines.next().await {
        let Ok(line) = line else {
          eprintln!("line not ok: {line:?}");
          continue;
        };
        let Ok(entry) = Match::parse(&line) else {
          eprintln!("match parse not ok");
          continue;
        };

        let entry_id = last_entry_id.fetch_add(1, Ordering::Relaxed);

        writeln!(
          stdin.lock(),
          "{path}{SPACE}{line}{SPACE}{line_start}{SPACE}{column}{SPACE}{kind}{SPACE}{symbol}",
          path = entry.path.to_string_lossy(),
          line = entry.loc.line,
          column = entry.loc.column,
          line_start = entry.loc.line.saturating_sub(3),
          kind = rg.symbol.kind.short(),
          symbol = entry.symbol
        )
        .expect("write");

        entries.lock().insert(entry_id, entry);
      }
    });
  }

  pub fn wait(self) -> Result<String, anyhow::Error> {
    let output = self.child.wait_with_output().context("wait")?;
    let selection = String::from_utf8_lossy(&output.stdout)
      .split('/')
      .take(3)
      .collect::<Vec<_>>()
      .join(":");

    Ok(selection)
  }
}
