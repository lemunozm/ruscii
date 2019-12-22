use ruscii::app::{App, State};
use ruscii::terminal::{Window, Pencil, Color, Style};
use ruscii::keyboard::{KeyEvent, Key};

use ruscii::gui::{FPSCounter};

use rand;
use rand::prelude::*;

struct GameState {
    pub dimension: (u16, u16),
    pub spaceship: (u16, u16),
    pub spaceship_shots: Vec<(u16, u16)>,
    pub last_shot_frame: usize,
    pub aliens: Vec<(u16, u16)>,
    pub aliens_shots: Vec<(u16, u16)>,
    pub aliens_movement: (i16, bool), //dir, just_down
    pub last_aliens_movement: usize,
    pub last_aliens_shots: usize,
    pub lives: usize,
    pub score: usize,
}

impl GameState {
    pub fn new(dimension: (u16, u16)) -> GameState {
        let mut aliens = Vec::new();
        for y in 2..7 {
            for x in 5..dimension.0 - 5 {
                if x % 2 != 0 {
                    aliens.push((x, y));
                }
            }
        }
        GameState {
            dimension,
            spaceship: (dimension.0 / 2, dimension.1 - 2),
            spaceship_shots: Vec::new(),
            last_shot_frame: 0,
            aliens,
            aliens_shots: Vec::new(),
            aliens_movement: (1, false),
            last_aliens_movement: 0,
            last_aliens_shots: 0,
            lives: 3,
            score: 0,
        }
    }

    pub fn spaceship_move_x(&mut self, displacement: i16) {
        if displacement < 0 && self.spaceship.0 != 0 || displacement > 0 && self.spaceship.0 != self.dimension.0 {
            self.spaceship.0 = (self.spaceship.0 as i16 + displacement) as u16;
        }
    }

    pub fn spaceship_shot(&mut self, shot_frame: usize) {
        if self.last_shot_frame + 15 < shot_frame {
            self.spaceship_shots.push(self.spaceship);
            self.last_shot_frame = shot_frame;
        }
    }

    pub fn update(&mut self, frame: usize) {
        let mut partial_score = 0;
        let aliens = &mut self.aliens;
        self.spaceship_shots.retain(|shot| {
            if shot.1 == 1 { return false; }
            let pre_len = aliens.len();
            aliens.retain(|alien| alien.0 != shot.0 || alien.1 != shot.1);
            let destroyed = aliens.len() != pre_len;
            if destroyed {
                partial_score += 5;
            }
            !destroyed
        });
        self.score += partial_score;

        self.spaceship_shots.iter_mut().for_each(|x| x.1 -= 1);

        if self.last_aliens_shots + 10 < frame {
            self.last_aliens_shots = frame;
            for alien in &self.aliens {
                let must_shot = rand::thread_rng().gen_range(0, 200) == 0;
                if must_shot {
                    self.aliens_shots.push(*alien);
                }
            }

            let bottom_shot_limit = self.dimension.1;
            self.aliens_shots.retain(|s| s.1 < bottom_shot_limit);
            self.aliens_shots.iter_mut().for_each(|x| x.1 += 1);
        }

        let mut damage = 0;
        let spaceship = &self.spaceship;
        self.aliens_shots.retain(|shot| {
            if shot.1 == spaceship.1 && (shot.0 == spaceship.0 || shot.0 == spaceship.0 + 1|| shot.0 == spaceship.0 - 1) {
                damage += 1;
                return false;
            }
            true
        });

        self.aliens.iter().for_each(|alien| {
            if alien.1 == spaceship.1 && (alien.0 == spaceship.0 || alien.0 == spaceship.0 + 1|| alien.0 == spaceship.0 - 1) {
                damage = 1000;
            }
        });

        self.lives = if damage >= self.lives { 0 } else { self.lives - damage };


        if self.aliens.len() > 0 {
            let left = self.aliens.iter().min_by_key(|e| e.0).unwrap();
            let right = self.aliens.iter().max_by_key(|e| e.0).unwrap();
            if self.last_aliens_movement + 20 < frame {
                self.last_aliens_movement = frame;

                if left.0 == 0 || right.0 == self.dimension.0 {
                    if self.aliens_movement.1 {
                        self.aliens_movement.0 = -self.aliens_movement.0;
                        let dir = self.aliens_movement.0;
                        self.aliens.iter_mut().for_each(|x| x.0 = (x.0 as i16 + dir) as u16);
                        self.aliens_movement.1 = false;
                    }
                    else
                    {
                        self.aliens.iter_mut().for_each(|x| x.1 += 1);
                        self.aliens_movement.1 = true;
                    }
                }
                else
                {
                    let dir = self.aliens_movement.0;
                    self.aliens.iter_mut().for_each(|x| x.0 = (x.0 as i16 + dir) as u16);
                }
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    let mut state = GameState::new((60, 22));
    let mut fps_counter = FPSCounter::new();

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
                Key::A | Key::H => state.spaceship_move_x(-1),
                Key::D | Key::L => state.spaceship_move_x(1),
                Key::Space => state.spaceship_shot(app_state.step()),
                _ => (),
            }
        }

        state.update(app_state.step());
        fps_counter.update();

        let win_size = window.size();
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), (1, 0));

        if state.aliens.is_empty() || state.lives == 0 {
            let status_msg = if state.lives > 0 {"You win! :D"} else {"You lose :("};
            let msg = &format!("{}  -  score: {}", status_msg, state.score);
            pencil.set_origin(((win_size.0 - msg.len() as u16) / 2, win_size.1 / 2));
            pencil.draw_text(msg, (0, 0));
            return ();
        }

        pencil.set_origin(((win_size.0 - state.dimension.0) / 2, (win_size.1 - state.dimension.1) / 2));
        pencil.draw_text(&format!("lives: {}  -  score: {}", state.lives, state.score), (15, 0));
        pencil.set_foreground(Color::Cyan);
        pencil.draw_char('^', state.spaceship);
        pencil.draw_char('/', (state.spaceship.0 - 1, state.spaceship.1));
        pencil.draw_char('\\', (state.spaceship.0 + 1, state.spaceship.1));
        pencil.draw_char('\'', (state.spaceship.0, state.spaceship.1 + 1));

        pencil.set_foreground(Color::Red);
        for shot in &state.aliens_shots {
            pencil.draw_char('|', *shot);
        }

        pencil.set_foreground(Color::Green);
        for alien in &state.aliens {
            pencil.draw_char('W', *alien);
        }

        pencil.set_foreground(Color::Yellow);
        pencil.set_style(Style::Bold);
        for shot in &state.spaceship_shots {
            pencil.draw_char('|', *shot);
        }
    });
}
