#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio;

extern crate libosinfo_sys as ffi;

extern crate libc;
#[macro_use]
extern crate bitflags;

pub use glib::{
    Error,
    Object,
};

pub use auto::*;

pub mod signal {
    pub use glib::signal::Inhibit;
}

pub mod prelude {
    pub use auto::traits::*;
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

mod auto;
