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

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct CustomerAddress {
  #[serde(rename = "address1")]
  pub address1: String,

  #[serde(rename = "address2")]
  pub address2: String,

  #[serde(rename = "city")]
  pub city: String,

  #[serde(rename = "country")]
  pub country: String,

  #[serde(rename = "country_code")]
  pub country_code: String,

  #[serde(rename = "country_name")]
  pub country_name: String,

  #[serde(rename = "company")]
  pub company: String,

  #[serde(rename = "customer_id")]
  pub customer_id: CustomerId,

  #[serde(rename = "first_name")]
  pub first_name: String,

  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "last_name")]
  pub last_name: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "province")]
  pub province: String,

  #[serde(rename = "province_code")]
  pub province_code: String,

  #[serde(rename = "zip")]
  pub zip: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct CustomerId {
  #[serde(rename = "id")]
  pub id: i64,
}
