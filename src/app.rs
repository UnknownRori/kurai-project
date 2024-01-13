use crate::constant::{HEIGHT, WIDTH};

use macroquad::prelude::*;

pub struct App {}

impl App {
    /// Initialize Game state
    pub fn new() -> App {
        Self {}
    }

    /// This is where the update happen
    pub async fn update(&mut self) {}

    /// This is where the draw happen
    pub async fn draw(&mut self) {
        clear_background(BLACK);
        draw_text(
            format!("{}", get_fps()).as_str(),
            (WIDTH - 32) as f32,
            (HEIGHT - 4) as f32,
            32.0,
            WHITE,
        );
    }
}
