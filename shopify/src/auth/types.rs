use std::borrow::Cow;

use crate::session::types::OnlineAccessInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthQuery {
  pub code: String,
  pub timestamp: String,
  pub state: String,
  pub shop: String,
  pub host: Option<String>,
  pub hmac: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AccessTokenResponse {
  pub access_token: String,
  pub scope: String,
}
#[derive(Clone, Deserialize, Debug)]
pub struct AuthResponse {
  #[serde(flatten)]
  pub access_token_response: AccessTokenResponse,
  #[serde(flatten)]
  pub online_accesss_info: OnlineAccessInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBody {
  pub client_id: String,
  pub client_secret: String,
  pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthOptions<'a> {
  pub path: String,
  pub content_type: String,
  pub data: Cow<'a, AuthBody>,
}
