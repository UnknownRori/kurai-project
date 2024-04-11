use hecs::World;
use macroquad::prelude::*;

use crate::{
    assets::preload::preload,
    controls::{init_controls, Action},
    engine::{
        assets::AssetsManager,
        camera::screen_shake::{self, ScreenShake},
        controls::Controls,
        fps_counter::FPSCounter,
        window::utils::get_adjusted_screen,
    },
    konst::DESIRED_ASPECT_RATIO,
    pause::Pause,
    render::{draw_main_ui, draw_stage, RenderingBuffer},
    scene::{stage::StageManager, stage1::Stage1Lazy},
    score::ScoreData,
    system::update_system,
    ui::game_hud::init_game_hud,
};

pub struct App {
    assets_manager: AssetsManager,
    fps_counter: FPSCounter,
    font: Font,

    world: World,
    score: ScoreData,
    controls: Controls<Action>,
    render: RenderingBuffer,
    stages_manager: StageManager,

    screen_shake: ScreenShake,
    pause: Pause,
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

        let screen_shake = ScreenShake::new();

        Self {
            stages_manager,
            fps_counter: FPSCounter::default(),
            font,

            world,
            render,
            assets_manager,
            controls: init_controls(),
            score: ScoreData::default(),

            screen_shake,
            pause: Default::default(),
        }
    }

    pub async fn update(&mut self) {
        let time = get_time();
        let delta = get_frame_time();

        self.fps_counter.update();
        self.pause.update(&self.controls);

        if self.pause.is_paused() {
            self.screen_shake.update();
            self.render.stage.camera.offset = self.screen_shake.get_shake_offset();

            update_system(
                &mut self.world,
                &self.controls,
                &mut self.score,
                time,
                delta,
            );
            self.stages_manager.update(&mut self.world, time, delta);
        }
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
            &self.fps_counter,
            time,
            delta,
        );

        draw_stage(
            &self.world,
            &self.assets_manager,
            &self.stages_manager,
            &self.render,
            &self.controls,
            &self.pause,
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
