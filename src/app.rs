use super::terminal::Window;
use std::{thread, time};

pub struct Config {
    pub fps: u32,
}

impl Config {
    pub fn new() -> Config {
        Config {fps: 60}
    }

    pub fn fps(mut self, fps: u32) -> Config {
        self.fps = fps;
        self
    }
}


pub struct State {
    pub abort: bool,
    pub dt: time::Duration,
}

impl State {
    pub fn new() -> State {
        State {
            abort: false,
            dt: time::Duration::new(0, 0)
        }
    }
}


pub fn run<F>(config: Config, mut frame_action: F)
where F: FnMut(&mut State, &mut Window) {
    let expected_duration = time::Duration::from_nanos(1_000_000_000 / config.fps as u64);
    let mut window = Window::new();
    let mut state = State::new();
    window.open();
    loop {
        let now = time::Instant::now();
        window.clear();

        frame_action(&mut state, &mut window);
        if state.abort {
            break;
        }

        window.update();

        state.dt = now.elapsed();
        if let Some(time) = expected_duration.checked_sub(state.dt) {
            thread::sleep(time);
        }
    }
    window.close();
}

