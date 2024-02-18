use std::future::Future;

use hecs::World;
use macroquad::{
    experimental::coroutines::{start_coroutine, stop_coroutine, wait_seconds},
    material::MaterialParams,
    math::vec2,
    time::get_time,
    window::next_frame,
};

use crate::{
    assets::{AssetsHandler, AssetsManager},
    window::Window,
};

use super::{loading::Loading, spawner::Spawner};

// If only the API is much stable on async end
// pub struct StageManager {
//     stages: Vec<Box<dyn Stage>>,
// }
//
// pub trait Stage {
//     fn preload(&mut self, _: &mut AssetsManager) -> Box<dyn Future<Output = ()>>;
//     fn start(&mut self) -> Box<dyn Future<Output = ()>>;
//     fn update(&mut self) -> Box<dyn Future<Output = ()>>;
//     fn draw(&self) -> Box<dyn Future<Output = ()>>;
// }

pub enum PreloadType<'a> {
    Sfx(&'a str, &'a str),
    Texture(&'a str, &'a str),
    Shader(&'a str, &'a str, &'a str, MaterialParams),
}

pub struct Stage<'a> {
    title: String,
    stage_number: String,
    start: Option<f64>,
    preloads: Option<Vec<PreloadType<'a>>>,
    background: Box<dyn Fn(&Window, &AssetsManager)>,
    spawner: Spawner,
}

impl<'a> Stage<'a> {
    pub fn new(
        title: &str,
        stage_number: &str,
        preloads: Vec<PreloadType<'a>>,
        spawner: Spawner,
        background: impl Fn(&Window, &AssetsManager) + 'static,
    ) -> Self {
        Self {
            title: title.to_string(),
            stage_number: stage_number.to_string(),
            preloads: Some(preloads),
            spawner,
            background: Box::new(background),
            start: None,
        }
    }
}

pub struct StageManager<'a> {
    stages: Vec<Stage<'a>>,
}

impl<'a> StageManager<'a> {
    pub const fn new(stages: Vec<Stage<'a>>) -> Self {
        Self { stages }
    }

    fn get_stage(&self) -> Option<&Stage<'a>> {
        self.stages.first()
    }

    fn get_mut_stage(&mut self) -> Option<&mut Stage<'a>> {
        self.stages.first_mut()
    }

    pub async fn preload(&mut self, assets_manager: &mut AssetsManager, screen: &Window) {
        let stage = self.get_mut_stage().unwrap();
        let mut loading_screen =
            Loading::new(stage.preloads.as_ref().unwrap().len(), vec2(0.4, 0.05));
        for preload_type in stage.preloads.take().unwrap() {
            loading_screen.draw(screen.game_window()).await;
            next_frame().await;
            match preload_type {
                PreloadType::Sfx(name, path) => {
                    assets_manager.sfx.register(name, path).await.unwrap()
                }
                PreloadType::Texture(name, path) => {
                    assets_manager.textures.register(name, path).await.unwrap()
                }
                PreloadType::Shader(name, vertex_path, fragment_path, params) => assets_manager
                    .shaders
                    .register(name, vertex_path, fragment_path, params)
                    .await
                    .unwrap(),
            };
        }
        let time = get_time();
        stage.start = Some(time);
        stage.spawner.start(time);
    }

    pub fn update(&mut self, time: f64, world: &mut World, assets_manager: &AssetsManager) {
        let stage = self.get_mut_stage().unwrap();
        stage.spawner.update(time, world, assets_manager)
    }

    pub fn draw(&self, screen: &Window, assets_manager: &AssetsManager) {
        let stage = self.get_stage().unwrap();
        (stage.background)(screen, assets_manager);
    }
}
