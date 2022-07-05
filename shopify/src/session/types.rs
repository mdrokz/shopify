use std::{borrow::Borrow, hash::Hash};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[cfg(feature = "openapi")]
use schemars::JsonSchema;

#[cfg(feature = "sqlx")]
use sqlx::FromRow;


pub trait SessionStorage {
  fn store_session(&mut self, session: Session) -> bool;
  fn load_session(&mut self, id: Uuid) -> Option<Session>;
  fn delete_session(&mut self, id: Uuid) -> bool;
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Hash, PartialEq, Eq,Debug, Serialize, Deserialize, Default, Clone)]
pub struct AssociatedUser {
  pub id: u64,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub email_verified: bool,
  pub account_owner: bool,
  pub locale: String,
  pub collaborator: bool,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Hash, PartialEq, Eq,Debug, Serialize, Deserialize, Default, Clone)]
pub struct OnlineAccessInfo {
  pub expires_in: i64,
  pub associated_user_scope: String,
  pub associated_user: AssociatedUser,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[derive(Hash, PartialEq, Eq,Debug, Serialize, Deserialize, Clone)]
pub struct Session {
  pub id: Uuid,
  pub shop: String,
  pub state: String,
  pub is_online: bool,
  pub scope: Option<String>,
  pub expires: Option<DateTime<Utc>>,
  pub access_token: Option<String>,
  pub online_access_info: Option<OnlineAccessInfo>,
}

impl Borrow<Uuid> for Session {
  fn borrow(&self) -> &Uuid {
    &self.id
  }
}

impl Default for Session {
  fn default() -> Self {
    Self {
      id: Default::default(),
      shop: Default::default(),
      state: Default::default(),
      is_online: Default::default(),
      scope: Default::default(),
      expires: Default::default(),
      access_token: Default::default(),
      online_access_info: Default::default(),
    }
  }
}

impl Session {
  pub fn is_active(&self) -> bool {
    true
  }

  pub fn new(id: Uuid, shop: &str, state: &str, is_online: bool) -> Self {
    Self {
      id,
      shop: shop.to_string(),
      state: state.to_string(),
      is_online,
      ..Default::default()
    }
  }
}
