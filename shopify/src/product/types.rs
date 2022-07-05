use serde_json::Value;
use crate::types::{Date};
pub use crate::variant::Variant;

#[cfg(feature = "openapi")]
use schemars::JsonSchema;

#[cfg(feature = "sqlx")]
use sqlx::FromRow;


#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Product {
  pub id: i64,
  pub title: String,
  pub body_html: String,
  pub vendor: String,
  pub product_type: String,
  pub created_at: Date,
  pub handle: String,
  pub updated_at: Option<Date>,
  pub published_at: Option<Date>,
  pub template_suffix: Value,
  pub tags: String,
  pub published_scope: String,
  pub variants: Vec<Variant>,
  pub options: Vec<ProductOption>,
  pub images: Vec<Image>,
  pub image: Option<Image>,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Image {
  pub id: i64,
  pub product_id: i64,
  pub position: i64,
  pub created_at: String,
  pub updated_at: Option<Date>,
  pub alt: Option<String>,
  pub width: i64,
  pub height: i64,
  pub src: String,
  pub variant_ids: Vec<i64>,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ProductOption {
  pub id: i64,
  pub product_id: i64,
  pub name: String,
  pub position: i64,
  pub values: Vec<String>,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ProductArg {
  pub title: String,
  pub body_html: String,
  pub vendor: String,
  pub product_type: String,
  pub tags: Vec<String>
}