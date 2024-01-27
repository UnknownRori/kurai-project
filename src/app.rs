use crate::{
    components::Position,
    controls::Controls,
    drawable::Drawable,
    entity::{spawn_enemy, spawn_player},
    score::ScoreData,
    system::{update_draw, update_system},
    ui::{draw_entity_number, draw_fps, draw_version, StageUI},
    window::Window,
};

use hecs::World;
use macroquad::prelude::*;

pub struct App {
    window: Window,
    controls: Controls,
    world: World,
    score_data: ScoreData,
}

impl App {
    /// Initialize Game state
    #[must_use]
    pub async fn new(window: Window, controls: Controls) -> Self {
        let mut world = World::new();
        let score_data = ScoreData::default();

        let _ = spawn_player(&mut world).await;
        let _ = spawn_enemy(&mut world, Position::from_array([100.0, 100.0])).await;

        Self {
            window,
            controls,
            world,
            score_data,
        }
    }

    /// This is where the update happen
    pub fn update(&mut self) {
        update_system(&mut self.world, &self.controls, &self.window);
    }

    /// This is where the draw happen
    pub async fn draw(&mut self) {
        clear_background(BLACK);

        update_draw(&self.world, &self.controls, &self.window);
        StageUI::draw(&self.window, &self.score_data).await;

        draw_entity_number(&self.window, self.world.len());
        draw_fps(&self.window, 32.0, WHITE);
        draw_version(&self.window);
    }
}
