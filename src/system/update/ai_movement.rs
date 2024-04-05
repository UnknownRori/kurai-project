use hecs::World;

use crate::components::{movement::MoveParams, waypoints::Waypoints};

pub fn enemy_movement_update(world: &mut World, time: f64, delta: f32) {
    world
        .query::<(&mut MoveParams, &mut Waypoints)>()
        .iter()
        .for_each(|(_, (move_params, waypoints))| {
            waypoints.update(move_params);
        });
}
