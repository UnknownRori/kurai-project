use std::sync::Arc;

use hecs::World;
use macroquad::texture::Texture2D;

use crate::{
    components::{bullet::Bullet, enemy::Enemy},
    engine::components::{CircleHitbox2D, Movable, Sprite2D, Transform2D, Velocity},
};

pub fn spawn_generic_bullet(
    world: &mut World,
    transform: Transform2D,
    velocity: Velocity,
    texture: Arc<Texture2D>,
) {
    let component = (
        Enemy,
        Bullet,
        transform,
        Movable::default(),
        CircleHitbox2D::new(0.010),
        Sprite2D::new(texture),
        velocity,
    );

    world.spawn(component);
}
