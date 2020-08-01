//! This library contains highlevel bindings to the `x11rb` crate and is intendet to
//! be used by with the `yoga` window manager (so only the for window manager relevant
//! parts of the api are implemented) alltough its absoluetly possible to use it in
//! other projects aswell.
//!
//! If you're new in working with x11 you should probably take a look at
//! ![https://tronche.com/gui/x/xlib/display/opening.html](this)
//!
//! `libx` can be broken up into 4 important parts:
//! - Display Management
//! - Window Management
//! - Input / Event Handling
//! - Drawing
//!
//! Each of these parts has it's own module whith further documentation.

#![feature(type_alias_impl_trait)]

pub mod error;
pub use error::X11Error;

use x11rb::connection::Connection as X11RawConnection;

/// Holds a Connection to a X11 Server.
pub type X11Connection = impl X11RawConnection + Send + Sync;

/// Holds the id of a specific screen.
pub type X11DisplayScreenNum = usize;

/// A generic result type used for all kinds of unsafe X Operations.
pub type X11Result<T> = Result<T, X11Error>;

/// Using this function you can establish a connection to the X11 Server.
pub fn connect() -> X11Result<(X11Connection, X11DisplayScreenNum)> {
    Ok(x11rb::connect(None)?)
}

pub mod prelude {
    pub use x11rb::connection::Connection as X11RawConnection;
}
