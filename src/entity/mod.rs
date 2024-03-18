pub mod bullet;
pub mod player;

use std::sync::Arc;

use hecs::{Entity, World};
use macroquad::{math::vec2, texture::Texture2D};

use crate::{
    attack_info::nonspells::fairy_burst::FairyBurst,
    components::{
        attack_info::{AttackInfo, GenericAttackInfo},
        bullet::Bullet,
        enemy::Enemy,
        player::Player,
    },
    engine::{
        assets::AssetsManager,
        components::{
            CircleHitbox2D, Hitpoint, Movable, Movement, MovementQueue, Sprite2D, Transform2D,
            Velocity,
        },
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
    assets_manager: &AssetsManager,
    transform: Transform2D,
    texture: Arc<Texture2D>,
    movement: Vec<Movement>,
    hitpoint: Hitpoint,
) -> Box<dyn Fn(&mut World)> {
    let attack = AttackInfo::new(
        GenericAttackInfo::new(1., 2.),
        Arc::new(FairyBurst::new(&assets_manager)),
    );

    Box::new(move |world| {
        world.spawn((
            Enemy,
            transform,
            Movable::new(0.2, 0.4),
            Sprite2D::new(texture.clone()),
            MovementQueue::new(movement.clone()),
            hitpoint.clone(),
            attack.clone(),
            CircleHitbox2D::new(0.01),
        ));
    })
}
