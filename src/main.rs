use ruscii::terminal::{self, Config, State, Window, Pencil};
use ruscii::gui::FPSCounter;

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();

    terminal::run(Config::new().fps(u32::MAX), &mut |_state: &mut State, window: &mut Window| {
        fps_counter.update();

        Pencil::new(window.surface_mut())
            .draw_text((1, 1), &format!("FPS: {}", fps_counter.count()));
    });
}
