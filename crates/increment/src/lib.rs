cargo_component_bindings::generate!();

use crate::bindings::exports::increment::increment::increase::Guest;
use bindings::counter::count::change::{current, increment};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn up() {
        increment();
    }

    fn current() -> u32 {
        current()
    }
}
