//! This library contains highlevel bindings to the `x11rb` crate and is intendet to
//! be used by with the `yoga` window manager (so only the for window manager relevant
//! parts of the api are implemented) alltough its absoluetly possible to use it in
//! other projects aswell.
//!
//! If you're new in working with x11 you should probably take a look at
//! ![https://tronche.com/gui/x/xlib/display/opening.html](this)
//!
//! `libx11` can be broken up into 4 important parts:
//! - Connection / Screen Management
//! - Window Management
//! - Input / Event Handling
//! - Graphics
//!
//! Each of these parts has it's own module whith further documentation.

#![allow(dead_code)]

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
	/// The ConnectionExt trait is providing utils to get detailed window information.
	pub use x11rb::protocol::xproto::ConnectionExt as X11ConnectionExt;

	pub use super::X11Connection;
	pub use super::screens::X11ScreenNum;
	pub use super::windows::X11WindowId;
}

/// Contains all screen management related components.
pub mod screens {
	use log::{error};

	use super::prelude::*;
	use super::X11Result;
	use super::X11Error;

	use x11rb::errors::ReplyError;
	use x11rb::protocol::Error;
	use x11rb::protocol::xproto::ChangeWindowAttributesAux;
	use x11rb::protocol::xproto::EventMask;

	/// This struct represents a single screen and holds meta data about it.
	pub use x11rb::protocol::xproto::Screen as X11Screen;

	/// Holds the id of a specific screen.
	pub type X11ScreenNum = usize;

	/// Turns a Connection into a vector of screens.
	pub fn get_all_screens(connection: &X11Connection) -> &Vec<X11Screen> {
		&connection.setup().roots
	}

	/// A utility for getting information about a specific screen.
	pub fn get_screen(connection: &X11Connection, screen_num: X11ScreenNum) -> &X11Screen {
		&get_all_screens(connection)[screen_num]
	}

	/// Try to become the window manager of all available screens.
	pub fn manage_all_screens(connection: &X11Connection) -> X11Result<()> {
		unimplemented!();

		// let screens = get_all_screens(&connection);

		// screens
		// 	.iter()
		// 	.map(|screen| manage_screen(&connection, &screen));

		// Ok(())
	}

	/// Try to become the window manager of the given screen.
	pub fn manage_screen(connection: &X11Connection, screen: &X11Screen) -> X11Result<()> {
		// Try to become the window manager. This causes an error if there is already another WM.
		let change = ChangeWindowAttributesAux::default()
			.event_mask(EventMask::SubstructureRedirect | EventMask::SubstructureNotify);

		let res = connection.change_window_attributes(screen.root, &change)?.check();

		if let Err(e) = res {
			if let ReplyError::X11Error(Error::Access(_)) = e {
				error!("[x11] unable to manage screen - there is probably another window manager running.");
				return Err(e.into());
			}
		}

		Ok(())
	}
	
}

/// Contains all window management related components.
pub mod windows {
	use log::{debug};

	use super::prelude::*;
	use super::screens::X11Screen;
	use super::X11Result;

	use x11rb::protocol::xproto::SetMode;
	use x11rb::protocol::xproto::MapState;
	use x11rb::protocol::xproto::QueryTreeReply;

	/// A Window ID. Needs to be provided to most X API-Calls.
	pub use x11rb::protocol::xproto::Window as X11WindowId;

	/// A struct which holds geometry data about a specific window.
	pub use x11rb::protocol::xproto::GetGeometryReply as X11GetGeometryReply;

	/// A struct which holds all attributes of a specific window.
	pub use x11rb::protocol::xproto::GetWindowAttributesReply as X11GetWindowAttributesReply;

	/// A utility to access the `WM_CLASS` prop of windows.
	pub use x11rb::properties::WmClass as X11WmClass;

	/// A X11Window combines multiple data structures about the same window and
	/// implements some utils for working with them.
	#[derive(Debug)]
	pub struct X11Window {
		pub id: X11WindowId,
		pub attributes: X11GetWindowAttributesReply,
		pub geometry: X11GetGeometryReply,
		pub wm_class: Option<X11WmClass>
	}

	impl X11Window {
		/// Creates a new X11Window
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

		/// Returns the second item of the WM_CLASS prop of a window if it's present.
		/// This is the "type" of a window. This is useful for targeting specific windows.
		pub fn get_wm_class(&self) -> Option<String> {
			match &self.wm_class {
				Some(class) => match std::str::from_utf8(class.class()) {
					Ok(res) => Some(res.to_owned()),
					Err(_) => None
				},
				None => None
			}
		}

