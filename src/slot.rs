use uuid::Uuid;

use std::hash::Hash;
use std::hash::sip::SipState;

pub struct Slot {
    pub listener: fn(),
    uuid: Uuid,
    run_once: bool
}

impl Slot {
    pub fn new(listener: fn()) -> Slot {
        Slot {
            listener: listener,
            uuid: Uuid::new_v4(),
            run_once: false
        }
    }
}

impl Eq for Slot {
    fn eq(&self, other: &Slot) -> bool {
        self.uuid == other.uuid
    }
}

impl Hash for Slot {
    fn hash(&self, state: &mut SipState) {
        self.uuid.hash(state)
    }
}

impl TotalEq for Slot {
}
