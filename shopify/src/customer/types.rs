// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[cfg(feature = "openapi")]
use schemars::JsonSchema;

#[cfg(feature = "sqlx")]
use sqlx::{FromRow};

// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }
#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CustomerCount {
  pub count: i64,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CustomerArg {
  #[serde(rename = "first_name")]
  pub first_name: String,

  #[serde(rename = "last_name")]
  pub last_name: String,

  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "verified_email")]
  pub verified_email: bool,

  #[serde(rename = "addresses")]
  pub addresses: Vec<AddressArg>,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddressArg {
  #[serde(rename = "address1")]
  pub address1: String,

  #[serde(rename = "city")]
  pub city: String,

  #[serde(rename = "province")]
  pub province: String,

  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "zip")]
  pub zip: String,

  #[serde(rename = "last_name")]
  pub last_name: String,

  #[serde(rename = "first_name")]
  pub first_name: String,

  #[serde(rename = "country")]
  pub country: String,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Customer {
  pub id: i64,
  pub email: String,
  pub accepts_marketing: bool,
  pub created_at: String,
  pub updated_at: String,
  pub first_name: String,
  pub last_name: String,
  pub orders_count: i64,
  pub state: String,
  pub total_spent: String,
  pub last_order_id: Option<serde_json::Value>,
  pub note: Option<serde_json::Value>,
  pub verified_email: bool,
  pub multipass_identifier: Option<serde_json::Value>,
  pub tax_exempt: bool,
  pub phone: String,
  pub tags: String,
  pub last_order_name: Option<serde_json::Value>,
  pub currency: String,
  pub addresses: Vec<Address>,
  pub accepts_marketing_updated_at: String,
  pub marketing_opt_in_level: Option<serde_json::Value>,
  pub tax_exemptions: Vec<Option<serde_json::Value>>,
  pub email_marketing_consent: MarketingConsent,
  pub sms_marketing_consent: MarketingConsent,
  pub admin_graphql_api_id: String,
  pub default_address: Address,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Address {
  pub id: i64,
  pub customer_id: i64,
  pub first_name: String,
  pub last_name: String,
  pub company: Option<serde_json::Value>,
  pub address1: String,
  pub address2: Option<serde_json::Value>,
  pub city: String,
  pub province: String,
  pub country: String,
  pub zip: String,
  pub phone: String,
  pub name: String,
  pub province_code: String,
  pub country_code: String,
  pub country_name: String,
  #[serde(rename = "default")]
  pub address_default: bool,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MarketingConsent {
  pub state: String,
  pub opt_in_level: String,
  pub consent_updated_at: Option<serde_json::Value>,
  pub consent_collected_from: Option<String>,
}
