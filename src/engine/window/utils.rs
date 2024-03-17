use macroquad::{
    math::{vec2, Vec2},
    window::{screen_height, screen_width},
};

#[inline(always)]
pub fn get_adjusted_screen(aspect_ratio: f32) -> Vec2 {
    let (width, height, actual_aspect_ratio) = {
        (
            screen_width(),
            screen_height(),
            screen_width() / screen_height(),
        )
    };

    let adjusted = if actual_aspect_ratio > aspect_ratio {
        vec2(height * aspect_ratio, height)
    } else {
        vec2(width, width / aspect_ratio)
    };

    adjusted
}
