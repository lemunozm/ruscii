use ruscii::terminal::{self, Config, State, Window, Pencil};
use ruscii::keyboard::Keyboard;

use std::time;

fn main() {
    let now = time::Instant::now();
    let mut keyboard = Keyboard::new();
    let mut events = Vec::new();
    terminal::run(Config::new(), &mut |state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());

        for event in keyboard.consume_key_events() {
            events.push(event);
        }

        for (i, event) in events.iter().rev().enumerate() {
            pencil.draw_text(&format!("{:?}", event), (0, i as u16));
        }

        if now.elapsed().as_secs() > 5 {
            state.abort = true;
        }
    });
}
