#![feature(lazy_cell)]

cargo_component_bindings::generate!();

use crate::bindings::exports::counter::count::change::Guest as ChangeGuest;
use crate::bindings::exports::counter::count::current::Guest as ShowGuest;

use std::sync::{LazyLock, Mutex};

static COUNT: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0u32));

struct Component;

impl ChangeGuest for Component {
    /// Increment the count
    fn increment() {
        let mut count = COUNT.lock().unwrap();
        *count += 1;
    }

    fn current() -> u32 {
        let count = COUNT.lock().unwrap();
        *count
    }
}

impl ShowGuest for Component {
    /// Show the count
    fn count() -> u32 {
        let count = COUNT.lock().unwrap();
        *count
    }
}
