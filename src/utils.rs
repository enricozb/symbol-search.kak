use serde::Deserialize;

use crate::ext::ResultExt;

const DEFAULT_NUM_THREADS: usize = 8;

pub fn num_threads() -> usize {
  std::thread::available_parallelism().map(usize::from).warn_with(DEFAULT_NUM_THREADS)
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
  One(T),
  Many(Vec<T>),
}

impl<T> From<OneOrMany<T>> for Vec<T> {
  fn from(from: OneOrMany<T>) -> Self {
    match from {
      OneOrMany::One(val) => vec![val],
      OneOrMany::Many(vec) => vec,
    }
  }
}
