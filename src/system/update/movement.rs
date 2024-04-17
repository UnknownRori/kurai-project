use hecs::World;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{components::movement::MoveParams, components::transform2d::Transform2D};

pub fn update_movement(world: &mut World, delta: f32) {
    world
        .query::<(&mut Transform2D, &mut MoveParams)>()
        .iter()
        .par_bridge()
        .for_each(|(_, (transform, move_params))| {
            move_params.update(&mut transform.position, delta);
        });
}
