use std::fmt::Debug;

#[extend::ext(name=ResultExt)]
pub impl<T, E: Debug> Result<T, E> {
  fn warn(self) -> T
  where
    T: Default,
  {
    self.warn_with(T::default())
  }

  fn warn_with(self, value: T) -> T {
    match self {
      Ok(t) => t,
      Err(e) => {
        eprintln!("{e:?}");

        value
      }
    }
  }
}
