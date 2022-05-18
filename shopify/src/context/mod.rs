mod types;

use reqwest::RequestBuilder;

use self::types::ApiVersion;


use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Context {
  api_key: String,
  api_secret_key: String,
  password: String,
  access_token: String,
  scopes: Vec<String>,
  host_name: String,
  api_version: ApiVersion,
  is_embedded_app: bool,
  is_private_app: bool,
}

impl Default for Context {
  fn default() -> Self {
    Self {
      api_key: String::new(),
      access_token: String::new(),
      password: String::new(),
      api_secret_key: String::new(),
      scopes: vec!["write_products", "write_customers", "write_draft_orders"]
        .iter()
        .map(|x| x.to_string())
        .collect(),
      host_name: String::new(),
      api_version: Default::default(),
      is_embedded_app: true,
      is_private_app: false,
    }
  }
}

impl Context {
  pub fn initialize(
    api_key: &str,
    api_secret_key: &str,
    password: &str,
    scopes: Vec<String>,
  ) -> Result<Self, String> {
    if api_key.is_empty() {
      return Err(String::from("SHOPIFY_API_KEY is missing"));
    }

    if api_secret_key.is_empty() {
      return Err(String::from("SHOPIFY_API_SECRET is missing"));
    }

    if password.is_empty() {
      return Err(String::from("PASSWORD is missing"));
    }

    Ok(Self {
      api_key: api_key.to_string(),
      api_secret_key: api_secret_key.to_string(),
      scopes,
      ..Default::default()
    })
  }

  pub fn initialize_with_token(access_token: &str, scopes: Vec<String>) -> Result<Self, String> {
    if access_token.is_empty() {
      return Err(String::from("ACCESS_TOKEN is missing"));
    }

    Ok(Self {
      scopes,
      ..Default::default()
    })
  }

  pub fn authenticate(&self, b: RequestBuilder) -> RequestBuilder {
    let mut b = b;
    if !self.access_token.is_empty() {
      b = b.header("X-Shopify-Access-Token", &self.access_token);
    } else {
      b = b.basic_auth(&self.api_key, Some(&self.password));
    }

    b
  }
}
