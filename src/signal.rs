use collections::hashmap::HashSet;

use slot::Slot;

pub struct Signal {
    slots: HashSet<Slot>
}

impl Signal {
    pub fn new() -> Signal {
        Signal {
            slots: HashSet::<Slot>::new()
        }
    }

    pub fn connect(&mut self, slot: Slot) {
        self.slots.insert(slot);
    }

    pub fn emit(&self) {
        for s in self.slots.iter() {
            (s.listener)();
        }
    }

    pub fn disconnect(&mut self, slot: Slot) {
        self.slots.remove(&slot);
    }

    pub fn disconnect_all(&mut self) {
        self.slots.clear();
    }
}
