mod types;

use reqwest::RequestBuilder;

use self::types::ApiVersion;

use crate::{
  result::{ShopifyError, ShopifyResult},
  session::MemorySession,
};

macro_rules! builder {
  ($o: ident,$($v: ident => $s: path),*) => {
    #[derive(Debug,Clone)]
        pub struct $o {
            $(
                pub $v: $s,
            )*
        }

        pub struct Builder {
          value: $o
        }

        impl Builder {
          $(
            pub fn $v(mut self,$v: $s) -> Builder {
              self.value.$v = $v;

              self
            }
          )*

          pub fn build(self) -> $o {
            self.value
          }

          #[inline]
          pub fn new() -> Self {
           Builder {
             value: Default::default()
           }
          }
        }
  };
}

builder! { Context,
   api_key => String,
   api_secret_key => String,
   password => String,
   access_token => String,
   scopes => Vec<String>,
   host_name => String,
   api_version => ApiVersion,
   is_embedded_app => bool,
   is_private_app => bool,
   session => MemorySession
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
      session: MemorySession::new(),
    }
  }
}

impl Context {
  pub fn authenticate(&self, b: RequestBuilder) -> ShopifyResult<RequestBuilder> {
    let mut errors = String::new();

    if self.api_key.is_empty() {
      errors.push_str(&format!("SHOPIFY_API_KEY "));
    }

    if self.api_secret_key.is_empty() {
      errors.push_str(&format!("SHOPIFY_API_SECRET "));
    }

    if self.host_name.is_empty() {
      errors.push_str(&format!("SHOPIFY_HOST_NAME "));
    }

    if self.password.is_empty() {
      errors.push_str(&format!("PASSWORD "));
    }

    if errors.len() > 0 {
      return Err(ShopifyError::ContextMissingValues {
        missing_values: errors,
      });
    }

    let mut b = b;
    if !self.access_token.is_empty() {
      b = b.header("X-Shopify-Access-Token", &self.access_token);
    } else {
      b = b.basic_auth(&self.api_key, Some(&self.password));
    }

    Ok(b)
  }
}
