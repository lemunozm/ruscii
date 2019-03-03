use ruscii::terminal::{self, Config, State, Window, Pencil, Color, KeyEvent};
use ruscii::gui::FPSCounter;

use std::time::Duration;

#[derive(Clone, Copy)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn to_coord(&self) -> (i16, i16) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (2, 0),
            Direction::Left => (-2, 0),
        }
    }
}

struct GameState {
    player_pos: (u16, u16),
    player_eye: (u16, u16),
    player_last_move: Option<Direction>,
    player_just_shot: bool,
    map_dim: (u16, u16),
    shots: Vec<((u16, u16), (i16, i16))>,
}

impl GameState {
    pub fn update(&mut self, dt: Duration) {
        if let Some(direction) = self.player_last_move {
            let movement = direction.to_coord();
            let future_pos = (self.player_pos.0 as i16 + movement.0, self.player_pos.1 as i16 + movement.1);
            if future_pos.0 < (self.map_dim.0 - 1) as i16
              && future_pos.0 > 0
              && future_pos.1 < (self.map_dim.1 - 1) as i16
              && future_pos.1 > 0 {
                self.player_pos = (future_pos.0 as u16, future_pos.1 as u16);
                self.player_eye = ((future_pos.0 + movement.0) as u16, (future_pos.1 + movement.1) as u16)
            }
            self.player_last_move = None
        }

        if self.player_just_shot {
            self.shots.push(self.player_eye);
            self.player_just_shot = false;
        }

        for shot in self.shots {
            shot.0 = (shot.0.0 +shot)
        }
        //TODO shoot collision
    }
}


fn main() {
    let (width, height) = terminal::size();
    let mut fps_counter = FPSCounter::new();
    let mut game_state = GameState {
        player_pos: (width / 4, height / 4),
        player_eye: (width / 4 + 1, height / 4),
        player_last_move: None,
        player_just_shot: false,
        map_dim: (width / 2, height / 2),
        shots: Vec::new(),
    };

    terminal::run(Config::new(), &mut |term_state: &mut State, window: &mut Window| {
        fps_counter.update();

        for key_event in window.key_events() {
            match key_event {
                KeyEvent::Esc => term_state.abort = true,
                KeyEvent::Char('q') => term_state.abort = true,
                KeyEvent::Ctrl('c') => term_state.abort = true,

                KeyEvent::Char('h') => game_state.player_last_move = Some(Direction::Left),
                KeyEvent::Char('j') => game_state.player_last_move = Some(Direction::Down),
                KeyEvent::Char('k') => game_state.player_last_move = Some(Direction::Up),
                KeyEvent::Char('l') => game_state.player_last_move = Some(Direction::Right),
                KeyEvent::Char('f') => game_state.player_just_shot = true,
                _ => (),
            }
        }

        game_state.update(term_state.dt);

        Pencil::new(window.surface_mut())
            .set_origin((width / 4, height / 4))
            .set_foreground(Color::LightGrey)
            .draw_char('╔', (0, 0))
            .draw_char('╗', (game_state.map_dim.0 - 1, 0))
            .draw_char('╝', (game_state.map_dim.0 - 1, game_state.map_dim.1 - 1))
            .draw_char('╚', (0, game_state.map_dim.1 - 1))
            .draw_hline('═', (1, 0), game_state.map_dim.0 - 2)
            .draw_hline('═', (1, game_state.map_dim.1 - 1), game_state.map_dim.0 - 2)
            .draw_vline('║', (0, 1), game_state.map_dim.1 - 2)
            .draw_vline('║', (width / 2 - 1, 1), game_state.map_dim.1 - 2);

        Pencil::new(window.surface_mut())
            .set_origin((width / 4, height / 4))
            .set_foreground(Color::Grey)
            .draw_char('·', game_state.player_eye);
            .set_foreground(Color::Yellow)
            .draw_char('A', game_state.player_pos)

        let mut shot_pencil = Pencil::new(window.surface_mut());
        shot_pencil.set_origin((width / 4, height / 4));
        shot_pencil.set_foreground(Color::Xterm(208));

        for shot in &game_state.shots {
            shot_pencil.draw_char('o', *shot.0);
        }

        Pencil::new(window.surface_mut())
            .draw_text(&format!("FPS: {}", fps_counter.count()), (0, 0));
    });
}
