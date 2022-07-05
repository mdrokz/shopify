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
use sqlx::FromRow;

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Order {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "admin_graphql_api_id")]
    pub admin_graphql_api_id: String,

    #[serde(rename = "app_id")]
    pub app_id: i64,

    #[serde(rename = "browser_ip")]
    pub browser_ip: Option<serde_json::Value>,

    #[serde(rename = "buyer_accepts_marketing")]
    pub buyer_accepts_marketing: bool,

    #[serde(rename = "cancel_reason")]
    pub cancel_reason: Option<serde_json::Value>,

    #[serde(rename = "cancelled_at")]
    pub cancelled_at: Option<serde_json::Value>,

    #[serde(rename = "cart_token")]
    pub cart_token: Option<serde_json::Value>,

    #[serde(rename = "checkout_id")]
    pub checkout_id: Option<serde_json::Value>,

    #[serde(rename = "checkout_token")]
    pub checkout_token: Option<serde_json::Value>,

    #[serde(rename = "closed_at")]
    pub closed_at: Option<serde_json::Value>,

    #[serde(rename = "confirmed")]
    pub confirmed: bool,

    #[serde(rename = "contact_email")]
    pub contact_email: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "currency")]
    pub currency: Currency,

    #[serde(rename = "current_subtotal_price")]
    pub current_subtotal_price: String,

    #[serde(rename = "current_subtotal_price_set")]
    pub current_subtotal_price_set: Set,

    #[serde(rename = "current_total_discounts")]
    pub current_total_discounts: String,

    #[serde(rename = "current_total_discounts_set")]
    pub current_total_discounts_set: Set,

    #[serde(rename = "current_total_duties_set")]
    pub current_total_duties_set: Option<serde_json::Value>,

    #[serde(rename = "current_total_price")]
    pub current_total_price: String,

    #[serde(rename = "current_total_price_set")]
    pub current_total_price_set: Set,

    #[serde(rename = "current_total_tax")]
    pub current_total_tax: String,

    #[serde(rename = "current_total_tax_set")]
    pub current_total_tax_set: Set,

    #[serde(rename = "customer_locale")]
    pub customer_locale: Option<serde_json::Value>,

    #[serde(rename = "device_id")]
    pub device_id: Option<serde_json::Value>,

    #[serde(rename = "discount_codes")]
    pub discount_codes: Vec<Option<serde_json::Value>>,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "estimated_taxes")]
    pub estimated_taxes: bool,

    #[serde(rename = "financial_status")]
    pub financial_status: String,

    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: Option<serde_json::Value>,

    #[serde(rename = "gateway")]
    pub gateway: String,

    #[serde(rename = "landing_site")]
    pub landing_site: Option<serde_json::Value>,

    #[serde(rename = "landing_site_ref")]
    pub landing_site_ref: Option<serde_json::Value>,

    #[serde(rename = "location_id")]
    pub location_id: Option<serde_json::Value>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "note")]
    pub note: Option<serde_json::Value>,

    #[serde(rename = "note_attributes")]
    pub note_attributes: Vec<Option<serde_json::Value>>,

    #[serde(rename = "number")]
    pub number: i64,

    #[serde(rename = "order_number")]
    pub order_number: i64,

    #[serde(rename = "order_status_url")]
    pub order_status_url: String,

    #[serde(rename = "original_total_duties_set")]
    pub original_total_duties_set: Option<serde_json::Value>,

    #[serde(rename = "payment_gateway_names")]
    pub payment_gateway_names: Vec<Option<serde_json::Value>>,

    #[serde(rename = "phone")]
    pub phone: Option<serde_json::Value>,

    #[serde(rename = "presentment_currency")]
    pub presentment_currency: Currency,

    #[serde(rename = "processed_at")]
    pub processed_at: String,

    #[serde(rename = "processing_method")]
    pub processing_method: String,

    #[serde(rename = "reference")]
    pub reference: Option<serde_json::Value>,

    #[serde(rename = "referring_site")]
    pub referring_site: Option<serde_json::Value>,

    #[serde(rename = "source_identifier")]
    pub source_identifier: Option<serde_json::Value>,

    #[serde(rename = "source_name")]
    pub source_name: String,

    #[serde(rename = "source_url")]
    pub source_url: Option<serde_json::Value>,

    #[serde(rename = "subtotal_price")]
    pub subtotal_price: String,

    #[serde(rename = "subtotal_price_set")]
    pub subtotal_price_set: Set,

    #[serde(rename = "tags")]
    pub tags: String,

    #[serde(rename = "tax_lines")]
    pub tax_lines: Vec<Option<serde_json::Value>>,

    #[serde(rename = "taxes_included")]
    pub taxes_included: bool,

    #[serde(rename = "test")]
    pub test: bool,

    #[serde(rename = "token")]
    pub token: String,

    #[serde(rename = "total_discounts")]
    pub total_discounts: String,

    #[serde(rename = "total_discounts_set")]
    pub total_discounts_set: Set,

    #[serde(rename = "total_line_items_price")]
    pub total_line_items_price: String,

    #[serde(rename = "total_line_items_price_set")]
    pub total_line_items_price_set: Set,

    #[serde(rename = "total_outstanding")]
    pub total_outstanding: String,

    #[serde(rename = "total_price")]
    pub total_price: String,

    #[serde(rename = "total_price_set")]
    pub total_price_set: Set,

    #[serde(rename = "total_price_usd")]
    pub total_price_usd: String,

    #[serde(rename = "total_shipping_price_set")]
    pub total_shipping_price_set: Set,

    #[serde(rename = "total_tax")]
    pub total_tax: String,

    #[serde(rename = "total_tax_set")]
    pub total_tax_set: Set,

    #[serde(rename = "total_tip_received")]
    pub total_tip_received: String,

    #[serde(rename = "total_weight")]
    pub total_weight: i64,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "user_id")]
    pub user_id: Option<serde_json::Value>,

    #[serde(rename = "customer")]
    pub customer: Customer,

    #[serde(rename = "discount_applications")]
    pub discount_applications: Vec<Option<serde_json::Value>>,

    #[serde(rename = "fulfillments")]
    pub fulfillments: Vec<Option<serde_json::Value>>,

    #[serde(rename = "line_items")]
    pub line_items: Vec<LineItem>,

    #[serde(rename = "payment_terms")]
    pub payment_terms: Option<serde_json::Value>,

    #[serde(rename = "refunds")]
    pub refunds: Vec<Option<serde_json::Value>>,

    #[serde(rename = "shipping_lines")]
    pub shipping_lines: Vec<Option<serde_json::Value>>,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Set {
    #[serde(rename = "shop_money")]
    pub shop_money: Money,

    #[serde(rename = "presentment_money")]
    pub presentment_money: Money,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Money {
    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "currency_code")]
    pub currency_code: Currency,
}


