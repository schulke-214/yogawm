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

use x11rb::protocol::xproto::ConnectionExt as X11ConnectionExt;
use x11rb::connection::Connection as X11RawConnection;
use x11rb::connection::RequestConnection as X11RequestConnection;

/// Holds a Connection to a X11 Server.
pub type X11Connection = impl X11RawConnection + X11RequestConnection + X11ConnectionExt + Send + Sync;

/// A generic result type used for all kinds of unsafe X Operations.
pub type X11Result<T> = Result<T, X11Error>;

/// Using this function you can establish a connection to the X11 Server.
pub fn connect() -> X11Result<(X11Connection, self::screens::X11ScreenNum)> {
    Ok(x11rb::connect(None)?)
}

/// This module should be imported to ensure all dependencies are loaded correctly when using
/// other parts of this library.
pub mod prelude {
    /// This is the Connecton trait which should be in scope when accessing nodes of it.
    pub use x11rb::connection::Connection as X11RawConnection;
    pub use x11rb::protocol::xproto::ConnectionExt as X11ConnectionExt;
    pub use super::X11Connection;
    pub use super::screens::X11ScreenNum;
    pub use super::windows::X11WindowId;
}

/// Contains all screen management related components.
pub mod screens {
    use super::prelude::*;
    pub use x11rb::protocol::xproto::Screen as X11Screen;

    /// Holds the id of a specific screen.
    pub type X11ScreenNum = usize;

    pub fn get_all_screens(connection: &X11Connection) -> &Vec<X11Screen> {
        &connection.setup().roots
    }

    /// A utility for getting information about a specific screen.
    pub fn get_screen(connection: &X11Connection, screen_num: X11ScreenNum) -> &X11Screen {
        &get_all_screens(connection)[screen_num]
    }
}

/// Contains all window management related components.
pub mod windows {
    use super::prelude::*;
    use super::screens::X11Screen;
    use super::X11Result;

    use x11rb::protocol::xproto::MapState;

    pub use x11rb::protocol::xproto::Window as X11WindowId;
    pub use x11rb::protocol::xproto::GetGeometryReply as X11GetGeometryReply;
    pub use x11rb::protocol::xproto::GetWindowAttributesReply as X11GetWindowAttributesReply;
    pub use x11rb::properties::WmClass as X11WmClass;

    #[derive(Debug)]
    pub struct X11Window {
        pub id: X11WindowId,
        pub attributes: X11GetWindowAttributesReply,
        pub geometry: X11GetGeometryReply,
        pub wm_class: Option<X11WmClass>
    }

    impl X11Window {
        pub fn new(
            id: X11WindowId,
            attributes: X11GetWindowAttributesReply,
            geometry: X11GetGeometryReply,
            wm_class: Option<X11WmClass>
        ) -> Self {
            X11Window {
                id,
                attributes,
                geometry,
                wm_class
            }
        }

        pub fn get_wm_class(&self) -> Option<String> {
            match &self.wm_class {
                Some(class) => match std::str::from_utf8(class.class()) {
                    Ok(res) => Some(res.to_owned()),
                    Err(_) => None
                },
                None => None
            }
        }

        pub fn get_wm_class_instance(&self) -> Option<String> {
            match &self.wm_class {
                Some(class) => match std::str::from_utf8(class.instance()) {
                    Ok(res) => Some(res.to_owned()),
                    Err(_) => None
                },
                None => None
            }
        }
    }

    pub fn get_windows(connection: &X11Connection, screen: &X11Screen) -> X11Result<Vec<X11Window>> {
        let tree_reply = connection.query_tree(screen.root)?.reply()?;

        let mut cookies = Vec::with_capacity(tree_reply.children.len());
        for win in tree_reply.children {
            let attr = connection.get_window_attributes(win)?;
            let geom = connection.get_geometry(win)?;
            let class = X11WmClass::get(connection, win)?;
            cookies.push((win, attr, geom, class));
        }

        let mut windows: Vec<X11Window> = Vec::with_capacity(cookies.len());
        for (win, attr, geom, class) in cookies {
            let (attr, geom, class) = (attr.reply(), geom.reply(), class.reply_unchecked());

            if attr.is_err() || geom.is_err() || class.is_err() {
                continue; // Just skip this window
            }

            let (attr, geom, class) = (attr.unwrap(), geom.unwrap(), class.unwrap());

            if !attr.override_redirect && attr.map_state != MapState::Unmapped {
                windows.push(X11Window::new(win, attr, geom, class));
            }
        }

        Ok(windows)
    }
}

/// Contains all components which are related to io and event handling.
pub mod io {
    pub mod events {}
}

/// Contains utilities to draw to X Screens.
pub mod gfx {}

