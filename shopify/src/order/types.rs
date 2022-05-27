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

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
  #[serde(rename = "orders")]
  pub orders: Vec<OrderElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderElement {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "admin_graphql_api_id")]
  pub admin_graphql_api_id: String,

  #[serde(rename = "app_id")]
  pub app_id: Option<serde_json::Value>,

  #[serde(rename = "browser_ip")]
  pub browser_ip: String,

  #[serde(rename = "buyer_accepts_marketing")]
  pub buyer_accepts_marketing: bool,

  #[serde(rename = "cancel_reason")]
  pub cancel_reason: Option<serde_json::Value>,

  #[serde(rename = "cancelled_at")]
  pub cancelled_at: Option<serde_json::Value>,

  #[serde(rename = "cart_token")]
  pub cart_token: String,

  #[serde(rename = "checkout_id")]
  pub checkout_id: i64,

  #[serde(rename = "checkout_token")]
  pub checkout_token: String,

  #[serde(rename = "client_details")]
  pub client_details: ClientDetails,

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

  #[serde(rename = "customer_locale")]
  pub customer_locale: Option<serde_json::Value>,

  #[serde(rename = "device_id")]
  pub device_id: Option<serde_json::Value>,

  #[serde(rename = "discount_codes")]
  pub discount_codes: Vec<DiscountCode>,

  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "financial_status")]
  pub financial_status: String,

  #[serde(rename = "fulfillment_status")]
  pub fulfillment_status: Option<serde_json::Value>,

  #[serde(rename = "gateway")]
  pub gateway: String,

  #[serde(rename = "landing_site")]
  pub landing_site: String,

  #[serde(rename = "landing_site_ref")]
  pub landing_site_ref: String,

  #[serde(rename = "location_id")]
  pub location_id: Option<serde_json::Value>,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "note")]
  pub note: Option<serde_json::Value>,

  #[serde(rename = "note_attributes")]
  pub note_attributes: Vec<NoteAttribute>,

  #[serde(rename = "number")]
  pub number: i64,

  #[serde(rename = "order_number")]
  pub order_number: i64,

  #[serde(rename = "order_status_url")]
  pub order_status_url: String,

  #[serde(rename = "payment_gateway_names")]
  pub payment_gateway_names: Vec<String>,

  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "presentment_currency")]
  pub presentment_currency: Currency,

  #[serde(rename = "processed_at")]
  pub processed_at: String,

  #[serde(rename = "processing_method")]
  pub processing_method: String,

  #[serde(rename = "reference")]
  pub reference: String,

  #[serde(rename = "referring_site")]
  pub referring_site: String,

  #[serde(rename = "source_identifier")]
  pub source_identifier: String,

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
  pub tax_lines: Vec<TaxLine>,

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

  #[serde(rename = "billing_address")]
  pub billing_address: Address,

  #[serde(rename = "customer")]
  pub customer: Customer,

  #[serde(rename = "discount_applications")]
  pub discount_applications: Vec<DiscountApplication>,

  #[serde(rename = "fulfillments")]
  pub fulfillments: Vec<Fulfillment>,

  #[serde(rename = "line_items")]
  pub line_items: Vec<LineItem>,

  #[serde(rename = "payment_details")]
  pub payment_details: PaymentDetails,

  #[serde(rename = "refunds")]
  pub refunds: Vec<Refund>,

  #[serde(rename = "shipping_address")]
  pub shipping_address: Address,

  #[serde(rename = "shipping_lines")]
  pub shipping_lines: Vec<ShippingLine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  #[serde(rename = "first_name")]
  pub first_name: Option<String>,

  #[serde(rename = "address1")]
  pub address1: String,

  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "city")]
  pub city: String,

  #[serde(rename = "zip")]
  pub zip: String,

  #[serde(rename = "province")]
  pub province: String,

  #[serde(rename = "country")]
  pub country: String,

  #[serde(rename = "last_name")]
  pub last_name: Option<String>,

  #[serde(rename = "address2")]
  pub address2: String,

  #[serde(rename = "company")]
  pub company: Option<serde_json::Value>,

  #[serde(rename = "latitude")]
  pub latitude: Option<f64>,

  #[serde(rename = "longitude")]
  pub longitude: Option<f64>,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "country_code")]
  pub country_code: String,

  #[serde(rename = "province_code")]
  pub province_code: String,

  #[serde(rename = "id")]
  pub id: Option<i64>,

  #[serde(rename = "customer_id")]
  pub customer_id: Option<i64>,

  #[serde(rename = "country_name")]
  pub country_name: Option<String>,

  #[serde(rename = "default")]
  pub address_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDetails {
  #[serde(rename = "accept_language")]
  pub accept_language: Option<serde_json::Value>,

  #[serde(rename = "browser_height")]
  pub browser_height: Option<serde_json::Value>,

  #[serde(rename = "browser_ip")]
  pub browser_ip: String,

  #[serde(rename = "browser_width")]
  pub browser_width: Option<serde_json::Value>,

  #[serde(rename = "session_hash")]
  pub session_hash: Option<serde_json::Value>,

  #[serde(rename = "user_agent")]
  pub user_agent: Option<serde_json::Value>,
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
  pub currency: Currency,

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
pub struct DiscountApplication {
  #[serde(rename = "target_type")]
  pub target_type: String,

  #[serde(rename = "type")]
  pub discount_application_type: String,

  #[serde(rename = "value")]
  pub value: String,

  #[serde(rename = "value_type")]
  pub value_type: String,

  #[serde(rename = "allocation_method")]
  pub allocation_method: String,

  #[serde(rename = "target_selection")]
  pub target_selection: String,

  #[serde(rename = "code")]
  pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountCode {
  #[serde(rename = "code")]
  pub code: String,

  #[serde(rename = "amount")]
  pub amount: String,

  #[serde(rename = "type")]
  pub discount_code_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fulfillment {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "admin_graphql_api_id")]
  pub admin_graphql_api_id: String,

  #[serde(rename = "created_at")]
  pub created_at: String,

  #[serde(rename = "location_id")]
  pub location_id: i64,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "order_id")]
  pub order_id: i64,

  #[serde(rename = "receipt")]
  pub receipt: FulfillmentReceipt,

  #[serde(rename = "service")]
  pub service: String,

  #[serde(rename = "shipment_status")]
  pub shipment_status: Option<serde_json::Value>,

  #[serde(rename = "status")]
  pub status: String,

  #[serde(rename = "tracking_company")]
  pub tracking_company: String,

  #[serde(rename = "tracking_number")]
  pub tracking_number: String,

  #[serde(rename = "tracking_numbers")]
  pub tracking_numbers: Vec<String>,

  #[serde(rename = "tracking_url")]
  pub tracking_url: String,

  #[serde(rename = "tracking_urls")]
  pub tracking_urls: Vec<String>,

  #[serde(rename = "updated_at")]
  pub updated_at: String,

  #[serde(rename = "line_items")]
  pub line_items: Vec<LineItem>,
}

