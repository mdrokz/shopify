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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct Order {
    #[serde(rename = "app_id")]
    pub app_id: i64,

    #[serde(rename = "billing_address")]
    pub billing_address: IngAddress,

    #[serde(rename = "browser_ip")]
    pub browser_ip: String,

    #[serde(rename = "buyer_accepts_marketing")]
    pub buyer_accepts_marketing: bool,

    #[serde(rename = "cancel_reason")]
    pub cancel_reason: String,

    #[serde(rename = "cancelled_at")]
    pub cancelled_at: Option<serde_json::Value>,

    #[serde(rename = "cart_token")]
    pub cart_token: String,

    #[serde(rename = "checkout_token")]
    pub checkout_token: String,

    #[serde(rename = "client_details")]
    pub client_details: ClientDetails,

    #[serde(rename = "closed_at")]
    pub closed_at: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "currency")]
    pub currency: Currency,

    #[serde(rename = "current_total_discounts")]
    pub current_total_discounts: String,

    #[serde(rename = "current_total_discounts_set")]
    pub current_total_discounts_set: CurrentTotalDiscountsSet,

    #[serde(rename = "current_total_duties_set")]
    pub current_total_duties_set: CurrentTotalDutiesSet,

    #[serde(rename = "current_total_price")]
    pub current_total_price: String,

    #[serde(rename = "current_total_price_set")]
    pub current_total_price_set: CurrentTotalPriceSet,

    #[serde(rename = "current_subtotal_price")]
    pub current_subtotal_price: String,

    #[serde(rename = "current_subtotal_price_set")]
    pub current_subtotal_price_set: CurrentSubtotalPriceSet,

    #[serde(rename = "current_total_tax")]
    pub current_total_tax: String,

    #[serde(rename = "current_total_tax_set")]
    pub current_total_tax_set: CurrentTotalTaxSet,

    #[serde(rename = "customer")]
    pub customer: Customer,

    #[serde(rename = "customer_locale")]
    pub customer_locale: String,

    #[serde(rename = "discount_applications")]
    pub discount_applications: DiscountApplications,

    #[serde(rename = "discount_codes")]
    pub discount_codes: Vec<DiscountCode>,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "estimated_taxes")]
    pub estimated_taxes: bool,

    #[serde(rename = "financial_status")]
    pub financial_status: String,

    #[serde(rename = "fulfillments")]
    pub fulfillments: Vec<Fulfillment>,

    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: String,

    #[serde(rename = "gateway")]
    pub gateway: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "landing_site")]
    pub landing_site: String,

    #[serde(rename = "line_items")]
    pub line_items: Vec<LineItem>,

    #[serde(rename = "location_id")]
    pub location_id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "note")]
    pub note: String,

    #[serde(rename = "note_attributes")]
    pub note_attributes: Vec<NoteAttribute>,

    #[serde(rename = "number")]
    pub number: i64,

    #[serde(rename = "order_number")]
    pub order_number: i64,

    #[serde(rename = "original_total_duties_set")]
    pub original_total_duties_set: OriginalTotalDutiesSet,

    #[serde(rename = "payment_details")]
    pub payment_details: PaymentDetails,

    #[serde(rename = "payment_terms")]
    pub payment_terms: PaymentTerms,

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

    #[serde(rename = "referring_site")]
    pub referring_site: String,

    #[serde(rename = "refunds")]
    pub refunds: Vec<Refund>,

    #[serde(rename = "shipping_address")]
    pub shipping_address: IngAddress,

    #[serde(rename = "shipping_lines")]
    pub shipping_lines: Vec<ShippingLine>,

    #[serde(rename = "source_name")]
    pub source_name: String,

    #[serde(rename = "source_identifier")]
    pub source_identifier: String,

    #[serde(rename = "source_url")]
    pub source_url: String,

    #[serde(rename = "subtotal_price")]
    pub subtotal_price: i64,

    #[serde(rename = "subtotal_price_set")]
    pub subtotal_price_set: Set,

    #[serde(rename = "tags")]
    pub tags: String,

    #[serde(rename = "tax_lines")]
    pub tax_lines: Vec<OrderTaxLine>,

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
    pub user_id: i64,

    #[serde(rename = "order_status_url")]
    pub order_status_url: OrderStatusUrl,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
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

    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "note")]
    pub note: Option<serde_json::Value>,

    #[serde(rename = "verified_email")]
    pub verified_email: bool,

    #[serde(rename = "multipass_identifier")]
    pub multipass_identifier: Option<serde_json::Value>,

    #[serde(rename = "tax_exempt")]
    pub tax_exempt: bool,

    #[serde(rename = "tax_exemptions")]
    pub tax_exemptions: Addresses,

    #[serde(rename = "phone")]
    pub phone: String,

    #[serde(rename = "tags")]
    pub tags: String,

    #[serde(rename = "currency")]
    pub currency: Currency,

    #[serde(rename = "addresses")]
    pub addresses: Addresses,

    #[serde(rename = "admin_graphql_api_id")]
    pub admin_graphql_api_id: String,

    #[serde(rename = "default_address")]
    pub default_address: Addresses,
}


