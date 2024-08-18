use std::{
  ffi::OsString,
  io::{BufRead, BufReader},
  os::unix::ffi::OsStringExt,
  path::PathBuf,
  process::{Command, Stdio},
  thread::JoinHandle,
};

use anyhow::Context;
use crossbeam::channel::Receiver;

pub struct Fd {
  files: Receiver<PathBuf>,
  handle: JoinHandle<()>,
}

impl Fd {
  /// Spawns an fd process finding all files with the provided `extensions`.
  pub fn new<'a>(extensions: impl IntoIterator<Item = &'a String>) -> Result<Self, anyhow::Error> {
    let extension_args: Vec<&str> = extensions
      .into_iter()
      .flat_map(|ext| vec!["-e", ext.as_str()])
      .collect();

    let mut child = Command::new("fd")
      .args(["-t", "f", "-0"])
      .args(extension_args)
      .stdout(Stdio::piped())
      .spawn()
      .context("spawn")?;

    let (send, recv) = crossbeam::channel::bounded(crate::utils::num_threads());

    let stdout = child.stdout.take().context("stdout")?;

    let handle = std::thread::spawn(move || {
      for line in BufReader::new(stdout).split(b'\0') {
        let line = OsString::from_vec(line.expect("failed to get line"));

        send.send(PathBuf::from(line)).expect("failed to send");
      }
    });

    Ok(Self { files: recv, handle })
  }

  /// Returns the channel of files outputted by fd.
  pub fn files(&self) -> &Receiver<PathBuf> {
    &self.files
  }
}
