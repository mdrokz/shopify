mod types;

use self::types::ApiVersion;

pub struct Context {
  api_key: String,
  api_secret_key: String,
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
  fn initialize(
    api_key: String,
    api_secret_key: String,
    scopes: Vec<String>,
  ) -> Result<Self, String> {
    if api_key.is_empty() {
      return Err(String::from("SHOPIFY_API_KEY is missing"))
    }

    if api_secret_key.is_empty() {
      return Err(String::from("SHOPIFY_API_SECRET is missing"));
    }

    Ok(Self {
      api_key,
      api_secret_key,
      scopes,
      ..Default::default()
    })
  }
}
