use hecs::World;
use macroquad::prelude::*;

use crate::{
    assets::preload::preload,
    controls::{init_controls, Action},
    engine::{assets::AssetsManager, controls::Controls, window::utils::get_adjusted_screen},
    konst::DESIRED_ASPECT_RATIO,
    render::{draw_main_ui, draw_stage, RenderingBuffer},
    scene::{stage::StageManager, stage1::Stage1Lazy},
    score::ScoreData,
    system::update_system,
    ui::game_hud::init_game_hud,
};

pub struct App {
    assets_manager: AssetsManager,
    font: Font,

    world: World,
    score: ScoreData,
    controls: Controls<Action>,
    render: RenderingBuffer,
    stages_manager: StageManager,
}

impl App {
    pub async fn new() -> Self {
        let render = RenderingBuffer::default();

        let mut assets_manager = AssetsManager::default();
        preload(&mut assets_manager).await;
        let mut world = World::new();

        let mut stages_manager = StageManager::new(vec![Box::new(Stage1Lazy)]);

        world.spawn(init_game_hud(&assets_manager));

        stages_manager.start_stage_id(1, &assets_manager, get_time(), get_frame_time());

        let mut font = load_ttf_font("./resources/fonts/AveriaSansLibre-Regular.ttf")
            .await
            .unwrap();
        font.set_filter(FilterMode::Nearest);

        Self {
            stages_manager,
            font,

            world,
            render,
            assets_manager,
            controls: init_controls(),
            score: ScoreData::default(),
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
            &self.render,
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
            &self.render,
            &self.controls,
            time,
            delta,
        );

        // INFO : Draw the buffer to the screen
        let adjusted = get_adjusted_screen(DESIRED_ASPECT_RATIO);
        let offset = vec2((width - adjusted.x) / 2f32, (height - adjusted.y) / 2f32);
        draw_texture_ex(
            self.render.ui.texture(),
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(adjusted),
                ..Default::default()
            },
        );

        macroquad_profiler::profiler(macroquad_profiler::ProfilerParams {
            fps_counter_pos: vec2(0., 0.),
        });
    }
}
