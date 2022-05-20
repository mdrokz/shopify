pub mod types;

// use std::cell::RefCell;
use std::collections::HashSet;
// use std::rc::Rc;

use std::sync::{Arc, Mutex};

use uuid::Uuid;

use self::types::*;

#[derive(Debug, Clone)]
pub struct MemorySession {
  sessions: Arc<Mutex<HashSet<Session>>>,
}

impl SessionStorage for MemorySession {
  fn store_session(&mut self, session: Session) -> bool {
    let sessions = &mut *self.sessions.lock().expect("failed to get session lock");
    // let mut sessions = *session_guard;
    sessions.insert(session)
  }

  fn load_session(&mut self, id: Uuid) -> Option<Session> {
    let sessions = self.sessions.lock().expect("failed to get session lock");
    sessions.get(&id).cloned()
  }

  fn delete_session(&mut self, id: Uuid) -> bool {
    let sessions = &mut *self.sessions.lock().expect("failed to get session lock");

    sessions.remove(&id)
  }
}

impl MemorySession {
  pub fn new() -> Self {
    Self {
      sessions: Arc::new(Mutex::new(HashSet::new())),
    }
  }
}
