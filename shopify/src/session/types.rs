pub trait SessionStorage {
  fn storeSession(session: Session) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociatedUser {
    id: u32,
    first_name: String,
    last_name: String,
    email: String,
    email_verified: bool,
    account_owner: bool,
    locale: String,
    collaborator: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineAccessInfo {
    expires_in: i32,
    associated_user_scope: String,
    associated_user: AssociatedUser
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
  id: String,
  shop: String,
  state: String,
  is_online: bool,
  scope: Option<String>,
  expires: Option<DateTime<Utc>>,
  access_token: Option<String>,
  online_access_info: Option<OnlineAccessInfo>,
}

impl Session {
  fn isActive() -> bool {
    true
  }
}
