use std::sync::Arc;

use hecs::World;

pub struct SpawnEvent {
    pub start: f32,
    pub is_spawned: bool,
    pub spawn: Arc<dyn Fn(&mut World)>,
}

impl SpawnEvent {
    pub fn new(start: f32, spawn: Arc<dyn Fn(&mut World)>) -> Self {
        Self {
            start,
            is_spawned: false,
            spawn,
        }
    }
}

pub struct Spawner {
    timer: f32,
    lists: Vec<SpawnEvent>,
}

impl Spawner {
    pub fn new(lists: Vec<SpawnEvent>) -> Self {
        Self { lists, timer: 0. }
    }

    pub fn update(&mut self, world: &mut World, time: f32) {
        self.timer += time;
        self.lists
            .iter_mut()
            .filter(|event| !event.is_spawned && event.start < self.timer)
            .for_each(|event| {
                (event.spawn)(world);
                event.is_spawned = true;
            });

        self.lists.retain(|event| !event.is_spawned);
    }
}
