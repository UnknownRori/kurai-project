use num_complex::Complex;

use crate::engine::{math::ComplexExt, time::Instant};

#[derive(Debug, Clone, Copy)]
pub struct Movement {
    pub target: Complex<f32>,
    pub wait: f64,
    pub start: Option<Instant>,
    pub done: Option<Instant>,
    pub smooth: bool,
}

impl Movement {
    pub fn new(target: Complex<f32>, wait: f64, smooth: bool) -> Self {
        Self {
            target,
            start: None,
            done: None,
            wait,
            smooth,
        }
    }

    pub fn dir(&self, current_pos: &Complex<f32>, move_speed: f32, delta: f32) -> Complex<f32> {
        (self.target - current_pos).normalize() * move_speed * delta
    }
}
