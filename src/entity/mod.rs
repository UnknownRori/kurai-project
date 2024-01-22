pub mod remilia_scarlet;

use hecs::{Entity, World};
use macroquad::math::Vec2;

use crate::components::{
    CanShoot, Controllable, DummyDraw, Movable, Player, PlayerBullet, Position, Velocity,
};

pub type PlayerEntity<'a> = (
    &'a Player,
    &'a Controllable,
    &'a Movable,
    &'a mut Position,
    &'a CanShoot,
    &'a DummyDraw,
);

pub type PlayerBulletEntity<'a> = (&'a PlayerBullet, &'a Position, &'a Movable, &'a Velocity);
pub type BulletEntity<'a> = (&'a Position, &'a Movable, &'a Velocity);

pub fn spawn_player(world: &mut World) -> Entity {
    // TODO : Make this support a bunch of protagonist
    world.spawn((
        Player,
        Controllable,
        Movable::default(),
        Position::default(),
        CanShoot::default(),
        DummyDraw,
    ))
}

pub fn spawn_player_bullet(world: &mut World, position: &Position, velocity: Vec2) -> Entity {
    // TODO : Refactor this later
    let component = (
        PlayerBullet,
        *position,
        Movable::new(50.0),
        Velocity::from_vec2(velocity),
    );
    world.spawn(component)
}
