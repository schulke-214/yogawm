#![feature(type_alias_impl_trait)]

mod error;
mod x11;

use x11::prelude::*;
use x11::connect;
use x11::screens::{get_screen, manage_screen};
use x11::windows::{get_all_windows, X11Window};

use error::YogaError;

type YogaResult<T> = Result<T, YogaError>;


struct WindowManagerState {
	active_screen: u32
	active_window: u32
	screens: Vec<Screen>
}

impl WindowManagerState {
	pub fn new(&connection: X11Connection) -> YogaError<Self> {
		let screens = get_all_screens();

		WindowManagerState {
			active_screen: 0,
			active_window: 0,
			screens
		}
	}

	pub fn scan() {
		println!("scan windows on all screens");
	}

	pub fn refresh() {
		println!("refresh wm");
	}
}

struct Screen {
	layout: Layout
	windows: Vec<Window>
}

impl Screen {
	pub fn refresh() {
		println!("refresh screen");
	}
}

struct Window {
	x11_window: &X11Window,
	position: (u32, u32)
}

impl Window {
	pub fn refresh() {
		println!("refresh window");
	}
}

struct Event {}

trait WindowManager {
	pub fn mount();
	pub fn handle_x11_event();
	pub fn handle_key_event();
}

trait Layout {}


fn main() -> Result<(), YogaError> {
	let (connection, screen_num) = connect()?;
	let screen = get_screen(&connection, screen_num);

	manage_screen(&connection, &screen)?;

	let mut wm_state = WindowManagerState::new(&connection);
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

