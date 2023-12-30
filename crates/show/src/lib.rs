cargo_component_bindings::generate!();

use crate::bindings::exports::shower::show::display::Guest;
use bindings::counter::count::current::count;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn view() -> u32 {
        count()
    }
}
