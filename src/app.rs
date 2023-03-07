//! # App
//!
//! The `app` module provides functionality related to application itself, including its
//! framerate, the keyboard, and its execution.

use super::keyboard::Keyboard;
use super::terminal::Window;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{panic, thread, time};

use std::io::{self, BufRead};

/// Contains the [`App`] configuration settings, currently including only the framerate.
pub struct Config {
    pub fps: u32,
}

impl Config {
    pub fn new() -> Config {
        Config { fps: 30 }
    }

    /// Consumes the receiver [`Config`] and returns a new one with the maximum framerate to the
    /// given `fps`.
    pub fn fps(mut self, fps: u32) -> Config {
        self.fps = fps;
        self
    }
}

impl Default for Config {
    /// Constructs a [`Config`] with a default maximum framerate of 30.
    fn default() -> Self {
        Self { fps: 30 }
    }
}

/// Contains the run state of the the [`App`] and the [`Keyboard`] through [`State::keyboard`] for
/// the key event interface.
#[derive(Default)]
pub struct State {
    running: Arc<AtomicBool>,
    keyboard: Keyboard,
    pub(self) dt: time::Duration,
    pub(self) step: usize,
}

impl State {
    pub fn run(&self) {
        self.running.store(true, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Returns the [`Keyboard`]. For more information on how to access keyboard input, see
    /// documentation for the [keyboard](crate::keyboard) module.
    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn dt(&self) -> &time::Duration {
        &self.dt
    }

    pub fn step(&self) -> usize {
        self.step
    }
}

/// The essential parts of the application, containing its [`Config`], [`State`], and [`Window`].
///
/// [`App`] objects are created with a default maximum framerate of 30 using [`App::default`].
/// To change this, pass a [`Config`] object with the desired framerate using [`App::config`].
#[derive(Default)]
pub struct App {
    config: Config,
    state: State,
    window: Window,
}

impl App {
    /// Constructs an [`App`] with the given [`Config`].
    pub fn config(config: Config) -> App {
        App {
            config,
            state: State::default(),
            window: Window::default(),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Begins running the terminal application.
    ///
    /// This function begins a loop where key events are first registered, the window is cleared,
    /// `frame_action` is called adding characters to the `Canvas`, and the window is redrawn.
    ///
    /// If the time it takes to execute all of these is less than the [`App`] expects according to
    /// the framerate set in the [`Config`], the current thread is put to sleep until the next
    /// frame, thereby limiting FPS.
    ///
    /// Catches all unwinding panics that occur within `frame_action`, allowing terminal recovery.
    pub fn run<F>(&mut self, mut frame_action: F)
    where
        F: FnMut(&mut State, &mut Window),
    {
        let expected_duration = time::Duration::from_nanos(1_000_000_000 / self.config.fps as u64);
        self.state.run();

        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            self.window.open();
            while self.state.is_running() {
                let now = time::Instant::now();
                self.window.clear();

                self.state.keyboard.consume_key_events();
                frame_action(&mut self.state, &mut self.window);

                self.window.draw();

                self.state.dt = now.elapsed();
                self.state.step += 1;
                if let Some(time) = expected_duration.checked_sub(self.state.dt) {
                    thread::sleep(time);
                }
            }
            self.window.close();
        }));

        if result.is_err() {
            println!("\n\n[Press 'enter' to recover the terminal]");
            io::stdin().lock().lines().next().unwrap().unwrap();
            self.window.close();
        }
    }
}
