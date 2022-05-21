use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

use chrono::{DateTime, Duration as ChronoDuration, Utc};
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
  pub async fn begin_auth(
    &mut self,
    shop: &str,
    redirect_path: &str,
    cookie_jar: &mut CookieJar,
  ) -> Result<String> {
    let key = Key::derive_from(self.context.api_secret_key.as_bytes());

    let session = Session::new(Uuid::new_v4(), shop, "", true);

    let cookie = Cookie::build(SESSION_KEY, session.id.to_string())
      .secure(true)
      .expires(OffsetDateTime::now_utc().checked_add(Duration::seconds(60000)))
      .same_site(cookie::SameSite::Lax)
      .finish();

    self.context.session.store_session(session);

    cookie_jar.signed_mut(&key).add(cookie);

    let query: Query = map! {
        "client_id" => self.context.api_key.clone(),
        "scope" => self.context.scopes.join(","),
        "redirect_uri" => format!("https://{}{}",self.context.host_name,redirect_path),
        "state" => "".to_string(),
        "grant_options[]" => "per_user".to_string()
    }
    .into();

    Ok(format!(
      "https://{}/admin/oauth/authorize?{}",
      shop, query.0
    ))
  }

  pub async fn validate_auth_callback(
    &mut self,
    auth_query: AuthQuery,
    cookie_jar: &mut CookieJar,
  ) -> Result<Session> {
    let key = Key::derive_from(self.context.api_secret_key.as_bytes());

    let session_cookie = cookie_jar.signed_mut(&key)
      .get(SESSION_KEY)
      .expect("Failed to get current session cookie");

    let id = session_cookie.value();

    let mut current_session = self
      .context
      .session
      .load_session(Uuid::from_str(id)?)
      .expect(&format!("failed to load session by this id {}", id));

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

    let body = request.json::<AuthResponse>().await?;

    let (access_token, scope) = (
      body.access_token_response.access_token,
      body.access_token_response.scope,
    );

    let session_expiration = Utc::now()
      .checked_add_signed(ChronoDuration::seconds(body.online_accesss_info.expires_in))
      .expect("failed to add seconds to expiration")
      .checked_add_signed(ChronoDuration::seconds(1000))
      .expect("failed to add seconds to expiration");

    current_session.access_token = Some(access_token);
    current_session.expires = Some(session_expiration);
    current_session.scope = Some(scope);
    current_session.online_access_info = Some(body.online_accesss_info);

    let cookie = Cookie::build(SESSION_KEY, current_session.id.to_string())
      .secure(true)
      .same_site(cookie::SameSite::Lax)
      .expires(OffsetDateTime::from_unix_timestamp(
        session_expiration.timestamp_millis(),
      )?)
      .finish();

    cookie_jar.signed_mut(&key).add(cookie);

    self.context.session.store_session(current_session.clone());

    Ok(current_session)
  }
}
