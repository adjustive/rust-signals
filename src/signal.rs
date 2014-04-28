use std::vec::Vec;

use slot::Slot;

pub struct Signal {
    slots: Vec<Slot>
}

impl Signal {
    pub fn new() -> Signal {
        Signal {
            slots: Vec::new()
        }
    }

    pub fn connect(&mut self, slot: Slot) {
        self.slots.push(slot);
    }

    pub fn emit(&self) {
        for slot in self.slots.iter() {
            (slot.listener)();
        }
    }
}