		/// Returns first item of the WM_CLASS prop of a window if it's present.
		/// This is the instance name of a window. You probably wont need this.
		pub fn get_wm_class_instance(&self) -> Option<String> {
			match &self.wm_class {
				Some(class) => match std::str::from_utf8(class.instance()) {
					Ok(res) => Some(res.to_owned()),
					Err(_) => None
				},
				None => None
			}
		}

		/// Retrives all children of a given window.
		pub fn get_subwindows(&self, connection: &X11Connection) -> X11Result<Vec<X11Window>> {
			// Loads all children of the given window.
			let tree_reply = connection.query_tree(self.id)?.reply()?;

			query_tree_reply_to_windows(connection, tree_reply, false)
		}

		/// Destroys the current window if its not already destroyed.
		pub fn destroy(&self, connection: &X11Connection) -> X11Result<()> {
			debug!("Destroy: {} {}", self.id, self.get_wm_class().unwrap());
			connection.change_save_set(SetMode::Delete, self.id)?;
			connection.destroy_window(self.id)?;

			Ok(())
		}

		/// Destroyes all subwindows if possible.
		pub fn destroy_subwindows(&self, connection: &X11Connection) -> X11Result<()> {
			connection.destroy_subwindows(self.id)?;

			Ok(())
		}

		/// Unmaps ("hides") the current window if possible.
		pub fn unmap(&self, connection: &X11Connection) -> X11Result<()> {
			connection.unmap_window(self.id)?;

			Ok(())
		}

		/// Maps ("shows") the current window if possible.
		pub fn map(&self, connection: &X11Connection) -> X11Result<()> {
			connection.map_window(self.id)?;

			Ok(())
		}

		/// Maps ("shows") the subwindow of the current window if possible.
		pub fn map_subwindows(&self, connection: &X11Connection) -> X11Result<()> {
			connection.map_subwindows(self.id)?;

			Ok(())
		}

		/// Unmaps ("hides") the subwindow of the current window if possible.
		pub fn unmap_subwindows(&self, connection: &X11Connection) -> X11Result<()> {
			connection.unmap_subwindows(self.id)?;

			Ok(())
		}

	}

	/// Turns a QueryTree into a vector of X11Windows.
	fn query_tree_reply_to_windows(connection: &X11Connection, reply: QueryTreeReply, show_mapped_windows: bool) -> X11Result<Vec<X11Window>> {
		// Turns them into X-Cookies and fetches data about each window.
		let mut cookies = Vec::with_capacity(reply.children.len());
		for win in reply.children {
			let attr = connection.get_window_attributes(win)?;
			let geom = connection.get_geometry(win)?;
			let class = X11WmClass::get(connection, win)?;
			cookies.push((win, attr, geom, class));
		}

		// Transforms "unmanaged" windows into a X11Window and returns them.
		let mut windows: Vec<X11Window> = Vec::with_capacity(cookies.len());
		for (win, attr, geom, class) in cookies {
			let (attr, geom, class) = (attr.reply(), geom.reply(), class.reply_unchecked());

			// Checking for any errors. We skip a window if we cant get all information about it.
			if attr.is_err() || geom.is_err() || class.is_err() {
				continue;
			}

			// It's safe to unwrap now because we checked for errors already.
			let (attr, geom, class) = (attr.unwrap(), geom.unwrap(), class.unwrap());
			let xwin = X11Window::new(win, attr, geom, class);

			// If we should take all windows into account, we just use it now
			// without any other checks.
			if show_mapped_windows {
				windows.push(xwin);
			}
			// else we check for unmanaged or "unmapped" windows and take only those
			// into account which fullfill these requirements.
			else if !xwin.attributes.override_redirect && xwin.attributes.map_state != MapState::Unmapped {
				windows.push(xwin);
			}
		}

		Ok(windows)
	}

	/// Fetches all windows which exist on a given screen and turns them into a X11Window.
	pub fn get_all_windows(connection: &X11Connection, screen: &X11Screen) -> X11Result<Vec<X11Window>> {
		// Loads all children of the current screen.
		let tree_reply = connection.query_tree(screen.root)?.reply()?;

		query_tree_reply_to_windows(connection, tree_reply, true)
	}

	/// Fetches windows which are currently unmappped on a given screen and turns them into a X11Window.
	pub fn get_unmapped_windows(connection: &X11Connection, screen: &X11Screen) -> X11Result<Vec<X11Window>> {
		// Loads all children of the current screen.
		let tree_reply = connection.query_tree(screen.root)?.reply()?;

		query_tree_reply_to_windows(connection, tree_reply, false)
	}
}

/// Contains all components which are related to io and event handling.
pub mod io {
	pub mod events {}
}

/// Contains utilities to draw to X Screens.
pub mod gfx {}

