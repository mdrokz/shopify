use serde::Serialize;

#[cfg(feature = "openapi")]
use schemars::JsonSchema;

#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebHookBody<T: Serialize = WebHookArg> {
  pub webhook: T,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ShopRedact {
  #[serde(rename = "shop_id")]
  pub shop_id: i64,

  #[serde(rename = "shop_domain")]
  pub shop_domain: String,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Customer {
  #[serde(rename = "id")]
  pub id: Option<i64>,

  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "phone")]
  pub phone: String,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DataRequest {
  #[serde(rename = "id")]
  pub id: i64,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
