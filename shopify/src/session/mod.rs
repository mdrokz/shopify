pub mod types;

use std::cell::RefCell;
use std::collections::HashSet;

use uuid::Uuid;

use self::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySession {
  sessions: RefCell<HashSet<Session>>,
}

impl SessionStorage for MemorySession {
  fn store_session(&mut self, session: Session) -> bool {
    let sessions = self.sessions.get_mut();
    sessions.insert(session)
  }

  fn load_session(&mut self, id: Uuid) -> Option<Session> {
    let sessions = self.sessions.borrow();

    sessions.get(&id).cloned()
  }

  fn delete_session(&mut self, id: Uuid) -> bool {
    let sessions = self.sessions.get_mut();

    sessions.remove(&id)
  }
}

impl MemorySession {
  pub fn new() -> Self {
    Self {
      sessions: RefCell::new(HashSet::new()),
    }
  }
}
