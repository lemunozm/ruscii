use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

fn main() {
    let mut key_events = Vec::new();
    let mut app = App::default();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            key_events.push(*key_event);
            if let KeyEvent::Pressed(Key::Q) = key_event {
                app_state.stop();
            }
        }

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil
            .draw_text("Press Q for exit", Vec2::xy(0, 0))
            .set_origin(Vec2::xy(0, 3));

        for (i, key_event) in key_events.iter().rev().enumerate() {
            pencil.draw_text(&format!("{:?}", key_event), Vec2::y(i));
        }
    });
}
