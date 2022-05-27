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

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerParams {
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
    pub addresses: Vec<AddressParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressParams {
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



#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "accepts_marketing")]
  pub accepts_marketing: bool,

  #[serde(rename = "created_at")]
  pub created_at: String,

  #[serde(rename = "updated_at")]
  pub updated_at: String,

  #[serde(rename = "first_name")]
  pub first_name: String,

  #[serde(rename = "last_name")]
  pub last_name: String,

  #[serde(rename = "orders_count")]
  pub orders_count: i64,

  #[serde(rename = "state")]
  pub state: String,

  #[serde(rename = "total_spent")]
  pub total_spent: String,

  #[serde(rename = "last_order_id")]
  pub last_order_id: i64,

  #[serde(rename = "note")]
  pub note: Option<serde_json::Value>,

  #[serde(rename = "verified_email")]
  pub verified_email: bool,

  #[serde(rename = "multipass_identifier")]
  pub multipass_identifier: Option<serde_json::Value>,

  #[serde(rename = "tax_exempt")]
  pub tax_exempt: bool,

  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "tags")]
  pub tags: String,

  #[serde(rename = "last_order_name")]
  pub last_order_name: String,

  #[serde(rename = "currency")]
  pub currency: String,

  #[serde(rename = "addresses")]
  pub addresses: Vec<Address>,

  #[serde(rename = "accepts_marketing_updated_at")]
  pub accepts_marketing_updated_at: String,

  #[serde(rename = "marketing_opt_in_level")]
  pub marketing_opt_in_level: Option<serde_json::Value>,

  #[serde(rename = "tax_exemptions")]
  pub tax_exemptions: Vec<Option<serde_json::Value>>,

  #[serde(rename = "admin_graphql_api_id")]
  pub admin_graphql_api_id: String,

  #[serde(rename = "default_address")]
  pub default_address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "customer_id")]
  pub customer_id: i64,

  #[serde(rename = "first_name")]
  pub first_name: Option<serde_json::Value>,

  #[serde(rename = "last_name")]
  pub last_name: Option<serde_json::Value>,

  #[serde(rename = "company")]
  pub company: Option<serde_json::Value>,

  #[serde(rename = "address1")]
  pub address1: String,

  #[serde(rename = "address2")]
  pub address2: String,

  #[serde(rename = "city")]
  pub city: String,

  #[serde(rename = "province")]
  pub province: String,

  #[serde(rename = "country")]
  pub country: String,

  #[serde(rename = "zip")]
  pub zip: String,

  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "province_code")]
  pub province_code: String,

  #[serde(rename = "country_code")]
  pub country_code: String,

  #[serde(rename = "country_name")]
  pub country_name: String,

  #[serde(rename = "default")]
  pub address_default: bool,
}
