// this lib will contain easy bindings for creating, moving and closing x windows
// probably using the following crate: https://docs.rs/x11rb/0.5.0/x11rb/

pub mod error;
pub use error::X11Error;

use x11rb::connection::Connection;

pub type X11DisplayConnection = usize;
pub type X11DisplayScreenNum = usize;
pub type X11Result<T> = Result<T, X11Error>;

pub fn connect() -> X11Result<(impl Connection + Sync + Send, X11DisplayScreenNum)> {
    Ok(x11rb::connect(None)?)
}

