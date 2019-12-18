use ruscii::terminal::{self, Config, State, Window, Pencil};
use ruscii::keyboard::Keyboard;

use std::time;

fn main() {
    let keyboard = Keyboard::new();
    let now = time::Instant::now();
    terminal::run(Config::new(), &mut |state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());

        for (i, signal) in keyboard.get_signals().iter().enumerate() {
            pencil.draw_text(&format!("{:?}", signal), (0, i as u16 % 20));
        }

        if now.elapsed().as_secs() > 2 {
            state.abort = true;
        }
    });
}
