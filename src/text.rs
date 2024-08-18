use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Loc {
  pub line: usize,
  pub column: usize,
}

impl Loc {
  pub fn new(line: usize, column: usize) -> Self {
    Self { line, column }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Span {
  pub start: Loc,
  pub end: Loc,
}

impl Span {
  pub fn new(start: Loc, end: Loc) -> Self {
    Self { start, end }
  }
}
