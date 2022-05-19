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

/// The Shopify API's shop object is a collection of the general settings and information about the shop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "domain")]
  pub domain: String,

  #[serde(rename = "province")]
  pub province: String,

  #[serde(rename = "country")]
  pub country: String,

  #[serde(rename = "address1")]
  pub address1: String,

  #[serde(rename = "zip")]
  pub zip: String,

  #[serde(rename = "city")]
  pub city: String,

  #[serde(rename = "source")]
  pub source: Option<serde_json::Value>,

  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "latitude")]
  pub latitude: f64,

  #[serde(rename = "longitude")]
  pub longitude: f64,

  #[serde(rename = "primary_locale")]
  pub primary_locale: String,

  #[serde(rename = "address2")]
  pub address2: Option<serde_json::Value>,

  #[serde(rename = "created_at")]
  pub created_at: DateTime<Utc>,

  #[serde(rename = "updated_at")]
  pub updated_at: DateTime<Utc>,

  #[serde(rename = "country_code")]
  pub country_code: String,

  #[serde(rename = "country_name")]
  pub country_name: String,

  #[serde(rename = "currency")]
  pub currency: String,

  #[serde(rename = "customer_email")]
  pub customer_email: String,

  #[serde(rename = "timezone")]
  pub timezone: String,

  #[serde(rename = "iana_timezone")]
  pub iana_timezone: String,

  #[serde(rename = "shop_owner")]
  pub shop_owner: String,

  #[serde(rename = "money_format")]
  pub money_format: String,

  #[serde(rename = "money_with_currency_format")]
  pub money_with_currency_format: String,

  #[serde(rename = "weight_unit")]
  pub weight_unit: String,

  #[serde(rename = "province_code")]
  pub province_code: String,

  #[serde(rename = "taxes_included")]
  pub taxes_included: bool,

  #[serde(rename = "auto_configure_tax_inclusivity")]
  pub auto_configure_tax_inclusivity: Option<serde_json::Value>,

  #[serde(rename = "tax_shipping")]
  pub tax_shipping: Option<serde_json::Value>,

  #[serde(rename = "county_taxes")]
  pub county_taxes: bool,

  #[serde(rename = "plan_display_name")]
  pub plan_display_name: String,

  #[serde(rename = "plan_name")]
  pub plan_name: String,

  #[serde(rename = "has_discounts")]
  pub has_discounts: bool,

  #[serde(rename = "has_gift_cards")]
  pub has_gift_cards: bool,

  #[serde(rename = "myshopify_domain")]
  pub myshopify_domain: String,

  #[serde(rename = "google_apps_domain")]
  pub google_apps_domain: Option<serde_json::Value>,

  #[serde(rename = "google_apps_login_enabled")]
  pub google_apps_login_enabled: Option<serde_json::Value>,

  #[serde(rename = "money_in_emails_format")]
  pub money_in_emails_format: String,

  #[serde(rename = "money_with_currency_in_emails_format")]
  pub money_with_currency_in_emails_format: String,

  #[serde(rename = "eligible_for_payments")]
  pub eligible_for_payments: bool,

  #[serde(rename = "requires_extra_payments_agreement")]
  pub requires_extra_payments_agreement: bool,

  #[serde(rename = "password_enabled")]
  pub password_enabled: bool,

  #[serde(rename = "has_storefront")]
  pub has_storefront: bool,

  #[serde(rename = "eligible_for_card_reader_giveaway")]
  pub eligible_for_card_reader_giveaway: bool,

  #[serde(rename = "finances")]
  pub finances: bool,

  #[serde(rename = "primary_location_id")]
  pub primary_location_id: i64,

  #[serde(rename = "cookie_consent_level")]
  pub cookie_consent_level: String,

  #[serde(rename = "visitor_tracking_consent_preference")]
  pub visitor_tracking_consent_preference: String,

  #[serde(rename = "checkout_api_supported")]
  pub checkout_api_supported: bool,

  #[serde(rename = "multi_location_enabled")]
  pub multi_location_enabled: bool,

  #[serde(rename = "setup_required")]
  pub setup_required: bool,

  #[serde(rename = "pre_launch_enabled")]
  pub pre_launch_enabled: bool,

  #[serde(rename = "enabled_presentment_currencies")]
  pub enabled_presentment_currencies: Vec<String>,
}
