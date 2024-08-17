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
  config::Config,
  ext::ResultExt,
  fzf::{Entry, Fzf, Sink, SymbolKind},
};

pub struct Worker {
  config: Config,
  files: Receiver<PathBuf>,
  fzf: Sink,
}

struct ScopeKinds {
  cache: HashMap<Scope, SymbolKind>,
}

impl Worker {
  pub fn new(config: &Config, files: &Receiver<PathBuf>, fzf: &Fzf) -> Self {
    Self {
      config: config.clone(),
      files: files.clone(),
      fzf: fzf.sink(),
    }
  }

  pub fn run(self) -> JoinHandle<()> {
    let Self { config, files, fzf } = self;

    let mut scope_kinds = ScopeKinds::new();

    std::thread::spawn(move || {
      while let Ok(path) = files.recv() {
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
              fzf
                .send(&Entry::new(&path, span.start, symbol, scope_kinds.kind(scope)))
                .warn();
            }
          }
        }
      }
    })
  }
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

        // "interface" => SymbolKind::Class,
        _ => SymbolKind::Unknown,
      }
    })
  }
}
