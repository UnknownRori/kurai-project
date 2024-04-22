use std::collections::HashMap;

use hecs::World;
use macroquad::prelude::*;

use crate::{
    assets::{preload, Assets},
    controls::{init_controls, Action, Controls},
    render::{compose, Renderer, ScreenShake},
    scene::Scene,
    score::ScoreData,
    stage::{ActiveStage, LazyStage, LazyStage1, StageManager},
    state::GameState,
    system::{
        attack_info_trigger, clean_up_bullet, collision, movement, player_control, waypoint_update,
    },
    ui::{GameOverUI, PauseUI},
    utils::FPSCounter,
    vec2,
};

pub struct Game {
    pub world: World,
    pub assets: Assets,
    pub fps: FPSCounter,
    pub renderer: Renderer,
    pub controls: Controls,
    pub screen_shake: ScreenShake,

    pub score: ScoreData,
    pub stage_manager: StageManager,
    pub scene: Scene,
    pub current_stage: Option<Box<dyn ActiveStage>>,

    pub state: GameState,
    should_quit: bool,

    pub pause_ui: PauseUI,
    pub game_over_ui: GameOverUI,
}

pub fn reset(game: &mut Game) {
    game.world.clear();
    game.state = GameState::Running;
    game.score = ScoreData::default();
    game.current_stage = Some(game.stage_manager.get("stage1").build(game));
}

impl Game {
    pub async fn new() -> Self {
        let world = World::new();
        let mut assets = Assets::new().await;
        let fps = FPSCounter::default();
        let renderer = Renderer::default();

        preload(&mut assets).await;

        let mut stages = HashMap::new();
        stages.insert(
            "stage1".to_owned(),
            Box::new(LazyStage1) as Box<dyn LazyStage>,
        );

        let stage_manager = StageManager::new(stages);
        let scene = Scene::Stage;
        let score = ScoreData::default();

        let screen_shake = ScreenShake::new();

        let mut g = Self {
            world,
            assets,
            fps,
            renderer,
            controls: init_controls(),
            screen_shake,

            score,
            stage_manager,
            scene,
            current_stage: None,

            state: GameState::Running,
            should_quit: false,

            pause_ui: PauseUI::default(),
            game_over_ui: GameOverUI::default(),
        };

        g.current_stage = Some(g.stage_manager.get("stage1").build(&g));

        g
    }

    pub async fn update(&mut self) {
        self.fps.update();
        let scene = self.scene.clone();
        match scene {
            Scene::MainMenu => todo!(),
            Scene::Options => todo!(),
            Scene::Stage => {
                match self.state {
                    GameState::Running => {
                        if self.controls.is_pressed(Action::Escape) {
                            self.state = GameState::Pause;
                        }

                        if self.score.life <= 0 {
                            self.state = GameState::GameOver;
                        }

                        // INFO : Shake the stage!
                        self.screen_shake.update();
                        self.renderer.stage.offset = self.screen_shake.get_shake_offset();

                        let mut stage = self
                            .current_stage
                            .take()
                            .expect("It should already set the stage");

                        // INFO : System
                        stage.update(self);
                        player_control(&mut self.world, &self.controls);
                        waypoint_update(&mut self.world);
                        movement(&self.world);
                        attack_info_trigger(&mut self.world);
                        clean_up_bullet(&mut self.world);
                        collision(&mut self.world, &mut self.score, &self.assets);

                        self.current_stage = Some(stage);
                    }
                    GameState::Pause => match self.pause_ui.update(&self.controls) {
                        Some(action) => match action {
                            crate::ui::PauseChoice::Resume => self.state = GameState::Running,
                            crate::ui::PauseChoice::Restart => {
                                reset(self);
                            }
                            crate::ui::PauseChoice::Exit => {
                                self.should_quit = true;
                            }
                        },
                        None => {}
                    },
                    GameState::GameOver => match self.game_over_ui.update(&self.controls) {
                        Some(action) => match action {
                            crate::ui::GameOverChoice::Continue => {}
                            crate::ui::GameOverChoice::Restart => reset(self),
                            crate::ui::GameOverChoice::Exit => self.should_quit = true,
                        },
                        None => {}
                    },
                }
            }
        }
    }

    pub fn draw(&self) {
        compose(self);
        macroquad_profiler::profiler(macroquad_profiler::ProfilerParams {
            fps_counter_pos: vec2!(0., 0.),
        });
    }

    pub async fn game_loop(&mut self) {
        loop {
            if self.should_quit {
                break;
            }

            self.update().await;
            self.draw();
            next_frame().await;
        }
    }
}
