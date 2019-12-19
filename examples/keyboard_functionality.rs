use ruscii::app::{self, Config, State};
use ruscii::terminal::{Window, Pencil};
use ruscii::keyboard::{Keyboard, KeyEvent, Key};

fn main() {
    let mut keyboard = Keyboard::new();
    let mut events = Vec::new();

    app::run(Config::new(), &mut |state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text("Press Q for exit", (0, 0)).set_origin((0, 3));

        for event in keyboard.consume_key_events() {
            events.push(event);
            if let KeyEvent::Pressed(Key::Q) = event {
                state.abort = true;
            }
        }

        for (i, event) in events.iter().rev().enumerate() {
            pencil.draw_text(&format!("{:?}", event), (0, i as u16));
        }
    });
}