#[derive(Debug, Serialize, Deserialize)]
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
  pub product_id: i64,

  #[serde(rename = "properties")]
  pub properties: Vec<NoteAttribute>,

  #[serde(rename = "quantity")]
  pub quantity: i64,

  #[serde(rename = "requires_shipping")]
  pub requires_shipping: bool,

  #[serde(rename = "sku")]
  pub sku: String,

  #[serde(rename = "taxable")]
  pub taxable: bool,

  #[serde(rename = "title")]
  pub title: String,

  #[serde(rename = "total_discount")]
  pub total_discount: String,

  #[serde(rename = "total_discount_set")]
  pub total_discount_set: Set,

  #[serde(rename = "variant_id")]
  pub variant_id: i64,

  #[serde(rename = "variant_inventory_management")]
  pub variant_inventory_management: String,

  #[serde(rename = "variant_title")]
  pub variant_title: String,

  #[serde(rename = "vendor")]
  pub vendor: Option<serde_json::Value>,

  #[serde(rename = "tax_lines")]
  pub tax_lines: Vec<TaxLine>,

  #[serde(rename = "discount_allocations")]
  pub discount_allocations: Vec<DiscountAllocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountAllocation {
  #[serde(rename = "amount")]
  pub amount: String,

  #[serde(rename = "amount_set")]
  pub amount_set: Set,

  #[serde(rename = "discount_application_index")]
  pub discount_application_index: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Set {
  #[serde(rename = "shop_money")]
  pub shop_money: Money,

  #[serde(rename = "presentment_money")]
  pub presentment_money: Money,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
  #[serde(rename = "amount")]
  pub amount: String,

  #[serde(rename = "currency_code")]
  pub currency_code: Currency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteAttribute {
  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "value")]
  pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxLine {
  #[serde(rename = "price")]
  pub price: String,

  #[serde(rename = "price_set")]
  pub price_set: Set,

  #[serde(rename = "rate")]
  pub rate: f64,

  #[serde(rename = "title")]
  pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfillmentReceipt {
  #[serde(rename = "testcase")]
  pub testcase: bool,

  #[serde(rename = "authorization")]
  pub authorization: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetails {
  #[serde(rename = "credit_card_bin")]
  pub credit_card_bin: Option<serde_json::Value>,

  #[serde(rename = "avs_result_code")]
  pub avs_result_code: Option<serde_json::Value>,

  #[serde(rename = "cvv_result_code")]
  pub cvv_result_code: Option<serde_json::Value>,

  #[serde(rename = "credit_card_number")]
  pub credit_card_number: String,

  #[serde(rename = "credit_card_company")]
  pub credit_card_company: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Refund {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "admin_graphql_api_id")]
  pub admin_graphql_api_id: String,

  #[serde(rename = "created_at")]
  pub created_at: String,

  #[serde(rename = "note")]
  pub note: String,

  #[serde(rename = "order_id")]
  pub order_id: i64,

  #[serde(rename = "processed_at")]
  pub processed_at: String,

  #[serde(rename = "restock")]
  pub restock: bool,

  #[serde(rename = "user_id")]
  pub user_id: i64,

  #[serde(rename = "order_adjustments")]
  pub order_adjustments: Vec<Option<serde_json::Value>>,

  #[serde(rename = "transactions")]
  pub transactions: Vec<Transaction>,

  #[serde(rename = "refund_line_items")]
  pub refund_line_items: Vec<RefundLineItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundLineItem {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "line_item_id")]
  pub line_item_id: i64,

  #[serde(rename = "location_id")]
  pub location_id: i64,

  #[serde(rename = "quantity")]
  pub quantity: i64,

  #[serde(rename = "restock_type")]
  pub restock_type: String,

  #[serde(rename = "subtotal")]
  pub subtotal: f64,

  #[serde(rename = "subtotal_set")]
  pub subtotal_set: Set,

  #[serde(rename = "total_tax")]
  pub total_tax: f64,

  #[serde(rename = "total_tax_set")]
  pub total_tax_set: Set,

  #[serde(rename = "line_item")]
  pub line_item: LineItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "admin_graphql_api_id")]
  pub admin_graphql_api_id: String,

  #[serde(rename = "amount")]
  pub amount: String,

  #[serde(rename = "authorization")]
  pub authorization: String,

  #[serde(rename = "created_at")]
  pub created_at: String,

  #[serde(rename = "currency")]
  pub currency: Currency,

  #[serde(rename = "device_id")]
  pub device_id: Option<serde_json::Value>,

  #[serde(rename = "error_code")]
  pub error_code: Option<serde_json::Value>,

  #[serde(rename = "gateway")]
  pub gateway: String,

  #[serde(rename = "kind")]
  pub kind: String,

  #[serde(rename = "location_id")]
  pub location_id: Option<serde_json::Value>,

  #[serde(rename = "message")]
  pub message: Option<serde_json::Value>,

  #[serde(rename = "order_id")]
  pub order_id: i64,

  #[serde(rename = "parent_id")]
  pub parent_id: i64,

  #[serde(rename = "processed_at")]
  pub processed_at: String,

  #[serde(rename = "receipt")]
  pub receipt: TransactionReceipt,

  #[serde(rename = "source_name")]
  pub source_name: String,

  #[serde(rename = "status")]
  pub status: String,

  #[serde(rename = "test")]
  pub test: bool,

  #[serde(rename = "user_id")]
  pub user_id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionReceipt {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingLine {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "carrier_identifier")]
  pub carrier_identifier: Option<serde_json::Value>,

  #[serde(rename = "code")]
  pub code: String,

  #[serde(rename = "delivery_category")]
  pub delivery_category: Option<serde_json::Value>,

  #[serde(rename = "discounted_price")]
  pub discounted_price: String,

  #[serde(rename = "discounted_price_set")]
  pub discounted_price_set: Set,

  #[serde(rename = "phone")]
  pub phone: Option<serde_json::Value>,

  #[serde(rename = "price")]
  pub price: String,

  #[serde(rename = "price_set")]
  pub price_set: Set,

  #[serde(rename = "requested_fulfillment_service_id")]
  pub requested_fulfillment_service_id: Option<serde_json::Value>,

  #[serde(rename = "source")]
  pub source: String,

  #[serde(rename = "title")]
  pub title: String,

  #[serde(rename = "tax_lines")]
  pub tax_lines: Vec<Option<serde_json::Value>>,

  #[serde(rename = "discount_allocations")]
  pub discount_allocations: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
  #[serde(rename = "USD")]
  Usd,
}
