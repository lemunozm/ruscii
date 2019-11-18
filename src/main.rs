use ruscii::terminal;
use ruscii::terminal::{Window, Pencil};
use ruscii::gui::FPSCounter;

fn main() {
    let mut fps_counter = FPSCounter::new();
    //let mut i = 0;
    terminal::run(60, &mut |window: &mut Window| {
        fps_counter.update();

        let (width, height) = window.surface().dimension();

        let mut pencil = Pencil::new(window.surface_mut());
        //if i < 30 {
        //    i += 1;
        //}
        //pencil.set_origin((5, i));
        pencil.draw_text((width / 2 , height / 2), &format!("{}", fps_counter));

        true
    });
}
