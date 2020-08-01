use libx::prelude::*;
use libx::connect;

mod error;

use error::YogaError;

fn main() -> Result<(), YogaError> {
    let (connection, screen_num) = connect()?;
    let screen = &connection.setup().roots[screen_num];

    println!("Hello, world! Screen ID {}", screen_num);

    Ok(())
}
