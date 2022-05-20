use chrono::{DateTime, Utc};
use std::{borrow::Borrow, hash::Hash};
use uuid::Uuid;

pub trait SessionStorage {
  fn store_session(&mut self, session: Session) -> bool;
  fn load_session(&mut self, id: Uuid) -> Option<Session>;
  fn delete_session(&mut self, id: Uuid) -> bool;
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct AssociatedUser {
  id: u32,
  first_name: String,
  last_name: String,
  email: String,
  email_verified: bool,
  account_owner: bool,
  locale: String,
  collaborator: bool,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct OnlineAccessInfo {
  expires_in: i32,
  associated_user_scope: String,
  associated_user: AssociatedUser,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Session {
  id: Uuid,
  shop: String,
  state: String,
  is_online: bool,
  scope: Option<String>,
  expires: Option<DateTime<Utc>>,
  access_token: Option<String>,
  online_access_info: Option<OnlineAccessInfo>,
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
  pub fn isActive(&self) -> bool {
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
