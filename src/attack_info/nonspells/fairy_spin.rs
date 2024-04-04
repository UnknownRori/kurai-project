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
        time::Timer,
    },
};

#[derive(Debug)]
pub struct FairySpin {
    bullet: Texture2D,
    bullet_count: usize,
    current_bullet: usize,
    angular_seperation: f32,

    timer: Timer,
}

impl FairySpin {
    pub fn new(assets: &AssetsManager) -> Self {
        let bullet = assets.textures.get(RED_BULLET).unwrap();
        let bullet_count = 15;
        let current_bullet = 0;
        let angular_seperation = 2.0 * PI / (bullet_count as f32);
        let timer = Timer::new(0.1, true);

        Self {
            bullet,
            bullet_count,
            current_bullet,
            angular_seperation,

            timer,
        }
    }
}

impl AttackSpawner for FairySpin {
    fn spawn(
        &mut self,
        world: &mut hecs::World,
        current: &crate::engine::components::Transform2D,
        player: &crate::engine::components::Transform2D,
        _delta: f32,
    ) {
        self.timer.update();
        if !self.timer.completed() {
            return;
        }

        self.current_bullet = self.current_bullet + 1 % self.bullet_count;
        let dir = current.position().dir(player.as_ref());

        let angle = self.current_bullet as f32 * self.angular_seperation;
        let new_dir = if self.current_bullet == 0 {
            dir * Complex::from_polar(1., angle)
        } else {
            Complex::from_polar(1., angle)
        };

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

        world.spawn(component);

        // let bullet_count = 30;
        // let angular_seperation = 2.0 * PI / (bullet_count as f32);
        // let batch = (0..bullet_count)
        //     .map(|i| {
        //         let angle = i as f32 * angular_seperation;
        //         let new_dir = dir * Complex::from_polar(1., angle);
        //
        //         let transform = Transform2D {
        //             scale: vec2(0.03, 0.03),
        //             rotation: new_dir.rot(),
        //             ..*current
        //         };
        //
        //         let component = (
        //             Enemy,
        //             Bullet,
        //             transform,
        //             CircleHitbox2D::new(0.010),
        //             Sprite2D::new(self.bullet.clone()),
        //             MoveParams::move_accelerated(cmpx!(0.), new_dir),
        //         );
        //
        //         component
        //     })
        //     .collect::<Vec<_>>();
        //
        // world.spawn_batch(batch);
    }
}
