use std::{
    cell::Cell,
    sync::{Arc, Mutex},
};

use hecs::World;
use macroquad::texture::Texture2D;

use crate::{
    assets::Assets,
    attack_info::players::RemiliaBasicAttack,
    cmpx,
    components::{
        AttackInfo, Bullet, Controllable, Hitbox, MoveParams, Player, PlayerAttackInfo, SpellInfo,
        Sprite, Transform2D,
    },
    vec2,
};

pub type EntitySpawnAction = fn(&mut World, &Assets);

pub fn player_spawn(world: &mut World, assets: &Assets) {
    let remi = assets.textures.get("remi").unwrap().clone();
    let transform = Transform2D::new(cmpx!(0.5, 0.8), vec2!(0.11), 0.);
    let hitbox = Hitbox::new(0.01);

    let attack_info = PlayerAttackInfo {
        focus: AttackInfo::new(Arc::new(Mutex::new(RemiliaBasicAttack::new(assets)))),
        normal: AttackInfo::new(Arc::new(Mutex::new(RemiliaBasicAttack::new(assets)))),
        spell: SpellInfo::new(
            "Spear of Gugnir",
            16.,
            Arc::new(Mutex::new(RemiliaBasicAttack::new(assets))),
        ),
    };

    world.spawn((
        Player,
        Controllable,
        MoveParams::move_dampen(cmpx!(0.), 0.85),
        Sprite::new(remi),
        attack_info,
        transform,
        hitbox,
    ));
}

pub fn player_bullet_spawn(
    world: &mut World,
    sprite: &Texture2D,
    transform: Transform2D,
    movement: MoveParams,
) {
    let transform = Transform2D {
        scale: vec2!(0.15, 0.15),
        ..transform
    };

    let components = (
        Player,
        Bullet,
        transform,
        movement,
        Sprite::new(sprite.clone()),
        Hitbox::new(0.010),
    );

    world.spawn(components);
}
