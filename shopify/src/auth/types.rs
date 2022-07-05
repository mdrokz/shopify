use std::borrow::Cow;

use crate::session::types::OnlineAccessInfo;

#[cfg(feature = "openapi")]
use schemars::JsonSchema;

#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AuthQuery {
  pub code: String,
  pub timestamp: String,
  pub state: String,
  pub shop: String,
  pub host: Option<String>,
  pub hmac: Option<String>,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AccessTokenResponse {
  pub access_token: String,
  pub scope: String,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AuthResponse {
  #[serde(flatten)]
  pub access_token_response: AccessTokenResponse,
  #[serde(flatten)]
  pub online_accesss_info: OnlineAccessInfo,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AuthBody {
  pub client_id: String,
  pub client_secret: String,
  pub code: String,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AuthOptions<'a> {
  pub path: String,
  pub content_type: String,
  pub data: Cow<'a, AuthBody>,
}
