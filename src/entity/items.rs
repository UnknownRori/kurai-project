use hecs::World;
use num_complex::Complex;

use crate::{
    assets::{
        konst::{POINT, POWER},
        Assets,
    },
    cmpx,
    components::{DropItemType, Hitbox, Item, MoveParams, Sprite, Transform2D},
    math::{ComplexExt, ToComplex},
    utils::rand_dir,
    vec2,
};

pub fn point_spawn(world: &mut World, transform: &Transform2D, assets: &Assets) {
    let sprite = assets.textures.get(POINT).unwrap().clone();
    let transform = Transform2D {
        scale: vec2!(0.03),
        ..*transform
    };
    let first_dir = cmpx!(0., -1.5) * (rand_dir() * 0.35).to_cmpx();
    let fall = cmpx!(0., 12.);

    world.spawn((
        Item,
        Sprite::new(sprite),
        DropItemType::Value,
        transform,
        Hitbox::new(0.05),
        MoveParams::move_asymptotic_halflife(first_dir, fall, 5.),
    ));
}

pub fn power_spawn(world: &mut World, transform: &Transform2D, assets: &Assets) {
    let sprite = assets.textures.get(POWER).unwrap().clone();
    let transform = Transform2D {
        scale: vec2!(0.03),
        ..*transform
    };
    let first_dir = cmpx!(0., -1.5) * (rand_dir() * 0.35).to_cmpx();
    let fall = cmpx!(0., 12.);

    world.spawn((
        Item,
        Sprite::new(sprite),
        DropItemType::Power,
        transform,
        Hitbox::new(0.05),
        MoveParams::move_asymptotic_halflife(first_dir, fall, 5.),
    ));
}
