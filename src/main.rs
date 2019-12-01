use ruscii::terminal;
use ruscii::terminal::{Window, Pencil};
use ruscii::gui::FPSCounter;

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();

    terminal::run(u32::MAX, &mut |window: &mut Window| {
        fps_counter.update();

        let mut pencil = Pencil::new(window.surface_mut());
        pencil.draw_text((1, 1), &format!("FPS: {}", fps_counter.count()));

        true
    });
}
