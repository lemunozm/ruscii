use ruscii::app::{App, Config, State};
use ruscii::terminal::{Window, Color};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};

struct GameState {
    player_pos: Vec2,
    player_move: Vec2,
    map_dim: Vec2,
}

impl GameState {
    pub fn update(&mut self) {
        let future_pos = self.player_pos + self.player_move;
        self.player_move.clear();

        if future_pos.x < (self.map_dim.x - 1) && future_pos.x > 0
          && future_pos.y < (self.map_dim.y - 1) && future_pos.y > 0 {
            self.player_pos = future_pos;
        }
    }
}

fn main() {
    let mut app = App::config(Config::new().fps(20));
    let size = app.window().size();
    let mut fps_counter = FPSCounter::new();
    let mut state = GameState {
        player_pos: size / 4,
        player_move: Vec2::xy(1, 0),
        map_dim: size / 2,
    };

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::H | Key::A => state.player_move = Vec2::x(-2),
                Key::J | Key::S => state.player_move = Vec2::y(1),
                Key::K | Key::W => state.player_move = Vec2::y(-1),
                Key::L | Key::D => state.player_move = Vec2::x(2),
                _ => (),
            }
        }

        fps_counter.update();
        state.update();

        Pencil::new(window.canvas_mut())
            .draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 0))
            .draw_text("Press 'Q' or 'Esc' for exit", Vec2::y(2))
            .set_origin(size / 4)
            .set_foreground(Color::Grey)
            .draw_rect(&RectCharset::double_lines(), Vec2::zero(), state.map_dim)
            .set_foreground(Color::Yellow)
            .draw_char('A', state.player_pos);
    });
}
