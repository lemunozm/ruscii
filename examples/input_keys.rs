use ruscii::terminal::{self, Config, State, Window, Pencil};
use ruscii::input::{self, KeyDown, Key};

fn main() {
    terminal::run(Config::new(), &mut |state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text("Press any key... ", (0, 0));

        for (i, key_down) in input::get_keys_down().iter().enumerate() {
            pencil.draw_text(&format!("Key down: {:?}", key_down), (0, 2 + i as u16));
        }

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
