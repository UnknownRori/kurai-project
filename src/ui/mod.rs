use macroquad::prelude::*;

use crate::drawable::Drawable;
use crate::window::Window;

pub struct StageUI {}

fn draw_scoreboard(window: &Window) {
    let width_score = window.get_width() / 2.5;
    draw_rectangle(
        window.get_width() - width_score,
        0.0,
        width_score,
        *window.get_height(),
        RED,
    );
}

pub fn draw_entity_number(window: &Window, number: u32) {
    draw_text(
        format!("{number}").as_str(),
        window.get_width() - 14.0,
        window.get_height() - 32.0,
        12.0,
        WHITE,
    );
}

pub fn draw_fps(window: &Window, font_size: f32, color: Color) {
    draw_text(
        format!("{}", get_fps()).as_str(),
        window.get_width() - 32.0,
        window.get_height() - 4.0,
        font_size,
        color,
    );
}

impl Drawable for StageUI {
    async fn draw(window: &Window) {
        draw_scoreboard(window);
    }
}
