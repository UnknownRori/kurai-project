use macroquad::prelude::*;

use crate::{controls::Action, engine::controls::Controls};

#[derive(Debug, Default)]
pub struct Pause {
    currently_paused: bool,
}

impl Pause {
    pub fn is_paused(&self) -> bool {
        !self.currently_paused
    }

    pub fn update(&mut self, controls: &Controls<Action>) {
        if controls.is_pressed(Action::Escape) {
            self.currently_paused = !self.currently_paused;
        }
    }

    pub fn draw(&self) {
        if !self.currently_paused {
            return;
        }

        draw_rectangle(0., 0., 1., 1., Color::new(0., 0., 0., 0.75));
    }
}
