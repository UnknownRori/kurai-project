use crate::{
    components::{Controllable, Movable, Player},
    controls::{self, Controls},
    drawable::Drawable,
    entity::PlayerEntity,
    system::update_player_move,
    ui::{draw_fps, StageUI},
    window::Window,
};

use hecs::World;
use macroquad::prelude::*;

pub struct App {
    window: Window,
    controls: Controls,
    world: World,
}

impl App {
    /// Initialize Game state
    pub fn new(window: Window, controls: Controls) -> App {
        let mut world = World::new();

        world.spawn((Player, Controllable, Movable));

        Self {
            window,
            controls,
            world,
        }
    }

    /// This is where the update happen
    pub async fn update(&mut self) {}

    /// This is where the draw happen
    pub async fn draw(&mut self) {
        clear_background(BLACK);

        update_player_move(&mut self.world, &self.controls);
        StageUI::draw(&self.window).await;

        draw_fps(&self.window, 32.0, WHITE);
    }
}
