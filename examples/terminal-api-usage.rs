use ruscii::terminal;
use ruscii::terminal::{Window, Pencil, Color, Style};
use ruscii::gui::FPSCounter;

use std::u32;

fn main() {
    let mut fps_counter = FPSCounter::new();

    terminal::run(u32::MAX, &mut |window: &mut Window| {
        fps_counter.update();

        Pencil::new(window.surface_mut())
            .draw_text((0, 0), &format!("FPS: {}", fps_counter.count()))
            .draw_char((0, 3), 'A')
            .set_foreground(Color::Green)
            .draw_char((2, 3), 'B')
            .set_foreground(Color::Red)
            .set_background(Color::Blue)
            .draw_char((4, 3), 'C')
            .set_foreground(Color::White)
            .set_background(Color::Black)
            .draw_char((10, 3), '\u{263A}')
            .draw_text((0, 6), "This is a 'plain' string")
            .set_style(Style::Bold)
            .draw_text((0, 7), "This is a 'bold' string")
            .set_style(Style::Plain)
            .draw_text((0, 8), "This is a 'plain' string again");

        let (width, height) = window.size();

        Pencil::new(window.surface_mut())
            .set_origin((width / 2, height / 2))
            .draw_text((0, 0), "Centered text");

        true
    });
}
