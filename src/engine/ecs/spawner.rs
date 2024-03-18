use std::sync::Arc;

use hecs::World;

use crate::engine::time::Instant;

pub struct SpawnEvent {
    pub start: f64,
    pub is_spawned: bool,
    pub spawn: Arc<dyn Fn(&mut World)>,
}

impl SpawnEvent {
    pub fn new(start: f64, spawn: Arc<dyn Fn(&mut World)>) -> Self {
        Self {
            start,
            is_spawned: false,
            spawn,
        }
    }
}

pub struct Spawner {
    start: Option<Instant>,
    lists: Vec<SpawnEvent>,
}

impl Spawner {
    pub fn new(lists: Vec<SpawnEvent>) -> Self {
        Self { lists, start: None }
    }

    pub fn start(&mut self, time: f64) {
        self.start = Some(Instant::new(time));
    }

    pub fn update(&mut self, world: &mut World, time: f64) {
        self.lists
            .iter_mut()
            .filter(|event| !event.is_spawned && event.start < self.start.unwrap().elapsed(time))
            .for_each(|event| {
                (event.spawn)(world);
                event.is_spawned = true;
            });

        self.lists.retain(|event| !event.is_spawned);
    }
}
