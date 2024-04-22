use hecs::World;

use crate::assets::Assets;

use super::player::EntitySpawnAction;

pub struct SpawnEvent {
    pub start: f32,
    pub is_spawned: bool,
    pub spawn: EntitySpawnAction,
}

impl SpawnEvent {
    pub fn new(start: f32, spawn: EntitySpawnAction) -> Self {
        Self {
            start,
            spawn,
            is_spawned: false,
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

    pub fn update(&mut self, world: &mut World, assets: &Assets, time: f32) {
        self.timer += time;
        self.lists
            .iter_mut()
            .filter(|event| !event.is_spawned && event.start < self.timer)
            .for_each(|event| {
                (event.spawn)(world, assets);
                event.is_spawned = true;
            });

        self.lists.retain(|event| !event.is_spawned);
    }
}
