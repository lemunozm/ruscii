use ruscii::app::{App, Config, State};
use ruscii::terminal::{Window};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::config(Config::new().fps(u32::MAX));

    app.run(|app_state: &mut State, window: &mut Window| {
        fps_counter.update();
        let fps = &format!("FPS: {}", fps_counter.count());

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(fps, Vec2::xy(1, 1));

        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }
    });
}
