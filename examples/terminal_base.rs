use ruscii::terminal::{self, Config, State, Window, Pencil, KeyEvent};
use ruscii::gui::FPSCounter;

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();

    terminal::run(Config::new().fps(u32::MAX), &mut |state: &mut State, window: &mut Window| {
        fps_counter.update();

        Pencil::new(window.surface_mut())
            .draw_text(&format!("FPS: {}", fps_counter.count()), (1, 1));

        for key_event in window.key_events() {
            match key_event {
                KeyEvent::Esc => state.abort = true,
                KeyEvent::Char('q') => state.abort = true,
                KeyEvent::Ctrl('c') => state.abort = true,
                _ => (),
            }
        }
    });
}
