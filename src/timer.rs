use sdl2::timer;

pub struct Timer {
    last: u64,
}

impl Timer {
    /// Creates a new timer with the time set to the current time
    pub fn new() -> Timer {
        Timer {
            last: timer::get_performance_counter(),
        }
    }

    /// Returns the time elapsed since the timer was started
    pub fn elapsed(&self) -> u64 {
        timer::get_performance_counter() - self.last
    }

    /// Returns the time elapsed since the timer was started in seconds
    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed() as f32 / (timer::get_performance_frequency() as f32)
    }

    /// Resets the timer
    pub fn reset(&mut self) {
        self.last = timer::get_performance_counter();
    }
}
