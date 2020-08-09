use x11rb::errors::{
    ConnectError,
    ConnectionError,
    ParseError,
    ReplyError,
    ReplyOrIdError
};

#[derive(Debug)]
pub enum X11Error {
    ConnectError(ConnectError),
    ConnectionError(ConnectionError),
    ParseError(ParseError),
    ReplyError(ReplyError),
    ReplyOrIdError(ReplyOrIdError)
}

impl From<ConnectError> for X11Error {
    fn from(error: ConnectError) -> Self {
        X11Error::ConnectError(error)
    }
}

impl From<ConnectionError> for X11Error {
    fn from(error: ConnectionError) -> Self {
        X11Error::ConnectionError(error)
    }
}

impl From<ParseError> for X11Error {
    fn from(error: ParseError) -> Self {
        X11Error::ParseError(error)
    }
}

impl From<ReplyError> for X11Error {
    fn from(error: ReplyError) -> Self {
        X11Error::ReplyError(error)
    }
}

impl From<ReplyOrIdError> for X11Error {
    fn from(error: ReplyOrIdError) -> Self {
        X11Error::ReplyOrIdError(error)
    }
}
