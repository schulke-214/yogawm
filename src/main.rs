use libx11::prelude::*;
use libx11::connect;
use libx11::screens::get_screen;
use libx11::windows::get_all_windows;

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

    let mut xterm_is_hidden = false;
    let mut counter = 0;

    loop {
        let windows = get_all_windows(&connection, &screen);
        println!("{:#?}", windows);

        for win in windows.unwrap().iter() {

            if win.get_wm_class().unwrap() == "XTerm" {
                println!("xterm {:#?}", win.get_wm_class());

                if !xterm_is_hidden {
                    println!("hide");
                    win.unmap(&connection)?;
                    xterm_is_hidden = true;
                } else {
                    println!("show");
                    win.map(&connection)?;
                    xterm_is_hidden = false;
                }

                if counter >= 5 {
                    win.destroy(&connection)?;
                }

                counter += 1;
                connection.flush().unwrap();
            }

        }

        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}

