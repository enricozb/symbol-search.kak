use std::{collections::HashSet, hash::Hash};

use futures::{Stream, StreamExt};
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio_stream::wrappers::LinesStream;

#[extend::ext(name=AsyncReadExt)]
pub impl<T: AsyncRead> T {
  fn lines_stream(self) -> impl Stream<Item = Result<String, std::io::Error>> {
    LinesStream::new(BufReader::new(self).lines())
  }
}

#[extend::ext(name=UniqueExt)]
pub impl<I: Eq + Hash + Clone, T: Stream<Item = I>> T {
  fn unique(self) -> impl Stream<Item = I> {
    let mut seen = HashSet::new();

    self.filter_map(move |x| {
      if seen.contains(&x) {
        futures::future::ready(None)
      } else {
        seen.insert(x.clone());
        futures::future::ready(Some(x))
      }
    })
  }
}
