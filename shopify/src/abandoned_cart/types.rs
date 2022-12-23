
use serde::{Deserialize, Serialize};

// #[derive(Debug,Clone, Serialize, Deserialize)]
// pub struct AbandonedCheckout {
//     pub checkouts: Vec<Checkout>,
// }

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Checkout {
    pub id: i64,
    pub token: String,
    pub cart_token: String,
    pub email: String,
    pub gateway: Option<serde_json::Value>,
    pub buyer_accepts_marketing: bool,
    pub created_at: String,
    pub updated_at: String,
    pub landing_site: Option<serde_json::Value>,
    pub note: Option<serde_json::Value>,
    pub note_attributes: Vec<NoteAttribute>,
    pub referring_site: Option<serde_json::Value>,
    pub shipping_lines: Vec<ShippingLine>,
    pub taxes_included: bool,
    pub total_weight: i64,
    pub currency: String,
    pub completed_at: Option<serde_json::Value>,
    pub closed_at: Option<serde_json::Value>,
    pub user_id: Option<serde_json::Value>,
    pub location_id: Option<serde_json::Value>,
    pub source_identifier: Option<serde_json::Value>,
    pub source_url: Option<serde_json::Value>,
    pub device_id: Option<serde_json::Value>,
    pub phone: Option<serde_json::Value>,
    pub customer_locale: String,
    pub line_items: Vec<LineItem>,
    pub name: String,
    pub source: Option<serde_json::Value>,
    pub abandoned_checkout_url: String,
    pub discount_codes: Vec<DiscountCode>,
    pub tax_lines: Vec<TaxLine>,
    pub source_name: String,
    pub presentment_currency: String,
    pub buyer_accepts_sms_marketing: bool,
    pub sms_marketing_phone: Option<serde_json::Value>,
    pub total_discounts: String,
    pub total_line_items_price: String,
    pub total_price: String,
    pub total_tax: String,
    pub subtotal_price: String,
    pub total_duties: Option<serde_json::Value>,
    pub billing_address: Address,
    pub shipping_address: Address,
    pub customer: Customer,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Address {
    pub first_name: Option<String>,
    pub address1: String,
    pub phone: String,
    pub city: String,
    pub zip: String,
    pub province: String,
    pub country: String,
    pub last_name: Option<String>,
    pub address2: String,
    pub company: Option<serde_json::Value>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub name: String,
    pub country_code: String,
    pub province_code: String,
    pub id: Option<i64>,
    pub customer_id: Option<i64>,
    pub country_name: Option<String>,
    #[serde(rename = "default")]
    pub address_default: Option<bool>,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
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
    pub last_order_id: i64,
    pub note: Option<serde_json::Value>,
    pub verified_email: bool,
    pub multipass_identifier: Option<serde_json::Value>,
    pub tax_exempt: bool,
    pub tags: String,
    pub last_order_name: String,
    pub currency: String,
    pub phone: String,
    pub accepts_marketing_updated_at: String,
    pub marketing_opt_in_level: Option<serde_json::Value>,
    pub tax_exemptions: Vec<Option<serde_json::Value>>,
    pub email_marketing_consent: MarketingConsent,
    pub sms_marketing_consent: MarketingConsent,
    pub admin_graphql_api_id: String,
    pub default_address: Address,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct MarketingConsent {
    pub state: String,
    pub opt_in_level: Option<String>,
    pub consent_updated_at: String,
    pub consent_collected_from: Option<String>,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct DiscountCode {
    pub code: String,
    pub amount: String,
    #[serde(rename = "type")]
    pub discount_code_type: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub applied_discounts: Vec<Option<serde_json::Value>>,
    pub discount_allocations: Vec<DiscountAllocation>,
    pub key: String,
    pub destination_location_id: Option<serde_json::Value>,
    pub fulfillment_service: String,
    pub gift_card: bool,
    pub grams: i64,
    pub origin_location_id: Option<serde_json::Value>,
    pub presentment_title: String,
    pub presentment_variant_title: String,
    pub product_id: i64,
    pub properties: Option<serde_json::Value>,
    pub quantity: i64,
    pub requires_shipping: bool,
    pub sku: String,
    pub tax_lines: Vec<Option<serde_json::Value>>,
    pub taxable: bool,
    pub title: String,
    pub variant_id: i64,
    pub variant_title: String,
    pub variant_price: Option<serde_json::Value>,
    pub vendor: String,
    pub user_id: Option<serde_json::Value>,
    pub unit_price_measurement: Option<serde_json::Value>,
    pub rank: Option<serde_json::Value>,
    pub compare_at_price: Option<serde_json::Value>,
    pub line_price: String,
    pub price: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct DiscountAllocation {
    pub id: Option<serde_json::Value>,
    pub amount: String,
    pub description: Option<serde_json::Value>,
    pub created_at: Option<serde_json::Value>,
    pub application_type: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct NoteAttribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct ShippingLine {
    pub title: String,
    pub price: String,
    pub code: String,
    pub source: String,
    pub applied_discounts: Vec<Option<serde_json::Value>>,
    pub id: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct TaxLine {
    pub price: String,
    pub rate: f64,
    pub title: String,
    pub channel_liable: Option<serde_json::Value>,
}
