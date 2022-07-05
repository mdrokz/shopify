use crate::types::{Date};
use serde_json::Value;

#[cfg(feature = "openapi")]
use schemars::JsonSchema;

#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Location {
  pub id: i64,
  pub name: String,
  pub address1: Option<String>,
  pub address2: Option<String>,
  pub city: Option<String>,
  pub zip: Option<String>,
  pub province: Option<String>,
  pub country: Option<String>,
  pub phone: Option<String>,
  pub country_code: Option<String>,
  pub country_name: Option<String>,
  pub province_code: Option<String>,
  pub legacy: bool,
  pub active: bool,
  pub created_at: String,
  pub updated_at: String,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct InventoryLevel {
  pub inventory_item_id: i64,
  pub location_id: i64,
  pub available: Value,
  pub admin_graphql_api_id: String,
  pub updated_at: Date,
}
