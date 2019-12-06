use ruscii::terminal::{self, Config, State, Window, Pencil};
use ruscii::input::{self, KeyDown, Key};
use ruscii::gui::FPSCounter;

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();

    terminal::run(Config::new().fps(u32::MAX), &mut |state: &mut State, window: &mut Window| {
        fps_counter.update();

        Pencil::new(window.canvas_mut())
            .draw_text(&format!("FPS: {}", fps_counter.count()), (1, 1));

        for key_down in input::get_keys_down() {
            match key_down {
                KeyDown::Key(Key::Esc) => state.abort = true,
                KeyDown::Key(Key::Q) => state.abort = true,
                KeyDown::Ctrl(Key::C) => state.abort = true,
                _ => (),
            }
        }
    });
}
