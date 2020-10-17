use transitive_from::hierarchy;

use crate::x11::X11Error;
use crate::x11::error::internal::{
	ConnectError,
	ConnectionError,
	ParseError,
	ReplyError,
	ReplyOrIdError
};

#[derive(Debug)]
pub enum YogaError {
	X11Error(X11Error)
}

impl From<X11Error> for YogaError {
	fn from(error: X11Error) -> Self {
		YogaError::X11Error(error)
	}
}

hierarchy! {
	YogaError {
		X11Error {
			ConnectError,
			ConnectionError,
			ParseError,
			ReplyError,
			ReplyOrIdError
		}
	}
}

pub type YogaResult<T> = Result<T, YogaError>;
