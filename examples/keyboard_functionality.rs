use ruscii::app::{App, State};
use ruscii::terminal::{Window, Pencil};
use ruscii::keyboard::{KeyEvent, Key};

fn main() {
    let mut key_events = Vec::new();
    let mut app = App::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text("Press Q for exit", (0, 0)).set_origin((0, 3));

        for key_event in app_state.keyboard().last_key_events() {
            key_events.push(*key_event);
            if let KeyEvent::Pressed(Key::Q) = key_event {
                app_state.stop();
            }
        }

        for (i, key_event) in key_events.iter().rev().enumerate() {
            pencil.draw_text(&format!("{:?}", key_event), (0, i as u16));
        }
    });
}
