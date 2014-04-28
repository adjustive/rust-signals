extern crate signals;

use signals::slot::Slot;
use signals::signal::Signal;

fn handle_change() {
    println!("Things have changed");
}

fn more_change() {
    println!("I've been changed again");
}

fn main() {
    let mut changed = Signal::new();
    let mut changed_again = Signal::new();

    let b = Slot::new(handle_change);
    let c = Slot::new(more_change);

    changed.connect(b);
    changed.connect(c);
    changed_again.connect(c);
    changed_again.connect(b);

    changed.emit();
    changed_again.emit();
}
