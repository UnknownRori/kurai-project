use crate::{
    assets::AssetsManager,
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
    assets_manager: AssetsManager,
}

impl App {
    /// Initialize Game state
    #[must_use]
    pub async fn new(window: Window, controls: Controls) -> Self {
        let mut world = World::new();
        let mut assets_manager = AssetsManager::default();
        let score_data = ScoreData::default();
        assets_manager
            .register_texture_batch(&[
                ("remilia0", "./resources/textures/remilia-scarlet/1.png"),
                ("fairy0", "./resources/textures/fairy/fairy0001.png"),
                ("hud", "./resources/ui/hud.png"),
            ])
            .await
            .unwrap();

        let _ = spawn_player(
            &mut world,
            assets_manager
                .get_texture("remilia0")
                .expect("There is no Remilia Texture"),
        );
        let _ = spawn_enemy(
            &mut world,
            Position::from_array([0.2, 0.1]),
            assets_manager
                .get_texture("fairy0")
                .expect("There is no Fairy Texture"),
        );

        Self {
            window,
            controls,
            world,
            score_data,
            assets_manager,
        }
    }

    /// This is where the update happen
    pub fn update(&mut self) {
        self.window.update();
        update_system(&mut self.world, &self.controls, &self.window);
    }

    /// This is where the draw happen
    pub async fn draw(&mut self) {
        clear_background(BLACK);

        update_draw(&self.world, &self.controls, &self.window);
        StageUI::draw(&self.window, &self.score_data, &self.assets_manager).await;

        draw_entity_number(&self.window, self.world.len());
        draw_fps(&self.window, 32.0, WHITE);
        draw_version(&self.window);

        draw_rectangle(
            self.window.playable_window().get_start().x,
            self.window.playable_window().get_start().y,
            self.window.playable_window().size().x,
            self.window.playable_window().size().y,
            Color::new(255f32, 0f32, 0f32, 0.5),
        );
    }
}
