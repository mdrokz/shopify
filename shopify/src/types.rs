use std::{collections::HashMap, fmt::Display};

pub use chrono::{DateTime, Utc};
pub use serde_json::Value;

pub const SESSION_KEY: &str = "shopify_app_session";

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

use schemars::{schema::Schema, schema_for_value, JsonSchema};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Date(pub DateTime<Utc>);

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct UID(pub uuid::Uuid);

impl Default for Date {
  fn default() -> Self {
    Self(Utc::now())
  }
}

impl Default for UID {
  fn default() -> Self {
    Self(uuid::Uuid::new_v4())
  }
}

impl JsonSchema for UID {
  fn schema_name() -> String {
    "Unique ID".into()
  }

  fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let root_schema = schema_for_value!(UID::default());
    Schema::Object(root_schema.schema)
  }
}

impl JsonSchema for Date {
  fn schema_name() -> String {
    "UTC DateTime".into()
  }

  fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let root_schema = schema_for_value!(Date::default());
    Schema::Object(root_schema.schema)
  }
}
