use libx::prelude::*;
use libx::connect;
use libx::screens::get_screen;
use libx::windows::get_all_windows;

mod error;

use error::YogaError;

/*
struct WindowManagerState {}
struct Window {}
struct Event {}
trait WindowManager {}
trait Layout {}
*/

fn main() -> Result<(), YogaError> {
    let (connection, screen_num) = connect()?;
    let screen = get_screen(&connection, screen_num);

    println!("screen = {} x {}", screen.width_in_pixels, screen.height_in_pixels);

    loop {
        let windows = get_all_windows(&connection, &screen);

        for win in windows.unwrap().iter() {
            println!("window 0 {:#?}", win.get_wm_class());

            if win.get_wm_class().unwrap() == "XTerm" {
                println!("FOUND xterm :)");
                println!("KILL xterm !");

                win.destroy(&connection)?;
                connection.flush().unwrap();
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

