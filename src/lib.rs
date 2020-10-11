#![feature(type_alias_impl_trait)]

mod macros;
pub mod error;
pub mod event;
pub mod layout;
pub mod screen;
pub mod window;
pub mod state;
pub mod x11;

pub use x11::prelude::*;

use error::YogaResult;

pub trait WindowManager {
	fn mount(&self) -> YogaResult<()>;
	fn handle_x11_event(&self) -> YogaResult<()>;
	fn handle_key_event(&self) -> YogaResult<()>;
}
