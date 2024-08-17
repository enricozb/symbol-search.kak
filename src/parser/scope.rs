use serde::{de::Error, Deserialize, Deserializer};
use syntect::parsing::Scope;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ScopeExpr {
  pub scope: Scope,
  pub exclude: Option<Scope>,
}

impl<'de> Deserialize<'de> for ScopeExpr {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let parts: Vec<&str> = s.split('-').map(str::trim).collect();

    match parts.as_slice() {
      [scope] => Ok(ScopeExpr {
        scope: Scope::new(scope).map_err(D::Error::custom)?,
        exclude: None,
      }),
      [scope, exclude] => Ok(ScopeExpr {
        scope: Scope::new(scope).map_err(D::Error::custom)?,
        exclude: Some(Scope::new(exclude).map_err(serde::de::Error::custom)?),
      }),
      _ => Err(serde::de::Error::custom("Invalid scope expression format")),
    }
  }
}
