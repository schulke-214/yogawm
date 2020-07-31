use libx::connect;

mod error;

use error::YogaError;

fn main() -> Result<(), YogaError> {
    let (_, screen) = connect()?;

    println!("Hello, world! Screen ID {}", screen);

    Ok(())
}
