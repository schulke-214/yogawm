//! This library contains highlevel bindings to the `x11rb` crate and is intendet to
//! be used by with the `yoga` window manager (so only the for window manager relevant
//! parts of the api are implemented) alltough its absoluetly possible to use it in
//! other projects aswell.
//!
//! If you're new in working with x11 you should probably take a look at
//! ![https://tronche.com/gui/x/xlib/display/opening.html](this)
//!
//! `libx` can be broken up into 4 important parts:
//! - Connection / Screen Management
//! - Window Management
//! - Input / Event Handling
//! - Graphics
//!
//! Each of these parts has it's own module whith further documentation.

#![feature(type_alias_impl_trait)]

pub mod error;
pub use error::X11Error;

use x11rb::connection::Connection as X11RawConnection;

/// Holds a Connection to a X11 Server.
pub type X11Connection = impl X11RawConnection + Send + Sync;

/// Holds the id of a specific screen.
pub type X11ScreenNum = usize;

/// A generic result type used for all kinds of unsafe X Operations.
pub type X11Result<T> = Result<T, X11Error>;

/// Using this function you can establish a connection to the X11 Server.
pub fn connect() -> X11Result<(X11Connection, X11ScreenNum)> {
    Ok(x11rb::connect(None)?)
}

/// This module should be imported to ensure all dependencies are loaded correctly when using
/// other parts of this library.
pub mod prelude {
    /// This is the Connecton trait which should be in scope when accessing nodes of it.
    pub use x11rb::connection::Connection as X11RawConnection;
    pub use super::X11Connection;
    pub use super::X11ScreenNum;
    pub use super::X11Result;
}

/// Contains all screen management related components.
pub mod screens {
    use super::prelude::*;
    pub use x11rb::protocol::xproto::Screen as X11Screen;

    pub fn get_all_screens(connection: &X11Connection) -> &Vec<X11Screen> {
        &connection.setup().roots
    }

    /// A utility for getting information about a specific screen.
    pub fn get_screen(connection: &X11Connection, screen_num: X11ScreenNum) -> &X11Screen {
        &get_all_screens(connection)[screen_num]
    }
}

/// Contains all window management related components.
pub mod windows {}

/// Contains all components which are related to io and event handling.
pub mod io {
    pub mod events {}
}

/// Contains utilities to draw to X Screens.
pub mod gfx {}

