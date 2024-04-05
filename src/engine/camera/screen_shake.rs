use macroquad::prelude::*;

use crate::engine::{rand::rand_dir, time::Timer};

pub struct ScreenShake {
    pub distance: f32,
    pub camera_offset: Vec2,
    pub timer: Timer,
}

impl ScreenShake {
    pub fn new() -> Self {
        Self {
            distance: 0.1,
            camera_offset: Vec2::ZERO,
            timer: Timer::new(0., false),
        }
    }

    pub fn shake(&mut self, duration: f32, distance: f32) {
        self.distance = distance;
        self.timer.time = duration;
        self.timer.reset();
    }

    pub fn update(&mut self) {
        self.timer.update();
    }

    pub fn get_shake_offset(&self) -> Vec2 {
        if self.timer.completed() {
            return Vec2::ZERO;
        }

        return rand_dir() * self.timer.progress() * self.distance;
    }
}
