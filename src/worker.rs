use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader},
  path::PathBuf,
  thread::JoinHandle,
  time::SystemTime,
};

use anyhow::Context;
use crossbeam::channel::Receiver;
use syntect::parsing::Scope;

use crate::{
  cache::Cache,
  config::Config,
  fzf::{Entry, Fzf, Sink, SymbolKind},
};

pub struct Worker {
  config: Config,
  cache: Cache,
  files: Receiver<PathBuf>,
  fzf: Sink,
}

impl Worker {
  pub fn new(config: &Config, cache: &Cache, files: &Receiver<PathBuf>, fzf: &Fzf) -> Self {
    Self {
      config: config.clone(),
      cache: cache.clone(),
      files: files.clone(),
      fzf: fzf.sink(),
    }
  }

  pub fn run(self) -> JoinHandle<()> {
    std::thread::spawn(move || {
      let mut scope_kinds = ScopeKinds::new();

      while let Ok(path) = self.files.recv() {
        let modified = std::fs::metadata(&path).expect("metadata").modified().expect("modified");

        if self.use_cached_entries(&path, modified).expect("cached") {
          continue;
        }

        self.parse_file(&path, modified, &mut scope_kinds).expect("parse file");
      }
    })
  }

  /// Attempts to use the cache to compute a paths entries.
  ///
  /// Returns true if the cache's entries were used.
  fn use_cached_entries(&self, path: &PathBuf, modified: SystemTime) -> Result<bool, anyhow::Error> {
    if let Some(file_info) = self.cache.file_info(path) {
      // if the cached file and the current file have the same modified timestamp,
      // use the entries from the cache.
      if modified == file_info.modified {
        for Entry { loc, symbol, kind, .. } in &file_info.entries {
          // cached entries don't contain paths so they are re-inserted here.
          self.fzf.send(&Entry::new(&path, *loc, symbol, *kind)).context("send")?;
        }

        return Ok(true);
      }
    }

    Ok(false)
  }

  /// Parses a file and inserts its entries into the cache.
  fn parse_file(&self, path: &PathBuf, modified: SystemTime, scope_kinds: &mut ScopeKinds) -> Result<(), anyhow::Error> {
    self.cache.new_file_info(path.clone(), modified);

    let Some(mut parser) = self.config.parser_for_file(path) else {
      return Ok(());
    };

    let Ok(file) = File::open(path) else {
      return Ok(());
    };

    let mut lines = BufReader::new(file).lines();

    while let Some(Ok(line)) = lines.next() {
      if let Ok(matches) = parser.parse_line(&line) {
        for (span, scope, symbol) in matches {
          let entry = Entry::new(path, span.start, symbol, scope_kinds.kind(scope));

          self.fzf.send(&entry).context("send")?;

          self.cache.insert_entry(path, entry);
        }
      }
    }

    Ok(())
  }
}

/// A cache for computing [`SymbolKind`] from scopes.
///
/// Computing a [`Scope`]'s [`SymbolKind`] requires converting it to a string, which is expensive.
/// A scope's symbol kind is computed from its penultimate part. For example,
/// `entity.name.type.class.tsx` is a `SymbolKind::Class`.
struct ScopeKinds {
  cache: HashMap<Scope, SymbolKind>,
}

impl ScopeKinds {
  fn new() -> Self {
    Self { cache: HashMap::new() }
  }

  #[rustfmt::skip]
  fn kind(&mut self, scope: Scope) -> SymbolKind {
    *self.cache.entry(scope).or_insert_with_key(|scope| {
      let scope = scope.to_string();
      let Some(name) = scope.split('.').rev().nth(1) else {
        return SymbolKind::Unknown;
      };

      match name {
        "module"    => SymbolKind::Module,
        "macro"     => SymbolKind::Macro,
        "constant"  => SymbolKind::Constant,

        "class"     => SymbolKind::Class,
        "struct"    => SymbolKind::Struct,
        "enum"      => SymbolKind::Enum,
        "union"     => SymbolKind::Union,

        "alias"     => SymbolKind::Alias,
        "interface" => SymbolKind::Interface,
        "trait"     => SymbolKind::Trait,
        "type"      => SymbolKind::Type,

        "function"  => SymbolKind::Function,
        "impl"      => SymbolKind::Impl,

        _ => SymbolKind::Unknown,
      }
    })
  }
}
