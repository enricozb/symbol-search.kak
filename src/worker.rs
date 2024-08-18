use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader},
  path::PathBuf,
  thread::JoinHandle,
};

use crossbeam::channel::Receiver;
use syntect::parsing::Scope;

use crate::{
  cache::Cache,
  config::Config,
  ext::ResultExt,
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
    let Self {
      config,
      cache,
      files,
      fzf,
    } = self;

    let mut scope_kinds = ScopeKinds::new();

    // TODO(enricozb):
    // - shorten this spawn call / extract into a function
    // - have the new function return an error (remove expect / unwrap)
    // - catch and eprint it here
    std::thread::spawn(move || {
      while let Ok(path) = files.recv() {
        let path_modified = std::fs::metadata(&path)
          .expect("metadata")
          .modified()
          .expect("modified");

        if let Some(file_info) = cache.file_info(&path) {
          // if the cached file and the current file have the same modified timestamp,
          // use the entries from the cache.
          if path_modified == file_info.modified {
            for Entry { loc, symbol, kind, .. } in &file_info.entries {
              // cached entries don't contain paths so they are re-inserted here.
              fzf.send(&Entry::new(&path, *loc, symbol, *kind)).warn();
            }

            continue;
          }
        }

        cache.new_file_info(path.clone(), path_modified);

        let Some(mut parser) = config.parser_for_file(&path) else {
          continue;
        };

        let Ok(file) = File::open(&path) else {
          continue;
        };

        let mut lines = BufReader::new(file).lines();

        while let Some(Ok(line)) = lines.next() {
          if let Ok(matches) = parser.parse_line(&line) {
            for (span, scope, symbol) in matches {
              let entry = Entry::new(&path, span.start, symbol, scope_kinds.kind(scope));

              fzf.send(&entry).warn();

              cache.insert_entry(&path, entry);
            }
          }
        }
      }
    })
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
