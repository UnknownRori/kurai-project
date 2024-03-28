use hecs::World;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    components::{
        controllable::Controllable,
        player::Player,
        velocity::{DampedVelocity, Velocity},
    },
    engine::components::Transform2D,
};

pub fn update_velocity(world: &mut World, delta: f32, time: f64) {
    world
        .query::<(&mut Transform2D, &mut Velocity)>()
        .without::<(&Player, &Controllable)>()
        .iter()
        .par_bridge()
        .for_each(|(_, (transform, vel))| match vel {
            Velocity::Normal(a) => transform.position += *a * delta,
            Velocity::Multiply(a) => transform.position *= *a * delta,
        });
}

pub fn update_vel_damper(world: &mut World, delta: f32, time: f64) {
    world
        .query::<(&mut Velocity, &DampedVelocity)>()
        .iter()
        .par_bridge()
        .for_each(|(_, (vel, damper))| match vel {
            Velocity::Normal(vel) => *vel *= 1. - damper.0 * delta,
            Velocity::Multiply(vel) => *vel *= 1. - damper.0 * delta,
        });
}
