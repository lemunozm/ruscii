use std::time::SystemTime;
use std::fmt;

pub struct FPSCounter {
    fps: u32,
    fps_time_stamp: u128,
    frame_counter: u32,
}

impl FPSCounter {
    pub fn new() -> FPSCounter {
        FPSCounter {
            fps: 0,
            fps_time_stamp: 0,
            frame_counter: 0
        }
    }

    pub fn update(&mut self) {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();
        self.frame_counter += 1;
        if current_time - self.fps_time_stamp > 1000000 {
            self.fps = self.frame_counter;
            self.fps_time_stamp = current_time;
            self.frame_counter = 0;
        }
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }
}

impl fmt::Display for FPSCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FPS: {}", self.fps)
    }
}
