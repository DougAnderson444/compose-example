cargo_component_bindings::generate!();

use bindings::exports::aggregate::aggregate::controls::Guest;
use bindings::increment::increment::increase::{current, up};
use bindings::shower::show::display::view;

struct Component;

impl Guest for Component {
    fn more() {
        up();
    }

    fn current() -> u32 {
        current()
    }

    fn inspect() -> u32 {
        view()
    }
}
