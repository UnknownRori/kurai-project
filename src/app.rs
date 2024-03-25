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
    render::{draw_main_ui, draw_stage},
    scene::{stage::StageManager, stage1::Stage1Lazy},
    score::ScoreData,
    system::update_system,
    ui::game_hud::init_game_hud,
};

pub struct App {
    assets_manager: AssetsManager,
    game_buffer: ScreenBuffer2D,
    playable_buffer: ScreenBuffer2D,
    controls: Controls<Action>,
    world: World,
    stages_manager: StageManager,
    score: ScoreData,
    font: Font,
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
            font: load_ttf_font("./resources/fonts/AveriaSansLibre-Regular.ttf")
                .await
                .unwrap(),
        }
    }

    pub async fn update(&mut self) {
        let time = get_time();
        let delta = get_frame_time();
        update_system(
            &mut self.world,
            &self.controls,
            &mut self.score,
            time,
            delta,
        );
        self.stages_manager.update(&mut self.world, time, delta);
    }

    pub async fn draw(&self) {
        let width = screen_width();
        let height = screen_height();
        let time = get_time();
        let delta = get_frame_time();
        clear_background(BLACK);

        // INFO : Begin drawing on the buffer space
        draw_main_ui(
            &self.world,
            &self.game_buffer,
            &self.playable_buffer,
            &self.controls,
            &self.font,
            &self.score,
            time,
            delta,
        );

        draw_stage(
            &self.world,
            &self.assets_manager,
            &self.stages_manager,
            &self.playable_buffer,
            &self.controls,
            time,
            delta,
        );

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
