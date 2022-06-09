use schemars::JsonSchema;
use serde_json::Value;
use crate::types::{Date};
pub use crate::variant::Variant;

#[derive(Debug, Serialize, Deserialize,Default,JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize,Default,JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize,Default,JsonSchema)]
pub struct ProductOption {
  pub id: i64,
  pub product_id: i64,
  pub name: String,
  pub position: i64,
  pub values: Vec<String>,
}
