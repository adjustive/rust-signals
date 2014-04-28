pub struct Slot {
    pub listener: fn()
}

impl Slot {
    pub fn new(listener: fn()) -> Slot {
        Slot {
            listener: listener
        }
    }
}
