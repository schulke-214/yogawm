#![allow(unused)]

use log::{info};

use crate::x11::prelude::*;
use crate::error::YogaResult;
use crate::window::Window;

pub fn handle_configure_request(connection: &Connection, event: ConfigureRequestEvent) -> YogaResult<Window> {
	debug!("handle configure event: {:?}", event);

	let mut aux = ConfigureWindowAux::default();

	if event.value_mask & u16::from(ConfigWindow::X) != 0 {
		aux = aux.x(i32::from(event.x));
	}

	if event.value_mask & u16::from(ConfigWindow::Y) != 0 {
		aux = aux.y(i32::from(event.y));
	}

	if event.value_mask & u16::from(ConfigWindow::Width) != 0 {
		aux = aux.width(u32::from(event.width));
	}

	if event.value_mask & u16::from(ConfigWindow::Height) != 0 {
		aux = aux.height(u32::from(event.height));
	}

	&connection.configure_window(event.window, &aux)?;

	info!("configure window: {:?}", aux);

	Ok(Window {})
}
