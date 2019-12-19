use ruscii::app::{App, Config, State};
use ruscii::terminal::{Window, Pencil, Color};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::gui::{FPSCounter};


struct GameState {
    player_pos: (u16, u16),
    player_move: (i16, i16),
    map_dim: (u16, u16),
}

impl GameState {
    pub fn update(&mut self) {
        let future_pos = (
            (self.player_pos.0 as i16 + self.player_move.0),
            (self.player_pos.1 as i16 + self.player_move.1)
            );
        self.player_move = (0, 0);

        if future_pos.0 < (self.map_dim.0 - 1) as i16
          && future_pos.0 > 0
          && future_pos.1 < (self.map_dim.1 - 1) as i16
          && future_pos.1 > 0 {
            self.player_pos = (future_pos.0 as u16, future_pos.1 as u16);
        }
    }
}


fn main() {
    let mut app = App::config(Config::new().fps(20));
    let (width, height) = app.window().size();
    let mut fps_counter = FPSCounter::new();
    let mut game_state = GameState {
        player_pos: (width / 4, height / 4),
        player_move: (0, 0),
        map_dim: (width / 2, height / 2)
    };

    app.run(|state: &mut State, window: &mut Window| {
        fps_counter.update();

        for key_event in state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => state.stop(),
                KeyEvent::Pressed(Key::Q) => state.stop(),
                _ => (),
            }
        }

        for key_down in state.keyboard().get_keys_down() {
            match key_down {
                Key::H | Key::A => game_state.player_move = (-2, 0),
                Key::J | Key::S => game_state.player_move = (0, 1),
                Key::K | Key::W => game_state.player_move = (0, -1),
                Key::L | Key::D => game_state.player_move = (2, 0),
                _ => (),
            }
        }

        game_state.update();

        Pencil::new(window.canvas_mut())
            .draw_text(&format!("FPS: {}", fps_counter.count()), (0, 0));

        Pencil::new(window.canvas_mut())
            .set_origin((width / 4, height / 4))
            .set_foreground(Color::Grey)
            .draw_char('╔', (0, 0))
            .draw_char('╗', (game_state.map_dim.0 - 1, 0))
            .draw_char('╝', (game_state.map_dim.0 - 1, game_state.map_dim.1 - 1))
            .draw_char('╚', (0, game_state.map_dim.1 - 1))
            .draw_hline('═', (1, 0), game_state.map_dim.0 - 2)
            .draw_hline('═', (1, game_state.map_dim.1 - 1), game_state.map_dim.0 - 2)
            .draw_vline('║', (0, 1), game_state.map_dim.1 - 2)
            .draw_vline('║', (width / 2 - 1, 1), game_state.map_dim.1 - 2);

        Pencil::new(window.canvas_mut())
            .set_origin((width / 4, height / 4))
            .set_foreground(Color::Yellow)
            .draw_char('A', game_state.player_pos);
    });
}
