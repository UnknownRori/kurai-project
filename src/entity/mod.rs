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
        movement::MoveParams,
        player::Player,
    },
    engine::{
        assets::AssetsManager,
        components::{CircleHitbox2D, Hitpoint, Sprite2D, Transform2D},
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
    let attack = AttackInfo::new(
        GenericAttackInfo::new(1., 2.),
        Arc::new(FairyBurst::new(&assets_manager)),
    );

    Box::new(move |world| {
        world.spawn((
            Enemy,
            transform,
            sprite.clone(),
            hitpoint.clone(),
            attack.clone(),
            CircleHitbox2D::new(0.01),
        ));
    })
}
