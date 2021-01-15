#![feature(type_alias_impl_trait)]

use log::{debug, info};
use env_logger;

mod error;
mod event;
mod layout;
mod screen;
mod window;
mod state;
mod x11;

use x11::prelude::*;
use x11::connect;
use x11::screens::{get_screen, manage_screen};

use state::WindowManagerState;
use error::{YogaResult};

fn main() -> YogaResult<()> {
	env_logger::init();

	let (connection, screen_num) = connect()?;
	let screen = get_screen(&connection, screen_num);

	info!("connected succesfully to screen {}", screen_num);

	manage_screen(&connection, &screen)?;

	let mut wm_state = WindowManagerState::new(&connection)?;

	wm_state.scan()?;

	loop {
		wm_state.refresh()?;
		debug!("flush connection");

		connection.flush()?;

		debug!("wait for event");

		let event = connection.wait_for_event()?;
		let mut event_option = Some(event);

		while let Some(event) = event_option {
			debug!("found event {:#?}", event);
			
			// wm_state.handle_event(event).unwrap();
			event_option = connection.poll_for_event()?;
		}
	}
}
