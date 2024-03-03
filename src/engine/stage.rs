use std::future::Future;

use hecs::World;
use macroquad::{
    audio::play_sound_once,
    experimental::coroutines::{start_coroutine, stop_coroutine, wait_seconds},
    material::MaterialParams,
    math::vec2,
    time::get_time,
    window::next_frame,
};

use crate::{
    assets::{AssetsHandler, AssetsManager},
    time::Instant,
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
    Bgm(&'a str, &'a str),
}

pub trait StageBackground {
    fn draw(&self, _time: f64, _delta: f32, _screen: &Window);
    fn update(&mut self, _time: f64, _delta: f32);
}

pub struct Stage<'a> {
    title: String,
    stage_number: String,
    start: Option<Instant>,
    preloads: Option<Vec<PreloadType<'a>>>,
    background: Option<Box<dyn StageBackground>>,
    background_init: fn(&AssetsManager) -> Box<dyn StageBackground>,
    bgm: String, // TODO : Make this flexible and sensible
    spawner: Spawner,
}

impl<'a> Stage<'a> {
    pub fn new(
        title: &str,
        stage_number: &str,
        preloads: Vec<PreloadType<'a>>,
        bgm: String,
        spawner: Spawner,
        background_init: fn(&AssetsManager) -> Box<dyn StageBackground>,
    ) -> Self {
        Self {
            title: title.to_string(),
            stage_number: stage_number.to_string(),
            preloads: Some(preloads),
            bgm,
            spawner,
            background_init,
            background: None,
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
                PreloadType::Bgm(name, path) => {
                    assets_manager.bgm.register(name, path).await.unwrap()
                }
            };
        }
        let time = Instant::now();
        stage.background = (stage.background_init)(assets_manager).into();
        stage.start = Some(time);
        let sound = assets_manager.bgm.get(&stage.bgm).unwrap();
        play_sound_once(&*sound);
    }

    pub fn update(
        &mut self,
        time: f64,
        delta: f32,
        world: &mut World,
        assets_manager: &AssetsManager,
    ) {
        let stage = self.get_mut_stage().unwrap();
        stage.background.as_mut().unwrap().update(time, delta);
        stage
            .spawner
            .update(stage.start.unwrap().elapsed(time), world, assets_manager)
    }

    pub fn draw(&self, time: f64, delta: f32, screen: &Window, assets_manager: &AssetsManager) {
        let stage = self.get_stage().unwrap();
        stage.background.as_ref().unwrap().draw(time, delta, screen);
    }
}
