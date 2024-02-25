use keyframe::{keyframes, AnimationSequence};
use macroquad::{color::Color, shapes::draw_rectangle};

use crate::{
    controls::{Action, Controls},
    time::Instant,
    window::Window,
};

pub struct Pause {
    pub start: Option<Instant>,
    pub opacity: f32,
    pub keyframe: AnimationSequence<f32>,
}

impl Pause {
    const TARGET_OPACITY_OVERLAY: f32 = 0.3;

    pub fn new() -> Self {
        Self {
            start: None,
            opacity: 0.,
            keyframe: keyframes![(0.0, 0.0), (0.3, 1.0)],
        }
    }

    pub const fn is_paused(&self) -> bool {
        self.start.is_some()
    }

    pub fn draw(&mut self, time: f64, delta: f32, screen: &Window) {
        if let Some(pause) = self.start {
            self.keyframe.advance_by(pause.elapsed(time));
            draw_rectangle(
                screen.game_window().get_start().x,
                screen.game_window().get_start().y,
                screen.game_window().size().x,
                screen.game_window().size().y,
                Color::new(0.0, 0.0, 0.0, self.keyframe.now()),
            );
        }
    }

    pub fn update(&mut self, time: f64, controls: &Controls) {
        if controls.is_pressed(&Action::Escape) && self.start.is_none() {
            self.start = Instant::new(time).into();
            self.keyframe.advance_to(0.0);
        } else if controls.is_pressed(&Action::Escape) && self.start.is_some() {
            self.start = None;
        }
    }
}
