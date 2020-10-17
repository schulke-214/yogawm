#![allow(unused)]

use log::{debug};

use crate::error::YogaResult;
use crate::window::Window;

pub struct Screen {
	// layout: Layout
	windows: Vec<Window>
}

impl Screen {
	pub fn refresh(&self) -> YogaResult<()> {
		debug!("refresh screen");
		Ok(())
	}
}
