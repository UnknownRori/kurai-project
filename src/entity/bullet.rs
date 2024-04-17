use hecs::World;

use crate::components::{
    bullet::Bullet, circle_hitbox2d::CircleHitbox2D, enemy::Enemy, movement::MoveParams,
    sprite2d::Sprite2D, transform2d::Transform2D,
};

pub fn spawn_generic_bullet(
    world: &mut World,
    transform: Transform2D,
    velocity: MoveParams,
    sprite: Sprite2D,
) {
    let component = (
        Enemy,
        Bullet,
        transform,
        CircleHitbox2D::new(0.010),
        sprite,
        velocity,
    );

    world.spawn(component);
}
