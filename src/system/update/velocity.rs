use hecs::World;

use crate::engine::components::{Transform2D, Velocity};

pub fn update_velocity(world: &mut World, delta: f32) {
    world
        .query::<(&mut Transform2D, &Velocity)>()
        .iter()
        .for_each(|(_, (transform, velocity))| match velocity {
            Velocity::Normal(vel) => transform.position += vel * delta,
            Velocity::Multiply(_) => todo!(),
        })
}
