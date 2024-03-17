pub mod player;

use std::sync::Arc;

use hecs::{Entity, World};
use macroquad::{math::vec2, texture::Texture2D};

use crate::{
    components::{bullet::Bullet, enemy::Enemy, player::Player},
    engine::{
        components::{
            CircleHitbox2D, Hitpoint, Movable, Movement, MovementQueue, Sprite2D, Transform2D,
            Velocity,
        },
        math::ComplexExt,
    },
};

pub fn spawn_player_bullet(
    world: &mut World,
    transform: &Transform2D,
    texture: Arc<Texture2D>,
    velocity: Velocity,
) -> Entity {
    let transform = Transform2D {
        scale: vec2(0.15, 0.15),
        ..*transform
    };

    let component = (
        Player,
        Bullet,
        transform,
        Movable::default(),
        CircleHitbox2D::new(0.010),
        Sprite2D::new(texture),
        velocity,
    );

    world.spawn(component)
}

pub fn lazy_spawn_enemy(
    transform: Transform2D,
    texture: Arc<Texture2D>,
    movement: Vec<Movement>,
    hitpoint: Hitpoint,
) -> Box<dyn Fn(&mut World)> {
    Box::new(move |world| {
        world.spawn((
            Enemy,
            transform,
            Movable::new(0.2, 0.4),
            Sprite2D::new(texture.clone()),
            MovementQueue::new(movement.clone()),
            hitpoint.clone(),
        ));
    })
}

pub fn spawn_enemy(
    world: &mut World,
    transform: Transform2D,
    texture: Arc<Texture2D>,
    movement: MovementQueue,
    hitpoint: Hitpoint,
) {
    world.spawn((
        Enemy,
        transform,
        Movable::new(0.2, 0.4),
        Sprite2D::new(texture),
        movement,
        hitpoint,
    ));
}

// TODO : Not completed
pub fn spawn_generic_bullet(
    world: &mut World,
    current: &Transform2D,
    target: &Transform2D,
    velocity: Velocity,
    texture: Arc<Texture2D>,
) -> Entity {
    let speed = 0.5;
    let direction = (target.position - current.position).normalize() * speed;
    let rot = direction.conj().arg() - std::f32::consts::FRAC_PI_2;

    world.spawn((
        Enemy,
        Bullet,
        Sprite2D::new(texture),
        Transform2D::new(current.position, vec2(0.1, 0.1), rot),
        CircleHitbox2D::new(0.004),
        velocity,
    ))
}
