use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use hecs::{Entity, World};

use crate::assets::AssetsManager;

pub type SpawnerAction = Arc<RwLock<dyn FnMut(&mut World, &AssetsManager)>>;

pub struct SpawnEvent {
    pub start: f64,
    pub spawn_event: SpawnerAction,
    pub is_spawned: bool,
}

impl SpawnEvent {
    pub fn new(start: f64, spawn_event: impl FnMut(&mut World, &AssetsManager) + 'static) -> Self {
        Self {
            start,
            spawn_event: Arc::new(RwLock::new(spawn_event)),
            is_spawned: false,
        }
    }

    pub fn new2(
        start: f64,
        spawn_event: Arc<RwLock<dyn FnMut(&mut World, &AssetsManager)>>,
    ) -> Self {
        Self {
            start,
            spawn_event,
            is_spawned: false,
        }
    }
}

pub struct Spawner {
    start: Option<f64>,
    lists: Vec<SpawnEvent>,
}

impl Spawner {
    pub fn new(lists: Vec<SpawnEvent>) -> Self {
        Self { start: None, lists }
    }

    pub fn start(&mut self, time: f64) {
        if self.start.is_none() {
            self.start = Some(time);
        }
    }

    pub fn update(&mut self, time: f64, world: &mut World, assets_manager: &AssetsManager) {
        self.lists
            .iter_mut()
            .filter(|spawn_event| {
                !spawn_event.is_spawned && self.start.unwrap() + spawn_event.start < time
            })
            .for_each(|event| {
                let mut a = event.spawn_event.write().unwrap();
                (a)(world, assets_manager);
                event.is_spawned = true;
            });

        self.lists.retain(|spawn_event| !spawn_event.is_spawned);
    }
}
