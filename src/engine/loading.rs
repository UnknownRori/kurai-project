use macroquad::prelude::*;

use crate::{math::NormalizationVector2, window::PlayableWindow};

pub struct Loading {
    pub max: usize,
    pub current: usize,
    pub size: Vec2,
}

impl Loading {
    pub fn new(max: usize, size: Vec2) -> Self {
        Self {
            current: 0,
            max,
            size,
        }
    }

    pub async fn draw(&mut self, game_window: &PlayableWindow) {
        let wait = "Please wait warmly";
        let offset = measure_text(&wait, None, 24, 1.0);
        let center = vec2(0.5, 0.5).reset_from_vec2(*game_window.size()) + *game_window.get_start();
        clear_background(BLACK);
        draw_text(
            wait,
            center.x - offset.width / 2.,
            center.y - offset.height / 2.,
            24.,
            WHITE,
        );

        let size = self.size.reset_from_vec2(*game_window.size());
        let center = vec2(0.5, 0.55).reset_from_vec2(*game_window.size())
            + *game_window.get_start()
            - size / 2.;
        draw_rectangle_lines(center.x, center.y, size.x, size.y, 2., WHITE);
        draw_rectangle(
            center.x,
            center.y,
            size.x * (self.current as f32 / self.max as f32),
            size.y,
            WHITE,
        );

        self.current += 1;
    }
}
