/// `July21("2021-07")`
/// 
/// `October21("2021-10")`
/// 
/// `January22("2022-01")`
/// 
/// `April22("2022-04")`
/// 
/// `Nightly("unstable")`

pub enum ApiVersion {
  July21(String),    /// "2021-07",
  October21(String), // "2021-10",
  January22(String), // "2022-01",
  April22(String),   // "2022-04",
  Nightly(String),   // "unstable"
}

impl Default for ApiVersion {
  fn default() -> Self {
    Self::Nightly(String::from("unstable"))
  }
}
