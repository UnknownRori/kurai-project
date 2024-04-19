pub mod bullet;
pub mod player;
mod spawner;

pub use spawner::{SpawnEvent, Spawner};

use std::sync::{Arc, Mutex};

use hecs::{Entity, World};
use macroquad::math::vec2;

use crate::{
    assets::AssetsManager,
    attack_info::nonspells::{
        fairy_burst::FairyBurst, fairy_spin::FairySpin, fairy_spin_delay::FairySpinDelay,
    },
    components::{
        attack_info::AttackInfo, bullet::Bullet, circle_hitbox2d::CircleHitbox2D, enemy::Enemy,
        hitpoint::Hitpoint, movement::MoveParams, player::Player, sprite2d::Sprite2D,
        transform2d::Transform2D, waypoints::Waypoints,
    },
};

pub fn spawn_player_bullet(
    world: &mut World,
    transform: &Transform2D,
    sprite: Sprite2D,
    movement: MoveParams,
) -> Entity {
    let transform = Transform2D {
        scale: vec2(0.15, 0.15),
        ..*transform
    };

    let component = (
        Player,
        Bullet,
        transform,
        movement,
        CircleHitbox2D::new(0.010),
        sprite,
    );

    world.spawn(component)
}

pub fn lazy_spawn_enemy(
    assets_manager: &AssetsManager,
    transform: Transform2D,
    sprite: Sprite2D,
    hitpoint: Hitpoint,
) -> Box<dyn Fn(&mut World)> {
    let attack = Arc::new(Mutex::new(FairyBurst::new(&assets_manager)));

    Box::new(move |world| {
        world.spawn((
            Enemy,
            transform,
            sprite.clone(),
            hitpoint.clone(),
            MoveParams::default(),
            AttackInfo::new(attack.clone()),
            CircleHitbox2D::new(0.03),
        ));
    })
}

pub fn lazy_spawn_enemy2(
    assets_manager: &AssetsManager,
    transform: Transform2D,
    sprite: Sprite2D,
    hitpoint: Hitpoint,
    waypoints: Waypoints,
) -> Box<dyn Fn(&mut World)> {
    let attack = Arc::new(Mutex::new(FairySpin::new(&assets_manager)));
    // let attack = Arc::new(Mutex::new(FairySpinDelay::new(&assets_manager)));

    Box::new(move |world| {
        world.spawn((
            Enemy,
            transform,
            sprite.clone(),
            hitpoint.clone(),
            MoveParams::default(),
            // waypoints.clone(),
            AttackInfo::new(attack.clone()),
            CircleHitbox2D::new(0.03),
        ));
    })
}
