use std::sync::{Arc, RwLock};

use hecs::World;
use macroquad::prelude::*;
use num_traits::ToPrimitive;

use crate::{assets::AssetsManager, engine::spawner::SpawnEvent};

pub fn spawner_line(
    start: f64,
    delay_between: f64,
    total: usize,
    spawn_event: impl FnMut(&mut World, &AssetsManager) + 'static,
) -> Vec<SpawnEvent> {
    let spawn_event = Arc::new(RwLock::new(spawn_event));
    let mut temp = Vec::with_capacity(total);

    for i in 0..total {
        let spawn_clone = Arc::clone(&spawn_event);
        temp.push(SpawnEvent::new2(
            start + (delay_between * i.to_f64().unwrap()),
            spawn_clone,
        ));
    }
    temp
}
