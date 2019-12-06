use ruscii::terminal::{self, Config, State, Window, Pencil, Color};
use ruscii::input::{self, KeyDown, Key};
use ruscii::gui::FPSCounter;


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
        if future_pos.0 < (self.map_dim.0 - 1) as i16
          && future_pos.0 > 0
          && future_pos.1 < (self.map_dim.1 - 1) as i16
          && future_pos.1 > 0 {
            self.player_pos = (future_pos.0 as u16, future_pos.1 as u16);
        }
        self.player_move = (0, 0)
    }
}


fn main() {
    let (width, height) = terminal::size();
    let mut fps_counter = FPSCounter::new();
    let mut game_state = GameState{
        player_pos: (width / 4, height / 4),
        player_move: (0, 0),
        map_dim: (width / 2, height / 2)
    };

    terminal::run(Config::new().fps(30), &mut |term_state: &mut State, window: &mut Window| {
        fps_counter.update();

        for key_down in input::get_keys_down() {
            match key_down {
                KeyDown::Key(Key::Esc) => term_state.abort = true,
                KeyDown::Key(Key::Q) => term_state.abort = true,
                KeyDown::Ctrl(Key::C) => term_state.abort = true,

                KeyDown::Key(Key::H) | KeyDown::Key(Key::A) => game_state.player_move = (-2, 0),
                KeyDown::Key(Key::J) | KeyDown::Key(Key::S) => game_state.player_move = (0, 1),
                KeyDown::Key(Key::K) | KeyDown::Key(Key::W) => game_state.player_move = (0, -1),
                KeyDown::Key(Key::L) | KeyDown::Key(Key::D) => game_state.player_move = (2, 0),
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