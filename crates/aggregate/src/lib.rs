cargo_component_bindings::generate!();

use bindings::exports::aggregate::aggregate::controls::Guest;
use bindings::increment::increment::increase::up;
use bindings::shower::show::display::view;

struct Component;

impl Guest for Component {
    fn more() {
        up();
    }

    fn inspect() -> u32 {
        view()
    }
}