#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    pub currency: Currency,

    #[serde(rename = "accepts_marketing_updated_at")]
    pub accepts_marketing_updated_at: String,

    #[serde(rename = "marketing_opt_in_level")]
    pub marketing_opt_in_level: Option<serde_json::Value>,

    #[serde(rename = "tax_exemptions")]
    pub tax_exemptions: Vec<Option<serde_json::Value>>,

    #[serde(rename = "email_marketing_consent")]
    pub email_marketing_consent: MarketingConsent,

    #[serde(rename = "sms_marketing_consent")]
    pub sms_marketing_consent: MarketingConsent,

    #[serde(rename = "admin_graphql_api_id")]
    pub admin_graphql_api_id: String,

    #[serde(rename = "default_address")]
    pub default_address: DefaultAddress,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DefaultAddress {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "customer_id")]
    pub customer_id: i64,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "company")]
    pub company: Option<serde_json::Value>,

    #[serde(rename = "address1")]
    pub address1: String,

    #[serde(rename = "address2")]
    pub address2: Option<serde_json::Value>,

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
    pub default_address_default: bool,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MarketingConsent {
    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "opt_in_level")]
    pub opt_in_level: String,

    #[serde(rename = "consent_updated_at")]
    pub consent_updated_at: Option<serde_json::Value>,

    #[serde(rename = "consent_collected_from")]
    pub consent_collected_from: Option<String>,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LineItem {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "admin_graphql_api_id")]
    pub admin_graphql_api_id: String,

    #[serde(rename = "fulfillable_quantity")]
    pub fulfillable_quantity: i64,

    #[serde(rename = "fulfillment_service")]
    pub fulfillment_service: String,

    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: Option<serde_json::Value>,

    #[serde(rename = "gift_card")]
    pub gift_card: bool,

    #[serde(rename = "grams")]
    pub grams: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "price_set")]
    pub price_set: Set,

    #[serde(rename = "product_exists")]
    pub product_exists: bool,

    #[serde(rename = "product_id")]
    pub product_id: Option<serde_json::Value>,

    #[serde(rename = "properties")]
    pub properties: Vec<Option<serde_json::Value>>,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "requires_shipping")]
    pub requires_shipping: bool,

    #[serde(rename = "sku")]
    pub sku: Option<serde_json::Value>,

    #[serde(rename = "taxable")]
    pub taxable: bool,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "total_discount")]
    pub total_discount: String,

    #[serde(rename = "total_discount_set")]
    pub total_discount_set: Set,

    #[serde(rename = "variant_id")]
    pub variant_id: Option<serde_json::Value>,

    #[serde(rename = "variant_inventory_management")]
    pub variant_inventory_management: Option<serde_json::Value>,

    #[serde(rename = "variant_title")]
    pub variant_title: Option<serde_json::Value>,

    #[serde(rename = "vendor")]
    pub vendor: Option<serde_json::Value>,

    #[serde(rename = "tax_lines")]
    pub tax_lines: Vec<Option<serde_json::Value>>,

    #[serde(rename = "duties")]
    pub duties: Vec<Option<serde_json::Value>>,

    #[serde(rename = "discount_allocations")]
    pub discount_allocations: Vec<Option<serde_json::Value>>,
}

#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Currency {
    #[serde(rename = "CAD")]
    Cad,

    #[serde(rename = "EUR")]
    Eur,

    #[serde(rename = "USD")]
    Usd,
}


#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]

pub struct OrderArg {
  #[serde(rename = "line_items")]
  pub line_items: Vec<LineItemArg>,

  #[serde(rename = "customer")]
  pub customer: CustomerArg,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CustomerArg {
  #[serde(rename = "id")]
  pub id: i64,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LineItemArg {
  #[serde(rename = "variant_id")]
  pub variant_id: Option<i64>,

  #[serde(rename = "title")]
  pub title: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "price")]
  pub price: String,

  #[serde(rename = "quantity")]
  pub quantity: i64,
}


impl Default for Currency {
  fn default() -> Self {
    Self::Usd
  }
}
