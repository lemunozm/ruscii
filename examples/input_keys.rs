use ruscii::terminal::{self, Config, State, Window, Pencil};
use ruscii::input::{self, KeyDown, Key, KeyReader, KeyEvent};

fn main() {
    let mut char_input = Vec::new();
    let mut input_reader = KeyReader::new();
    input_reader.start();

    terminal::run(Config::new(), &mut |state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text("Press any key... ", (0, 0));
        pencil.draw_text("Press ESC or Ctrl-C to exit ... ", (0, 1));

        for key_down in input::get_keys_down() {
            match key_down {
                KeyDown::Key(Key::Esc) => state.abort = true,
                KeyDown::Ctrl(Key::C) => state.abort = true,
                _ => (),
            }
        }

        //Keys down check
        for (i, key_down) in input::get_keys_down().iter().enumerate() {
            pencil.draw_text(&format!("Key down: {:?}", key_down), (0, 3 + i as u16));
        }

        //Key events
        for key_event in input_reader.key_events() {
           match key_event {
                KeyEvent::Char(c) => char_input.push(c),
                _ => (),
           }
        }
        pencil.draw_text(&format!("stdin: {:?}", char_input), (0, 15));
    });

    input_reader.stop();
}
