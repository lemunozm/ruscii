use ruscii::terminal;
use ruscii::terminal::{Window, Pencil};
use ruscii::gui::FPSCounter;

fn main() {
    let mut fps_counter = FPSCounter::new();
    terminal::run(60, &mut |window: &mut Window| {
        fps_counter.update();

        //let (width, height) = window.surface().dimension();

        let mut pencil = Pencil::new(window.surface_mut());
        pencil.draw_text((0, 0), &format!("FPS: {}", fps_counter.count()));
        //pencil.draw_text((width / 2 , height / 2), &format!("FPS: {}", fps_counter.count()));

        true
    });
}
