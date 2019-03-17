use ruscii::keyboard::Keyboard;

use std::time;
use std::thread;

fn main() {
    let _keyboard = Keyboard::new();

    thread::sleep(time::Duration::from_secs(10));
}
