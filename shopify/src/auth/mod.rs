use std::borrow::Cow;
use std::collections::HashMap;

use cookie::time::{Duration, OffsetDateTime};
use cookie::{Cookie, CookieJar, Key};
use uuid::Uuid;

use crate::client::{Client, Method};
use crate::session::types::{Session, SessionStorage};
use anyhow::Result;

use crate::{map, types::Query};

mod types;
pub use self::types::*;

const SESSION_KEY: &str = "shopify_app_session";

impl Client {
  pub async fn beginAuth(
    &mut self,
    shop: &str,
    redirect_path: &str,
    cookie_jar: &mut CookieJar,
  ) -> Result<String> {
    let key = Key::from(self.context.api_secret_key.as_bytes());

    let cookie = Cookie::build(SESSION_KEY, "")
      .secure(true)
      .expires(OffsetDateTime::now_utc().checked_add(Duration::seconds(60000)))
      .same_site(cookie::SameSite::Lax)
      .finish();

    let session = Session::new(Uuid::new_v4(), shop, "", true);

    self.context.session.store_session(session);

    cookie_jar.signed_mut(&key).add(cookie);

    let query: Query = map! {
        "client_id" => self.context.api_key.clone(),
        "scope" => self.context.scopes.join(","),
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

  pub async fn validateAuthCallback(
    &self,
    auth_query: AuthQuery,
    cookie_jar: &CookieJar,
  ) -> Result<()> {
    let session_cookie = cookie_jar
      .get(SESSION_KEY)
      .expect("Failed to get current session cookie");

    let body = AuthBody {
      client_id: self.context.api_key.clone(),
      client_secret: self.context.api_secret_key.clone(),
      code: auth_query.code,
    };

    let auth_options = AuthOptions {
      path: "/admin/oauth/access_token".to_string(),
      content_type: "application/json".to_string(),
      data: Cow::Borrowed(&body),
    };

    let request = self
      .request_raw(
        Method::POST,
        &auth_options.path,
        |b| {
          b.header("Content-Type", &auth_options.content_type)
            .json(&body)
        },
        false,
      )
      .await?;

    Ok(())
  }
}
