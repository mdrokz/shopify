use std::fmt::{Display, Formatter};

/// `July21("2021-07")`
///
/// `July20("2020-07")`
///
/// `October21("2021-10")`
///
/// `January22("2022-01")`
///
/// `April22("2022-04")`
///
/// `Nightly("unstable")`

#[derive(Debug, Clone)]
pub enum ApiVersion {
  July20(String),
  July21(String),
  October21(String),
  January22(String),
  April22(String),
  Nightly(String),
}

impl Display for ApiVersion {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    match self {
      ApiVersion::July20(v) => write!(f, "{}", v)?,
      ApiVersion::July21(v) => write!(f, "{}", v)?,
      ApiVersion::October21(v) => write!(f, "{}", v)?,
      ApiVersion::January22(v) => write!(f, "{}", v)?,
      ApiVersion::April22(v) => write!(f, "{}", v)?,
      ApiVersion::Nightly(v) => write!(f, "{}", v)?,
    };

    Ok(())
  }
}

impl Default for ApiVersion {
  fn default() -> Self {
    Self::Nightly(String::from("unstable"))
  }
}
