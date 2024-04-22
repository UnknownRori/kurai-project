mod stage1;

pub use stage1::{LazyStage1, Stage1};

use std::collections::HashMap;

use crate::Game;

pub struct StageManager {
    stages: HashMap<String, Box<dyn LazyStage>>,
}

impl StageManager {
    pub fn new(stages: HashMap<String, Box<dyn LazyStage>>) -> Self {
        Self { stages }
    }

    pub fn get(&self, name: &str) -> &Box<dyn LazyStage> {
        &self.stages.get(name).unwrap()
    }
}

pub trait LazyStage {
    fn build(&self, _: &Game) -> Box<dyn ActiveStage>;
}

pub enum StageType {
    Story,
}

pub trait ActiveStage {
    fn title(&self) -> &str;
    fn sub_title(&self) -> &str;
    fn stage_type(&self) -> &StageType;
    fn update(&mut self, _: &mut Game);
    fn draw(&self, _: &Game);
}
