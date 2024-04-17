use macroquad::time::{get_frame_time, get_time};

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

    pub fn elapsed(&self, time_frame: f64) -> f64 {
        // INFO : It's dumb but it work
        time_frame - &self.time
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub time: f32,
    pub repeating: bool,
    remaining_time: f32,
    paused: bool,
    completed: bool,
    previously_completed: bool,
}

impl Timer {
    pub fn new(time: f32, repeating: bool) -> Self {
        Self {
            time,
            remaining_time: time,
            repeating,
            paused: false,
            completed: false,
            previously_completed: false,
        }
    }

    pub fn reset(&mut self) {
        self.remaining_time = self.time;
        self.completed = false;
        self.previously_completed = false;
    }

    pub fn update(&mut self) {
        if self.paused {
            return;
        }
        self.previously_completed = self.completed;
        if self.completed && self.repeating == true {
            self.reset();
        }
        if !self.repeating {
            self.completed = false;
        }
        self.remaining_time -= get_frame_time();
        if self.remaining_time <= 0. {
            self.completed = true;
        }
    }

    pub fn just_completed(&self) -> bool {
        !self.previously_completed && self.completed
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn progress(&self) -> f32 {
        (self.remaining_time / self.time).max(0.)
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn stop(&mut self) {
        self.paused = true;
        self.remaining_time = 0.;
        self.completed = true;
    }
}
