use crate::{
    controls::Controls,
    drawable::Drawable,
    ui::{draw_fps, StageUI},
    window::Window,
};

use macroquad::prelude::*;

pub struct App {
    window: Window,
    controls: Controls,
}

impl App {
    /// Initialize Game state
    pub fn new(window: Window, controls: Controls) -> App {
        Self { window, controls }
    }

    /// This is where the update happen
    pub async fn update(&mut self) {}

    /// This is where the draw happen
    pub async fn draw(&mut self) {
        clear_background(BLACK);

        StageUI::draw(&self.window).await;

        draw_fps(&self.window, 32.0, WHITE);
    }
}
