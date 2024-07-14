use std::{path::PathBuf, thread::JoinHandle};

use crossbeam::channel::Receiver;

use crate::{
  config::Config,
  fzf::{Fzf, Sink},
};

pub struct Worker {
  config: Config,
  files: Receiver<PathBuf>,
  fzf: Sink,
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

    std::thread::spawn(move || {
      while let Ok(file) = files.recv() {
        eprintln!("handling {file:?}");
      }
    })
  }
}
