use hecs::World;

use crate::engine::assets::AssetsManager;

use super::{lazy_stage::LazyStage, scene::Scene, stage_info::StageInfo};

pub struct StageManager {
    current: Option<Box<dyn Stage>>,
    stages: Vec<Box<dyn LazyStage>>,
}

impl StageManager {
    pub fn new(stages: Vec<Box<dyn LazyStage>>) -> Self {
        Self {
            current: None,
            stages,
        }
    }

    pub fn start_stage_id(
        &mut self,
        id: usize,
        assets_manager: &AssetsManager,
        time: f64,
        delta: f32,
    ) {
        let filtered = self
            .stages
            .iter()
            .filter(|stage| stage.id() == id)
            .collect::<Vec<_>>();
        let stage = filtered.first().expect("stage not found!");

        let mut stage = stage.build(assets_manager);
        stage.start(time, delta);
        self.current = Some(stage);
    }

    pub fn update(&mut self, world: &mut World, time: f64, delta: f32) {
        if let Some(stage) = &mut self.current {
            stage.update(world, time, delta);
        }
    }

    pub fn draw(&self, time: f64, delta: f32) {
        if let Some(stage) = &self.current {
            stage.draw(time, delta);
        }
    }
}

pub trait Stage: Scene + StageInfo {}
