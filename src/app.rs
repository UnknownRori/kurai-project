use crate::{
    assets::{AssetsHandler, AssetsManager},
    components::{Movement, MovementQueue, Position},
    controls::Controls,
    entity::{spawn_enemy, spawn_player},
    score::ScoreData,
    system::{update_draw, update_system},
    ui::{draw_entity_number, draw_fps, draw_version, StageUI},
    window::Window,
};

use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;

pub struct App {
    window: Window,
    controls: Controls,
    world: World,
    score_data: ScoreData,
    assets_manager: AssetsManager,
    debugger: crate::engine::debug::Debugger,
}

impl App {
    /// Initialize Game state
    #[must_use]
    pub async fn new(window: Window, controls: Controls) -> Self {
        let mut world = World::new();
        let mut assets_manager = AssetsManager::default();
        let score_data = ScoreData::default();
        assets_manager
            .textures
            .batch(&[
                ("remilia0", "./resources/textures/remilia-scarlet/1.png"),
                ("fairy0", "./resources/textures/fairy/fairy0001.png"),
                ("hud", "./resources/ui/hud.png"),
                ("stage1", "./resources/background/stage-1.png"),
                ("mask", "./resources/ui/playable-mask.png"),
            ])
            .await
            .unwrap();

        let _ = spawn_player(
            &mut world,
            assets_manager
                .textures
                .get("remilia0")
                .expect("There is no Remilia Texture"),
        );

        let pos = vec![
            Movement::new(Complex::new(0.1, 0.5), 2.0, false),
            Movement::new(Complex::new(0.5, 0.0), 0.0, true),
        ];
        let movement = MovementQueue::new(pos);
        let _ = spawn_enemy(
            &mut world,
            Position::from_array([1.0, 0.1]),
            assets_manager
                .textures
                .get("fairy0")
                .expect("There is no Fairy Texture"),
            movement,
        );

        // TODO : Put this into Engine part
        let debugger = crate::engine::debug::Debugger::new();

        Self {
            window,
            controls,
            world,
            score_data,
            assets_manager,
            debugger,
        }
    }

    /// This is where the update happen
    pub fn update(&mut self) {
        self.window.update();
        update_system(
            &mut self.world,
            &self.controls,
            &self.window,
            get_frame_time(),
            get_time(),
        );
    }

    /// This is where the draw happen
    pub async fn draw(&mut self) {
        clear_background(BLACK);

        // TODO : This stupid things should live in Stage Struct
        let offset = vec2(0.001, 0.001) * self.window.playable_window().size().clone()
            + self.window.playable_window().get_start().clone();
        let texture = self.assets_manager.textures.get("stage1").unwrap();
        let mask = self.assets_manager.textures.get("mask").unwrap();
        draw_texture_ex(
            &mask,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.window.playable_window().size().clone()),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &texture,
            offset.x,
            offset.y,
            // WHITE,
            Color::new(1f32, 1f32, 1f32, 0.5),
            DrawTextureParams {
                dest_size: Some(self.window.playable_window().size().clone()),
                ..Default::default()
            },
        );
        update_draw(&self.world, &self.controls, &self.window);
        StageUI::draw(&self.window, &self.score_data, &self.assets_manager).await;

        draw_entity_number(&self.window, self.world.len());
        draw_fps(&self.window, 32.0, WHITE);
        draw_version(&self.window);
        self.debugger.update(&self.window);
        self.debugger.draw(&self.window);
        // draw_rectangle(
        //     self.window.playable_window().get_start().x,
        //     self.window.playable_window().get_start().y,
        //     self.window.playable_window().size().x,
        //     self.window.playable_window().size().y,
        //     Color::new(255f32, 0f32, 0f32, 0.5),
        // );
    }
}
