pub mod types;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use uuid::Uuid;

use self::types::*;

#[derive(Debug, Clone)]
pub struct MemorySession {
  sessions: Rc<RefCell<HashSet<Session>>>,
}

impl SessionStorage for MemorySession {
  fn store_session(&mut self, session: Session) -> bool {
    let sessions = Rc::get_mut(&mut self.sessions)
      .expect("failed to take get sessions")
      .get_mut();
    sessions.insert(session)
  }

  fn load_session(&mut self, id: Uuid) -> Option<Session> {
    let sessions = self.sessions.borrow();

    sessions.get(&id).cloned()
  }

  fn delete_session(&mut self, id: Uuid) -> bool {
    let sessions = Rc::get_mut(&mut self.sessions)
      .expect("failed to take get sessions")
      .get_mut();

    sessions.remove(&id)
  }
}

impl MemorySession {
  pub fn new() -> Self {
    Self {
      sessions: Rc::new(RefCell::new(HashSet::new())),
    }
  }
}
