use ruscii::app::{App, Config, State};
use ruscii::terminal::{Window, Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::gui::{FPSCounter};

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::config(Config::new().fps(u32::MAX));

    app.run(|state: &mut State, window: &mut Window| {
        fps_counter.update();
        let fps = &format!("FPS: {}", fps_counter.count());

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(fps, (1, 1));

        for key_event in state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => state.stop(),
                KeyEvent::Pressed(Key::Q) => state.stop(),
                _ => (),
            }
        }
    });
}
