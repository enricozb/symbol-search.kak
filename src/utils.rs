use crate::ext::ResultExt;

const DEFAULT_NUM_THREADS: usize = 8;

pub fn num_threads() -> usize {
  std::thread::available_parallelism()
    .map(usize::from)
    .warn_with(DEFAULT_NUM_THREADS)
}
