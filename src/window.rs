#![allow(unused)]

use log::{debug};

use crate::error::YogaResult;

pub struct Window {
	// position: (u32, u32)
}

impl Window {
	pub fn refresh() -> YogaResult<()> {
		debug!("refresh window");
		Ok(())
	}
}