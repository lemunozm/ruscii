extern crate ruscii;

use ruscii::terminal::{Window, Pencil};
use std::{thread, time};

fn main() {
    let mut window = Window::open();

    for i in 0..3 {
        window.clear();

        let mut pencil = Pencil::new(window.surface_mut());
        pencil.move_to((5, 5));
        pencil.draw('A');

        window.update();

        println!("Update window {}", i + 1);
        thread::sleep(time::Duration::from_secs(1));
    }

    window.close();
}
