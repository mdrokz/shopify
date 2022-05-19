use crate::types::{DateTime, Utc, Value};

/// The Shopify API's shop object is a collection of the general settings and information about the shop.
/// // Example code that deserializes and serializes the model.
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

/// The Shopify API's shop object is a collection of the general settings and information about the shop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
  shop: ShopData,
}
/// The Shopify API's shop object is a collection of the general settings and information about the shop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopData {
  id: i64,
  name: String,
  email: String,
  domain: String,
  province: String,
  country: String,
  address1: String,
  zip: String,
  city: String,
  source: Option<serde_json::Value>,
  phone: String,
  latitude: f64,
  longitude: f64,
  primary_locale: String,
  address2: Option<serde_json::Value>,
  created_at: String,
  updated_at: String,
  country_code: String,
  country_name: String,
  currency: String,
  customer_email: String,
  timezone: String,
  iana_timezone: String,
  shop_owner: String,
  money_format: String,
  money_with_currency_format: String,
  weight_unit: String,
  province_code: String,
  taxes_included: bool,
  auto_configure_tax_inclusivity: Option<serde_json::Value>,
  tax_shipping: Option<serde_json::Value>,
  county_taxes: bool,
  plan_display_name: String,
  plan_name: String,
  has_discounts: bool,
  has_gift_cards: bool,
  myshopify_domain: String,
  google_apps_domain: Option<serde_json::Value>,
  google_apps_login_enabled: Option<serde_json::Value>,
  money_in_emails_format: String,
  money_with_currency_in_emails_format: String,
  eligible_for_payments: bool,
  requires_extra_payments_agreement: bool,
  password_enabled: bool,
  has_storefront: bool,
  eligible_for_card_reader_giveaway: bool,
  finances: bool,
  primary_location_id: i64,
  cookie_consent_level: String,
  visitor_tracking_consent_preference: String,
  checkout_api_supported: bool,
  multi_location_enabled: bool,
  setup_required: bool,
  pre_launch_enabled: bool,
  enabled_presentment_currencies: Vec<String>,
}