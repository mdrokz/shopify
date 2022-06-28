use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct WebHookArg {
  #[serde(rename = "address")]
  pub address: String,

  #[serde(rename = "fields")]
  pub fields: Vec<String>,

  #[serde(rename = "format")]
  pub format: String,

  #[serde(rename = "format")]
  pub topic: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct WebHookBody<T: Serialize = WebHookArg> {
  pub webhook: T,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct CustomerDataRequest {
  #[serde(rename = "shop_id")]
  pub shop_id: i64,

  #[serde(rename = "shop_domain")]
  pub shop_domain: String,

  #[serde(rename = "orders_requested")]
  pub orders_requested: Vec<i64>,

  #[serde(rename = "customer")]
  pub customer: Customer,

  #[serde(rename = "data_request")]
  pub data_request: DataRequest,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct CustomerDataRedact {
  #[serde(rename = "shop_id")]
  pub shop_id: i64,

  #[serde(rename = "shop_domain")]
  pub shop_domain: String,

  #[serde(rename = "orders_to_redact")]
  pub orders_to_redact: Vec<i64>,

  #[serde(rename = "customer")]
  pub customer: Customer,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
pub struct ShopRedact {
  #[serde(rename = "shop_id")]
  pub shop_id: i64,

  #[serde(rename = "shop_domain")]
  pub shop_domain: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
pub struct Customer {
  #[serde(rename = "id")]
  pub id: Option<i64>,

  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "phone")]
  pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
pub struct DataRequest {
  #[serde(rename = "id")]
  pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
pub struct Webhook {
  #[serde(rename = "address")]
  pub address: String,

  #[serde(rename = "api_version")]
  pub api_version: String,

  #[serde(rename = "created_at")]
  pub created_at: String,

  #[serde(rename = "fields")]
  pub fields: Vec<String>,

  #[serde(rename = "format")]
  pub format: String,

  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "metafield_namespaces")]
  pub metafield_namespaces: Vec<String>,

  #[serde(rename = "private_metafield_namespaces")]
  pub private_metafield_namespaces: Vec<String>,

  #[serde(rename = "topic")]
  pub topic: String,

  #[serde(rename = "updated_at")]
  pub updated_at: String,
}
