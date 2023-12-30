cargo_component_bindings::generate!();

use bindings::exports::aggregate::aggregate::controls::Guest;

struct Component;

impl Guest for Component {
    fn more() {
        println!("More!");
    }

    fn inspect() -> u32 {
        42
    }
}
