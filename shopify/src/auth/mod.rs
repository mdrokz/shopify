use std::collections::HashMap;

use cookie::time::{Duration, OffsetDateTime};
use cookie::{Cookie, CookieBuilder, CookieJar, Key};

use crate::client::{Client, Method};
use crate::result::*;

mod types;
pub use self::types::*;


pub struct Query(String);

trait X {}

impl Into<Query> for HashMap<String,String> {

    fn into(self) -> Query {
        todo!()
    } 
}

impl Client {
  pub async fn beginAuth(&self, shop: &str, redirectPath: &str, cookieJar: &mut CookieJar) {
    let key = Key::from(self.context.api_secret_key.as_bytes());

    let cookie = Cookie::build("shopify_app_session", "")
      .secure(true)
      .expires(OffsetDateTime::now_utc().checked_add(Duration::seconds(60000)))
      .same_site(cookie::SameSite::Lax)
      .finish();

    cookieJar.signed_mut(&key).add(cookie)
  }
}
