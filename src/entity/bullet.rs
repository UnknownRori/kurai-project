use std::sync::Arc;

use hecs::World;
use macroquad::texture::Texture2D;

use crate::{
    components::{bullet::Bullet, enemy::Enemy, movement::MoveParams},
    engine::components::{CircleHitbox2D, Sprite2D, Transform2D},
};

pub fn spawn_generic_bullet(
    world: &mut World,
    transform: Transform2D,
    velocity: MoveParams,
    texture: Arc<Texture2D>,
) {
    let component = (
        Enemy,
        Bullet,
        transform,
        CircleHitbox2D::new(0.010),
        Sprite2D::new(texture),
        velocity,
    );

    world.spawn(component);
}
