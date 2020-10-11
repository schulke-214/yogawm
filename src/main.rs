#![feature(type_alias_impl_trait)]

mod macros;
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
use x11::windows::{get_all_windows, X11Window};

use state::WindowManagerState;
use error::{YogaError, YogaResult};

fn main() -> YogaResult<()> {
	let (connection, screen_num) = connect()?;
	let screen = get_screen(&connection, screen_num);

	manage_screen(&connection, &screen)?;

	let mut wm_state = WindowManagerState::new(&connection)?;

	wm_state.scan()?;

	loop {
		wm_state.refresh()?;
		println!("[yoga] flush connection");

		connection.flush()?;

		println!("[yoga] wait for event");

		let event = connection.wait_for_event()?;

		println!("[yoga] found event {:#?}", event);

		let mut event_option = Some(event);

		while let Some(event) = event_option {
			// wm_state.handle_event(event).unwrap();
			event_option = connection.poll_for_event()?;
		}
	}
}
