use std::collections::HashMap;

use cookie::time::{Duration, OffsetDateTime};
use cookie::{Cookie, CookieJar, Key};

use crate::client::{Client, Method};
use anyhow::Result;

use crate::{map, types::Query};

mod types;
pub use self::types::*;

impl Client {
  pub async fn beginAuth(
    &self,
    shop: &str,
    redirect_path: &str,
    cookie_jar: &mut CookieJar,
  ) -> Result<String> {
    let key = Key::from(self.context.api_secret_key.as_bytes());

    let cookie = Cookie::build("shopify_app_session", "")
      .secure(true)
      .expires(OffsetDateTime::now_utc().checked_add(Duration::seconds(60000)))
      .same_site(cookie::SameSite::Lax)
      .finish();

    cookie_jar.signed_mut(&key).add(cookie);

    let query: Query = map! {
        "client_id" => self.context.api_key.clone(),
        "scope" => self.context.scopes.join(""),
        "redirect_uri" => format!("https://{}{}",shop,redirect_path),
        "state" => "".to_string(),
        "grant_options[]" => "per_user".to_string()
    }
    .into();

    Ok(format!(
      "https://${}/admin/oauth/authorize?{}",
      shop, query.0
    ))
  }
}
