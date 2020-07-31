use libx::connect;
use libx::X11Error;

mod error;

use error::YogaError;

fn main() -> Result<(), YogaError> {
    let x = connect()?;

    println!("Hello, world! Screen ID {:#?}", x);

    Ok(())
}
