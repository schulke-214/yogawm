use libx11::X11Error;

#[derive(Debug)]
pub enum YogaError {
    X11Error(X11Error)
}

impl From<X11Error> for YogaError {
    fn from(error: X11Error) -> Self {
        YogaError::X11Error(error)
    }
}

