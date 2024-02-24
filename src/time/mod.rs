use macroquad::time::get_time;

#[derive(Debug, Clone, Copy)]
pub struct Instant {
    time: f64,
}

impl Instant {
    pub fn new(time: f64) -> Self {
        Self { time }
    }

    /// Cannot be used between thread
    pub fn now() -> Self {
        Self { time: get_time() }
    }

    pub fn pause_offset(&self, pause_start_time: &Self) -> Self {
        Self {
            time: self.time - pause_start_time.time,
        }
    }

    pub fn elapsed(&self, time_frame: f64) -> f64 {
        // INFO : It's dumb but it work
        time_frame - &self.time
    }
}
