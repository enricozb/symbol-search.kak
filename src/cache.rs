use std::{
  collections::HashMap,
  fs::File,
  ops::Deref,
  path::{Path, PathBuf},
  sync::Arc,
  time::SystemTime,
};

use anyhow::Context;
use parking_lot::{lock_api::RwLockReadGuard, RwLock};
use serde::{Deserialize, Serialize};

use crate::{ext::ResultExt, fzf::Entry};

#[derive(Clone, Default)]
pub struct Cache {
  path: Option<PathBuf>,
  files: Arc<RwLock<HashMap<PathBuf, FileInfo>>>,
}

const CACHE_FILE_NAME: &str = "cache.json";

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
  pub modified: SystemTime,
  /// Cached entries don't contain their own path buffers as it is already
  /// stored in the [`Cache::files`] field.
  pub entries: Vec<Entry<(), String>>,
}

impl Cache {
  /// Read a cache from a directory containing the cache.
  ///
  /// If the directory does not exist or does not contain the cache file,
  /// the directory and file are created, and a default cache is returned.
  pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
    let path = path.as_ref().join(CACHE_FILE_NAME);

    if !path.exists() {
      std::fs::create_dir_all(path.parent().context("parent")?).context("create dir")?;

      return Ok(Self {
        path: Some(path.clone()),
        files: Arc::default(),
      });
    }

    let file = File::open(&path).context("open")?;

    Ok(Self {
      path: Some(path.clone()),
      files: Arc::new(RwLock::new(serde_json::from_reader(file).context("failed to parse cache").warn())),
    })
  }

  /// Returns the [`FileInfo`] for file at a given path, if any.
  pub fn file_info(&self, path: &PathBuf) -> Option<impl Deref<Target = FileInfo> + '_> {
    RwLockReadGuard::try_map(self.files.read(), |files| files.get(path)).ok()
  }

  /// Inserts a new [`FileInfo`] for a file at a given path.
  pub fn new_file_info(&self, path: PathBuf, modified: SystemTime) {
    self.files.write().insert(
      path,
      FileInfo {
        modified,
        entries: Vec::new(),
      },
    );
  }

  /// Inserts a new [`Entry`] for a file at a given path.
  ///
  /// [`new_file_info`] must be called first.
  pub fn insert_entry<P, S: Into<String>>(&self, path: &Path, entry: Entry<P, S>) {
    self
      .files
      .write()
      .get_mut(path)
      .unwrap()
      .entries
      .push(Entry::new((), entry.loc, entry.symbol.into(), entry.kind));
  }

  /// Save a cache to its path.
  pub fn save(&self) -> Result<(), anyhow::Error> {
    let Some(path) = &self.path else {
      return Ok(());
    };

    let json = serde_json::to_string(&*self.files.read()).context("to_string")?;

    std::fs::write(path, json).context("write")
  }
}

impl Drop for Cache {
  fn drop(&mut self) {
    self.save().warn();
  }
}
