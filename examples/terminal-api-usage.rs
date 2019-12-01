use ruscii::terminal;
use ruscii::terminal::{Window, Pencil, Color, Style};
use ruscii::gui::FPSCounter;

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();

    terminal::run(u32::MAX, &mut |window: &mut Window| {
        fps_counter.update();

        let mut pencil = Pencil::new(window.surface_mut());
        pencil.draw_text((0, 0), &format!("FPS: {}", fps_counter.count()));
        pencil.draw_char((0, 3), 'A');
        pencil.set_foreground(Color::Green);
        pencil.draw_char((2, 3), 'B');
        pencil.set_foreground(Color::Red);
        pencil.set_background(Color::Blue);
        pencil.draw_char((4, 3), 'C');
        pencil.set_foreground(Color::White);
        pencil.set_background(Color::Black);
        pencil.draw_char((10, 3), '\u{263A}');
        pencil.draw_text((0, 6), "This is a 'plain' string");
        pencil.set_style(Style::Bold);
        pencil.draw_text((0, 7), "This is a 'bold' string");
        pencil.set_style(Style::Plain);
        pencil.draw_text((0, 8), "This is a 'plain' string again");

        let (width, height) = window.size();

        let mut pencil = Pencil::new(window.surface_mut());
        pencil.set_origin((width / 2, height / 2));
        pencil.draw_text((0, 0), "Centered text");

        true
    });
}
