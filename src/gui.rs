//! # GUI
//!
//! The `gui` module provides utilities for common GUI elements. Currently, it includes an
//! [`FPSCounter`] that provides the user easy access to the application framerate without them
//! needing to make any time calculations.
//!
//! More examples of use of the [`FPSCounter`] can be found in the
//! [examples](https://github.com/lemunozm/ruscii/tree/master/examples) folder of the
//! [`ruscii`](https://github.com/lemunozm/ruscii) repository.

use std::time::SystemTime;

/// A struct that provides access to the application's framerate.
///
/// This FPS counter does not update automatically. [`FPSCounter::update`] should be called
/// to update the FPS for every frame, i.e., in the `frame_action` function parameter of
/// [`App::run`](super::app::App::run).
///
/// The FPS value can be obtained by calling [`FPSCounter::count`].
///
/// ## Example
///
/// ```rust,no_run
/// # use ruscii::app::{App, State};
/// # use ruscii::terminal::Window;
/// # use ruscii::drawing::{Pencil};
/// # use ruscii::spatial::{Vec2};
/// # use ruscii::gui::{FPSCounter};
///
/// let mut app = App::default();
/// let mut fps_counter = FPSCounter::default();
///
/// app.run(|app_state: &mut State, window: &mut Window| {
///     fps_counter.update();  // Updates the FPS
///
///     let mut pencil = Pencil::new(window.canvas_mut());
///     pencil.draw_text(
///         &format!("FPS: {}", fps_counter.count()),  // Draws the FPS
///         Vec2::xy(1, 0)
///     );
/// });
/// ```
#[derive(Default)]
pub struct FPSCounter {
    fps: u32,
    fps_time_stamp: u128,
    frame_counter: u32,
}

impl FPSCounter {
    /// Retrieves the framerate and updates the [`FPSCounter`].
    pub fn update(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();
        self.frame_counter += 1;
        if current_time - self.fps_time_stamp > 1000000 {
            self.fps = self.frame_counter;
            self.fps_time_stamp = current_time;
            self.frame_counter = 0;
        }
    }

    /// Returns the framerate.
    pub fn count(&self) -> u32 {
        self.fps
    }
}
