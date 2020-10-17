#![allow(unused)]

use log::{debug};

use crate::x11::prelude::*;
use crate::error::{YogaError, YogaResult};

pub struct WindowManagerState {
// active_screen: u32,
// active_window: u32
// screens: Vec<Screen>
}

impl WindowManagerState {
	pub fn new(connection: &X11Connection) -> YogaResult<Self> {
		// let screens = get_all_screens();

		Ok(WindowManagerState {
			// active_screen: 0,
			// active_window: 0,
			// screens
		})
	}

	pub fn scan(&mut self) -> YogaResult<()> {
		debug!("scan windows on all screens");
		Ok(())
	}

	pub fn refresh(&mut self) -> YogaResult<()> {
		debug!("refresh wm");
		Ok(())
	}
}