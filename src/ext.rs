use std::fmt::Debug;

#[extend::ext(name=ResultExt)]
pub impl<T, E: Debug> Result<T, E> {
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

#[extend::ext(name=AnyExt)]
pub impl<T> T {
  fn ok<E>(self) -> Result<T, E> {
    Ok(self)
  }
}
