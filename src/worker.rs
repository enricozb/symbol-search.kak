use std::{path::PathBuf, thread::JoinHandle, time::SystemTime};

use anyhow::Context;
use crossbeam::channel::Receiver;

use crate::{
  cache::Cache,
  config::Config,
  fzf::{Entry, Fzf, Sink},
  parser::Parser,
};

pub struct Worker {
  config: &'static Config,
  cache: Cache,
  files: Receiver<PathBuf>,
  fzf: Sink,
}

impl Worker {
  pub fn new(config: &'static Config, cache: &Cache, files: &Receiver<PathBuf>, fzf: &Fzf) -> Self {
    Self {
      config,
      cache: cache.clone(),
      files: files.clone(),
      fzf: fzf.sink(),
    }
  }

  pub fn run(self) -> JoinHandle<()> {
    std::thread::spawn(move || {
      while let Ok(path) = self.files.recv() {
        let modified = std::fs::metadata(&path).expect("metadata").modified().expect("modified");

        if self.use_cached_entries(&path, modified).expect("cached") {
          continue;
        }

        self.parse_file(&path, modified).expect("parse file");
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
        for Entry { loc, text, kind, .. } in &file_info.entries {
          // cached entries don't contain paths so they are re-inserted here.
          self.fzf.send(&Entry::new(&path, *loc, text, *kind)).context("send")?;
        }

        return Ok(true);
      }
    }

    Ok(false)
  }

  /// Parses a file and inserts its entries into the cache.
  fn parse_file(&self, path: &PathBuf, modified: SystemTime) -> Result<(), anyhow::Error> {
    self.cache.new_file_info(path.clone(), modified);

    let Some(parser) = Parser::from_path(self.config, &path) else {
      return Ok(());
    };

    parser.on_symbol(|symbol| {
      let entry = Entry::new(path, symbol.span.start, symbol.text, symbol.kind);

      self.fzf.send(&entry).context("send")?;

      self.cache.insert_entry(path, entry);

      Ok(())
    })
  }
}
