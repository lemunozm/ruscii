use ruscii::app::{App, State};
use ruscii::drawing::{AnimationFrame, Animator, Pencil};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

fn main() {
    let mut app = App::default();

    let animation = vec![
        AnimationFrame::new(include_str!("rowboat_frames/1.txt"), Vec2::zero(), 10),
        AnimationFrame::new(include_str!("rowboat_frames/2.txt"), Vec2::zero(), 10),
        AnimationFrame::new(include_str!("rowboat_frames/3.txt"), Vec2::zero(), 10),
        AnimationFrame::new(include_str!("rowboat_frames/2.txt"), Vec2::zero(), 10),
    ];
    let mut animator = Animator::new(animation);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        Pencil::new(window.canvas_mut())
            .set_background(Color::Blue)
            .draw_animator(&mut animator, Vec2::xy(5, 2));
    });
}
