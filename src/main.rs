#![feature(type_alias_impl_trait)]

mod error;
mod x11;

use x11::prelude::*;
use x11::connect;
use x11::screens::{get_screen, manage_screen};
use x11::windows::get_all_windows;

use error::YogaError;


struct WindowManagerState {}
struct Window {}
struct Event {}
trait WindowManager {}
trait Layout {}


fn main() -> Result<(), YogaError> {
	let (connection, screen_num) = connect()?;
	let screen = get_screen(&connection, screen_num);

	manage_screen(&connection, &screen)?;

	// let mut wm_state = WMState::new().unwrap();
	// wm_state.scan_windows().unwrap();

	loop {
		// wm_state.refresh().unwrap();
		println!("[yoga] flush connection");

		connection.flush().unwrap();

		println!("[yoga] wait for event");

		let event = connection.wait_for_event().unwrap();

		println!("[yoga] found event {:#?}", event);

		let mut event_option = Some(event);

		while let Some(event) = event_option {

			// wm_state.handle_event(event).unwrap();
			event_option = connection.poll_for_event().unwrap();
		}
	}
}

