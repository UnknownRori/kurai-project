use crate::{
    controls::{Action, Controls},
    time::Instant,
    window::Window,
};

pub struct Pause {
    pub start: Option<Instant>,
    pub opacity: f32,
}

impl Pause {
    pub const fn new() -> Self {
        Self {
            start: None,
            opacity: 0.,
        }
    }

    pub const fn is_paused(&self) -> bool {
        self.start.is_some()
    }

    pub fn draw(&self, screen: &Window) {
        todo!()
    }

    pub fn update(&mut self, time: f64, controls: &Controls) {
        if controls.is_pressed(&Action::Escape) && self.start.is_none() {
            self.start = Instant::new(time).into();
        } else if controls.is_pressed(&Action::Escape) && self.start.is_some() {
            self.start = None;
        }
    }
}
