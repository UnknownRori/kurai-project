pub mod remilia_scarlet;

use std::sync::Arc;

use hecs::{Entity, World};
use macroquad::texture::Texture2D;
use num_complex::Complex;

use crate::components::{
    CanShoot, Controllable, Enemy, EnemyBullet, Movable, Player, PlayerBullet, Position,
    SingleShoot, Sprite, TargetPlayer, Velocity,
};

pub type NormalFairyEntity<'a> = (
    &'a Enemy,
    &'a mut Position,
    &'a Movable,
    &'a CanShoot,
    &'a TargetPlayer,
    &'a SingleShoot,
    &'a Sprite,
);

pub type PlayerEntity<'a> = (
    &'a Player,
    &'a Controllable,
    &'a Movable,
    &'a mut Position,
    &'a CanShoot,
    &'a Sprite,
);

pub type PlayerBulletEntity<'a> = (
    &'a PlayerBullet,
    &'a mut Position,
    &'a Movable,
    &'a Velocity,
);
pub type NormalFairyBulletEntity<'a> =
    (&'a EnemyBullet, &'a mut Position, &'a Movable, &'a Velocity);
pub type BulletEntity<'a> = (&'a mut Position, &'a Movable, &'a Velocity);

pub fn spawn_generic_bullet(
    world: &mut World,
    current: &Position,
    target: &Position,
    velocity: Complex<f32>,
) -> Entity {
    todo!()
}

pub async fn spawn_enemy(world: &mut World, pos: Position, texture: Arc<Texture2D>) -> Entity {
    world.spawn((
        Enemy,
        pos,
        Movable::new(200.0),
        CanShoot::new(1.0, 500.0),
        TargetPlayer,
        SingleShoot,
        Sprite::new(texture),
    ))
}

pub fn spawn_player(world: &mut World, texture: Arc<Texture2D>) -> Entity {
    world.spawn((
        Player,
        Controllable,
        Movable::new(300.0),
        // Position::from_array([200.0, 450.0]), // TODO : Make starting position to middle bottom
        Position::from_array([200.0, 450.0]), // TODO : Make starting position to middle bottom
        CanShoot::new(5.0, 1000.0),
        Sprite::new(texture),
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
    );
    world.spawn(component)
}
