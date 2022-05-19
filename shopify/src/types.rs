use std::{collections::HashMap, fmt::Display};

pub use chrono::{DateTime, Utc};
pub use serde_json::Value;

#[macro_export]
macro_rules! map {
    ($($v: literal => $s: expr),*) => {{
            let mut h = HashMap::new();
            $(
                h.insert($v,$s);
            )*
            h
    }}
}

pub struct Query(pub String);

impl<V: Display> Into<Query> for HashMap<&str, V> {
  fn into(self) -> Query {
    let entries = self.iter();

    let mut query = String::new();

    for (key, value) in entries {
      query.push_str(&format!("{}={}&", key, value));
    }

    Query(query)
  }
}
