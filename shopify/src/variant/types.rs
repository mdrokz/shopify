use schemars::JsonSchema;

use crate::types::{Date};

#[derive(Debug, Serialize, Deserialize,Default,JsonSchema)]
pub struct Variant {
  pub id: i64,
  pub product_id: i64,
  pub title: String,
  pub price: String,
  pub sku: String,
  pub position: i64,
  pub inventory_policy: String,
  pub compare_at_price: Option<String>,
  pub fulfillment_service: Option<String>,
  pub inventory_management: Option<String>,
  pub option1: Option<String>,
  pub option2: Option<String>,
  pub option3: Option<String>,
  pub created_at: Date,
  pub updated_at: Date,
  pub taxable: bool,
  pub barcode: Option<String>,
  pub grams: i64,
  pub image_id: Option<i64>,
  pub inventory_quantity: i64,
  pub weight: f64,
  pub weight_unit: String,
  pub inventory_item_id: i64,
  pub old_inventory_quantity: i64,
  pub requires_shipping: bool,
}
