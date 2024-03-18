use hecs::World;
use macroquad::prelude::*;

use crate::{
    assets::preload::preload,
    controls::{init_controls, Action},
    engine::{
        assets::AssetsManager,
        camera::screen_buffer2d::{ScreenBuffer2D, ScreenBuffer2DBuilder},
        controls::Controls,
        window::utils::get_adjusted_screen,
    },
    konst::{
        DESIRED_ASPECT_RATIO, VIRTUAL_SCREEN_WIDTH, VIRTUAL_STAGE_HEIGHT, VIRTUAL_STAGE_WIDTH,
    },
    scene::{stage::StageManager, stage1::Stage1Lazy},
    score::ScoreData,
    system::{update_draw, update_draw_hud, update_system},
    ui::game_hud::{draw_entity_number, init_game_hud, init_hud_info},
};

pub struct App {
    assets_manager: AssetsManager,
    game_buffer: ScreenBuffer2D,
    playable_buffer: ScreenBuffer2D,
    controls: Controls<Action>,
    world: World,
    stages_manager: StageManager,
    score: ScoreData,
}

impl App {
    pub async fn new() -> Self {
        let game_buffer: ScreenBuffer2D =
            ScreenBuffer2DBuilder::from_aspect_ratio(VIRTUAL_SCREEN_WIDTH, DESIRED_ASPECT_RATIO)
                .filter(FilterMode::Nearest)
                .into();

        let playable_buffer: ScreenBuffer2D =
            ScreenBuffer2DBuilder::from_size(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT)
                .filter(FilterMode::Nearest)
                .into();

        let mut assets_manager = AssetsManager::default();
        preload(&mut assets_manager).await;
        let mut world = World::new();

        let mut stages_manager = StageManager::new(vec![Box::new(Stage1Lazy)]);

        world.spawn(init_hud_info());
        world.spawn(init_game_hud(&assets_manager));

        stages_manager.start_stage_id(1, &assets_manager, get_time(), get_frame_time());

        Self {
            stages_manager,
            assets_manager,
            game_buffer,
            controls: init_controls(),
            playable_buffer,
            world,
            score: ScoreData::default(),
        }
    }

    pub async fn update(&mut self) {
        let time = get_time();
        let delta = get_frame_time();
        update_system(&mut self.world, &self.controls, time, delta);
        self.stages_manager.update(&mut self.world, time, delta);
    }

    pub async fn draw(&self) {
        let width = screen_width();
        let height = screen_height();
        let time = get_time();
        let delta = get_frame_time();
        clear_background(BLACK);

        // INFO : Begin drawing on the buffer space
        self.game_buffer.set_camera();
        clear_background(BLACK);

        let offset = vec2(0.03, 0.009);
        draw_texture_ex(
            self.playable_buffer.texture(),
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(0.64, 0.975)),
                ..Default::default()
            },
        );
        update_draw_hud(&self.world, &self.controls, &self.score, time, delta);
        draw_entity_number(self.world.len());
        self.game_buffer.done_camera();

        // INFO : Begin drawing on playable area
        self.playable_buffer.set_camera();
        clear_background(BLACK);

        let material = self.assets_manager.shaders.get("stg1-bg").unwrap();
        material.set_uniform("iTime", time as f32);
        material.set_uniform(
            "iResolution",
            vec2(
                self.playable_buffer.texture().width(),
                self.playable_buffer.texture().height(),
            ),
        );
        gl_use_material(&*material);
        draw_rectangle(0., 0., 1.0, 1.0, WHITE);
        gl_use_default_material();

        self.stages_manager.draw(time, delta);
        update_draw(&self.world, &self.controls, time, delta);
        self.playable_buffer.done_camera();

        // INFO : Draw the buffer to the screen
        let adjusted = get_adjusted_screen(DESIRED_ASPECT_RATIO);
        let offset = vec2((width - adjusted.x) / 2f32, (height - adjusted.y) / 2f32);
        draw_texture_ex(
            self.game_buffer.texture(),
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(adjusted),
                ..Default::default()
            },
        );
    }
}
