use ruscii::app::{self, Config, State};
use ruscii::terminal::{Window, Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::gui::{FPSCounter};

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();

    app::run(Config::new().fps(u32::MAX), &mut |state: &mut State, window: &mut Window| {
        fps_counter.update();
        let fps = &format!("FPS: {}", fps_counter.count());

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(fps, (1, 1));

        for event in state.consume_key_events() {
            match event {
                KeyEvent::Pressed(Key::Esc) => state.abort = true,
                KeyEvent::Pressed(Key::Q) => state.abort = true,
                _ => (),
            }
        }
    });
}