#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct Fulfillment {
    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "order_id")]
    pub order_id: i64,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "tracking_company")]
    pub tracking_company: String,

    #[serde(rename = "tracking_number")]
    pub tracking_number: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct IngAddress {
    #[serde(rename = "address1")]
    pub address1: String,

    #[serde(rename = "address2")]
    pub address2: String,

    #[serde(rename = "city")]
    pub city: String,

    #[serde(rename = "company")]
    pub company: Option<serde_json::Value>,

    #[serde(rename = "country")]
    pub country: String,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "phone")]
    pub phone: String,

    #[serde(rename = "province")]
    pub province: String,

    #[serde(rename = "zip")]
    pub zip: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "province_code")]
    pub province_code: String,

    #[serde(rename = "country_code")]
    pub country_code: String,

    #[serde(rename = "latitude")]
    pub latitude: String,

    #[serde(rename = "longitude")]
    pub longitude: String,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct ClientDetails {
    #[serde(rename = "accept_language")]
    pub accept_language: String,

    #[serde(rename = "browser_height")]
    pub browser_height: i64,

    #[serde(rename = "browser_ip")]
    pub browser_ip: String,

    #[serde(rename = "browser_width")]
    pub browser_width: i64,

    #[serde(rename = "session_hash")]
    pub session_hash: String,

    #[serde(rename = "user_agent")]
    pub user_agent: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct CurrentSubtotalPriceSet {
    #[serde(rename = "current_subtotal_price_set")]
    pub current_subtotal_price_set: Set,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct Set {
    #[serde(rename = "shop_money")]
    pub shop_money: Money,

    #[serde(rename = "presentment_money")]
    pub presentment_money: Money,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct Money {
    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "currency_code")]
    pub currency_code: Currency,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct CurrentTotalDiscountsSet {
    #[serde(rename = "current_total_discounts_set")]
    pub current_total_discounts_set: Set,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct CurrentTotalDutiesSet {
    #[serde(rename = "current_total_duties_set")]
    pub current_total_duties_set: Set,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct CurrentTotalPriceSet {
    #[serde(rename = "current_total_price_set")]
    pub current_total_price_set: Set,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct CurrentTotalTaxSet {
    #[serde(rename = "current_total_tax_set")]
    pub current_total_tax_set: Set,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct Addresses {
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct DiscountApplications {
    #[serde(rename = "discount_applications")]
    pub discount_applications: Vec<DiscountApplication>,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct DiscountApplication {
    #[serde(rename = "type")]
    pub discount_application_type: String,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "value")]
    pub value: String,

    #[serde(rename = "value_type")]
    pub value_type: String,

    #[serde(rename = "allocation_method")]
    pub allocation_method: String,

    #[serde(rename = "target_selection")]
    pub target_selection: String,

    #[serde(rename = "target_type")]
    pub target_type: String,

    #[serde(rename = "code")]
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct DiscountCode {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "type")]
    pub discount_code_type: String,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct LineItem {
    #[serde(rename = "fulfillable_quantity")]
    pub fulfillable_quantity: i64,

    #[serde(rename = "fulfillment_service")]
    pub fulfillment_service: String,

    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: String,

    #[serde(rename = "grams")]
    pub grams: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "product_id")]
    pub product_id: i64,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "requires_shipping")]
    pub requires_shipping: bool,

    #[serde(rename = "sku")]
    pub sku: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "variant_id")]
    pub variant_id: i64,

    #[serde(rename = "variant_title")]
    pub variant_title: String,

    #[serde(rename = "vendor")]
    pub vendor: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "gift_card")]
    pub gift_card: bool,

    #[serde(rename = "price_set")]
    pub price_set: Set,

    #[serde(rename = "properties")]
    pub properties: Vec<NoteAttribute>,

    #[serde(rename = "taxable")]
    pub taxable: bool,

    #[serde(rename = "tax_lines")]
    pub tax_lines: Vec<DutyTaxLine>,

    #[serde(rename = "total_discount")]
    pub total_discount: String,

    #[serde(rename = "total_discount_set")]
    pub total_discount_set: Set,

    #[serde(rename = "discount_allocations")]
    pub discount_allocations: Vec<DiscountAllocation>,

    #[serde(rename = "origin_location")]
    pub origin_location: OriginLocation,

    #[serde(rename = "duties")]
    pub duties: Vec<Duty>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct DiscountAllocation {
    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "discount_application_index")]
    pub discount_application_index: i64,

    #[serde(rename = "amount_set")]
    pub amount_set: Set,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct Duty {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "harmonized_system_code")]
    pub harmonized_system_code: String,

    #[serde(rename = "country_code_of_origin")]
    pub country_code_of_origin: String,

    #[serde(rename = "shop_money")]
    pub shop_money: Money,

    #[serde(rename = "presentment_money")]
    pub presentment_money: Money,

    #[serde(rename = "tax_lines")]
    pub tax_lines: Vec<DutyTaxLine>,

    #[serde(rename = "admin_graphql_api_id")]
    pub admin_graphql_api_id: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct DutyTaxLine {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "rate")]
    pub rate: f64,

    #[serde(rename = "price_set")]
    pub price_set: Set,

    #[serde(rename = "channel_liable")]
    pub channel_liable: bool,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct OriginLocation {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "country_code")]
    pub country_code: String,

    #[serde(rename = "province_code")]
    pub province_code: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "address1")]
    pub address1: String,

    #[serde(rename = "address2")]
    pub address2: String,

    #[serde(rename = "city")]
    pub city: String,

    #[serde(rename = "zip")]
    pub zip: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct NoteAttribute {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct OrderStatusUrl {
    #[serde(rename = "order_status_url")]
    pub order_status_url: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct OriginalTotalDutiesSet {
    #[serde(rename = "original_total_duties_set")]
    pub original_total_duties_set: Set,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct PaymentDetails {
    #[serde(rename = "avs_result_code")]
    pub avs_result_code: String,

    #[serde(rename = "credit_card_bin")]
    pub credit_card_bin: String,

    #[serde(rename = "cvv_result_code")]
    pub cvv_result_code: String,

    #[serde(rename = "credit_card_number")]
    pub credit_card_number: String,

    #[serde(rename = "credit_card_company")]
    pub credit_card_company: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct PaymentTerms {
    #[serde(rename = "amount")]
    pub amount: i64,

    #[serde(rename = "currency")]
    pub currency: Currency,

    #[serde(rename = "payment_terms_name")]
    pub payment_terms_name: String,

    #[serde(rename = "payment_terms_type")]
    pub payment_terms_type: String,

    #[serde(rename = "due_in_days")]
    pub due_in_days: i64,

    #[serde(rename = "payment_schedules")]
    pub payment_schedules: Vec<PaymentSchedule>,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct PaymentSchedule {
    #[serde(rename = "amount")]
    pub amount: i64,

    #[serde(rename = "currency")]
    pub currency: Currency,

    #[serde(rename = "issued_at")]
    pub issued_at: String,

    #[serde(rename = "due_at")]
    pub due_at: String,

    #[serde(rename = "completed_at")]
    pub completed_at: String,

    #[serde(rename = "expected_payment_method")]
    pub expected_payment_method: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct Refund {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "order_id")]
    pub order_id: i64,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "note")]
    pub note: Option<serde_json::Value>,

    #[serde(rename = "user_id")]
    pub user_id: Option<serde_json::Value>,

    #[serde(rename = "processed_at")]
    pub processed_at: String,

    #[serde(rename = "refund_line_items")]
    pub refund_line_items: Vec<Option<serde_json::Value>>,

    #[serde(rename = "transactions")]
    pub transactions: Vec<Option<serde_json::Value>>,

    #[serde(rename = "order_adjustments")]
    pub order_adjustments: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct TransactionReceipt {}

#[derive(Debug, Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct ShippingLine {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "price_set")]
    pub price_set: Set,

    #[serde(rename = "discounted_price")]
    pub discounted_price: String,

    #[serde(rename = "discounted_price_set")]
    pub discounted_price_set: Set,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "tax_lines")]
    pub tax_lines: Vec<Option<serde_json::Value>>,

    #[serde(rename = "carrier_identifier")]
    pub carrier_identifier: String,

    #[serde(rename = "requested_fulfillment_service_id")]
    pub requested_fulfillment_service_id: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default,JsonSchema)]
pub struct OrderTaxLine {
    #[serde(rename = "price")]
    pub price: f64,

    #[serde(rename = "rate")]
    pub rate: f64,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "channel_liable")]
    pub channel_liable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Currency {
    #[serde(rename = "CAD")]
    Cad,

    #[serde(rename = "EUR")]
    Eur,

    #[serde(rename = "USD")]
    Usd,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]

pub struct OrderArg {
  #[serde(rename = "line_items")]
  pub line_items: Vec<LineItemArg>,

  #[serde(rename = "customer")]
  pub customer: CustomerArg,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct CustomerArg {
  #[serde(rename = "id")]
  pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
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
