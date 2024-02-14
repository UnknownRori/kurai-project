pub mod remilia_scarlet;

use std::sync::Arc;

use hecs::{Entity, World};
use macroquad::texture::Texture2D;
use num_complex::Complex;

use crate::{
    components::{
        CanShoot, Controllable, Enemy, EnemyBullet, Hitbox, Movable, MovementQueue, Player,
        PlayerBullet, Position, SingleShoot, Sprite, TargetPlayer, Velocity,
    },
    math::ExtendedComplexNumber,
};

pub type NormalFairyEntity<'a> = (
    &'a Enemy,
    &'a mut Position,
    &'a Movable,
    &'a CanShoot,
    &'a TargetPlayer,
    &'a SingleShoot,
    &'a Sprite,
    &'a MovementQueue,
    &'a Hitbox,
);

pub type PlayerEntity<'a> = (
    &'a Player,
    &'a Controllable,
    &'a Movable,
    &'a mut Position,
    &'a CanShoot,
    &'a Sprite,
    &'a Hitbox,
);

pub type PlayerBulletEntity<'a> = (
    &'a PlayerBullet,
    &'a mut Position,
    &'a Movable,
    &'a Velocity,
    &'a Hitbox,
);

pub type EnemyMovableEntity<'a> = (
    &'a Enemy,
    &'a mut Position,
    &'a Movable,
    &'a mut MovementQueue,
);
pub type DrawableEnemyEntity<'a> = (&'a Enemy, &'a Position, &'a Sprite);
pub type NormalFairyBulletEntity<'a> = (
    &'a EnemyBullet,
    &'a mut Position,
    &'a Movable,
    &'a Velocity,
    &'a Hitbox,
    &'a Sprite,
);
pub type BulletEntity<'a> = (
    &'a mut Position,
    &'a Movable,
    &'a Velocity,
    &'a Hitbox,
    &'a Sprite,
);

pub fn spawn_generic_bullet(
    world: &mut World,
    current: &Position,
    target: &Position,
    speed: f32,
    texture: Arc<Texture2D>,
) -> Entity {
    let direction = (target.position - current.position).normalize() * speed;

    world.spawn((
        EnemyBullet,
        current.clone(),
        Movable::default(),
        Velocity::from(direction),
        Hitbox::new(0.008),
        Sprite::new(texture),
    ))
}

pub fn spawn_enemy(
    world: &mut World,
    pos: Position,
    texture: Arc<Texture2D>,
    movement: MovementQueue,
) -> Entity {
    world.spawn((
        Enemy,
        pos,
        Movable::new(0.2, 0.4),
        CanShoot::new(1.0, 1.5),
        TargetPlayer,
        SingleShoot,
        Sprite::new(texture),
        movement,
        Hitbox::new(0.02),
    ))
}

pub fn spawn_player(world: &mut World, texture: Arc<Texture2D>) -> Entity {
    world.spawn((
        Player,
        Controllable,
        Movable::new(1.0, 1.0),
        Position::from_array([0.5, 0.8]),
        CanShoot::new(40.0, 1.5),
        Sprite::new(texture),
        Hitbox::new(0.008),
    ))
}

pub fn spawn_player_bullet(
    world: &mut World,
    position: &Position,
    velocity: Complex<f32>,
) -> Entity {
    // TODO : Refactor this later
    let component = (
        PlayerBullet,
        *position,
        Movable::default(),
        Velocity::from(velocity),
        Hitbox::new(0.008),
    );
    world.spawn(component)
}
