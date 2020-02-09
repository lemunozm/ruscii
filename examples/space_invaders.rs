use ruscii::app::{App, State};
use ruscii::terminal::{Window, Color, Style};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};

use rand::{self, prelude::*};

struct GameState {
    pub dimension: Vec2,
    pub spaceship: Vec2,
    pub spaceship_shots: Vec<Vec2>,
    pub last_shot_frame: usize,
    pub aliens: Vec<Vec2>,
    pub aliens_shots: Vec<Vec2>,
    pub aliens_movement: (i32, bool), //dir, just_down
    pub last_aliens_movement: usize,
    pub last_aliens_shots: usize,
    pub lives: usize,
    pub score: usize,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        let mut aliens = Vec::new();
        for y in 2..7 {
            for x in 5..dimension.x - 5 {
                if x % 2 != 0 {
                    aliens.push(Vec2::xy(x, y));
                }
            }
        }
        GameState {
            dimension,
            spaceship: Vec2::xy(dimension.x / 2, dimension.y - 2),
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

    pub fn spaceship_move_x(&mut self, displacement: i32) {
        if displacement < 0 && self.spaceship.x != 0 || displacement > 0 && self.spaceship.x != self.dimension.x {
            self.spaceship.x += displacement;
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
            if shot.y == 1 { return false; }
            let pre_len = aliens.len();
            aliens.retain(|alien| alien != shot);
            let destroyed = aliens.len() != pre_len;
            if destroyed {
                partial_score += 5;
            }
            !destroyed
        });
        self.score += partial_score;

        self.spaceship_shots.iter_mut().for_each(|shot| shot.y -= 1);

        if self.last_aliens_shots + 5 < frame {
            self.last_aliens_shots = frame;
            for alien in &self.aliens {
                let must_shot = rand::thread_rng().gen_range(0, 200) == 0;
                if must_shot {
                    self.aliens_shots.push(*alien);
                }
            }

            let bottom_shot_limit = self.dimension.y;
            self.aliens_shots.retain(|shot| shot.y < bottom_shot_limit);
            self.aliens_shots.iter_mut().for_each(|shot| shot.y += 1);
        }

        let mut damage = 0;
        let spaceship = &self.spaceship;
        self.aliens_shots.retain(|shot| {
            if shot.y == spaceship.y && (shot.x == spaceship.x || shot.x == spaceship.x + 1|| shot.x == spaceship.x - 1) {
                damage += 1;
                return false;
            }
            true
        });

        self.aliens.iter().for_each(|alien| {
            if alien.y == spaceship.y && (alien.x == spaceship.x || alien.x == spaceship.x + 1|| alien.x == spaceship.x - 1) {
                damage = 1000;
            }
        });

        self.lives = if damage >= self.lives { 0 } else { self.lives - damage };


        if self.aliens.len() > 0 {
            let left = self.aliens.iter().min_by_key(|alien| alien.x).unwrap();
            let right = self.aliens.iter().max_by_key(|alien| alien.x).unwrap();
            if self.last_aliens_movement + 20 < frame {
                self.last_aliens_movement = frame;

                if left.x == 0 || right.x == self.dimension.x {
                    if self.aliens_movement.1 {
                        self.aliens_movement.0 = -self.aliens_movement.0;
                        let dir = self.aliens_movement.0;
                        self.aliens.iter_mut().for_each(|alien| alien.x = alien.x + dir);
                        self.aliens_movement.1 = false;
                    }
                    else
                    {
                        self.aliens.iter_mut().for_each(|alien| alien.y += 1);
                        self.aliens_movement.1 = true;
                    }
                }
                else
                {
                    let dir = self.aliens_movement.0;
                    self.aliens.iter_mut().for_each(|alien| alien.x = alien.x + dir);
                }
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    let mut state = GameState::new(Vec2::xy(60, 22));
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
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 0));

        if state.aliens.is_empty() || state.lives == 0 {
            let status_msg = if state.lives > 0 {"You win! :D"} else {"You lose :("};
            let msg = &format!("{}  -  score: {}", status_msg, state.score);
            pencil.set_origin(win_size / 2 - Vec2::x(msg.len() / 2));
            pencil.draw_text(msg, Vec2::zero());
            return ();
        }

        pencil.set_origin((win_size - state.dimension) / 2);
        pencil.draw_text(&format!("lives: {}  -  score: {}", state.lives, state.score), Vec2::xy(15, 0));
        pencil.set_foreground(Color::Cyan);
        pencil.draw_char('^', state.spaceship);
        pencil.draw_char('/', state.spaceship - Vec2::x(1));
        pencil.draw_char('\\', state.spaceship + Vec2::x(1));
        pencil.draw_char('\'', state.spaceship + Vec2::y(1));

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
