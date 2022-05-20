use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthQuery {
  pub code: String,
  pub timestamp: String,
  pub state: String,
  pub shop: String,
  pub host: Option<String>,
  pub hmac: Option<String>,
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
