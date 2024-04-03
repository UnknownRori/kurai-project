use std::f32::consts::PI;

use macroquad::prelude::*;
use num_complex::Complex;

use crate::{
    assets::konst::RED_BULLET,
    cmpx,
    components::{attack_info::AttackSpawner, bullet::Bullet, enemy::Enemy, movement::MoveParams},
    engine::{
        assets::AssetsManager,
        components::{CircleHitbox2D, Sprite2D, Transform2D},
        math::ComplexExt,
    },
};

#[derive(Debug)]
pub struct FairySpin {
    bullet: Texture2D,
}

impl FairySpin {
    pub fn new(assets: &AssetsManager) -> Self {
        let bullet = assets.textures.get(RED_BULLET).unwrap();

        Self { bullet }
    }
}

impl AttackSpawner for FairySpin {
    fn spawn(
        &self,
        world: &mut hecs::World,
        current: &crate::engine::components::Transform2D,
        player: &crate::engine::components::Transform2D,
        bullet_speed: f32,
        delta: f32,
    ) {
        let dir = current.position().dir(player.as_ref());

        let bullet_count = 30;
        let angular_seperation = 2.0 * PI / (bullet_count as f32);
        let batch = (0..bullet_count)
            .map(|i| {
                let angle = i as f32 * angular_seperation;
                let new_dir = dir * Complex::from_polar(1., angle);

                let transform = Transform2D {
                    scale: vec2(0.03, 0.03),
                    rotation: new_dir.rot(),
                    ..*current
                };

                let component = (
                    Enemy,
                    Bullet,
                    transform,
                    CircleHitbox2D::new(0.010),
                    Sprite2D::new(self.bullet.clone()),
                    MoveParams::move_accelerated(cmpx!(0.), new_dir),
                );

                component
            })
            .collect::<Vec<_>>();

        world.spawn_batch(batch);
    }
}
