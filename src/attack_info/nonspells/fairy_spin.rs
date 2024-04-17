use std::f32::consts::PI;

use macroquad::prelude::*;
use num_complex::Complex;

use crate::{
    assets::konst::RED_BULLET,
    assets::AssetsManager,
    components::{
        attack_info::AttackSpawner, bullet::Bullet, circle_hitbox2d::CircleHitbox2D, enemy::Enemy,
        movement::MoveParams, sprite2d::Sprite2D, transform2d::Transform2D,
    },
    math::ComplexExt,
    time::Timer,
};

#[derive(Debug)]
pub struct FairySpin {
    bullet: Texture2D,
    bullet_count: usize,
    current_bullet: usize,
    angular_seperation: f32,
    last_position: Option<Complex<f32>>,
    last_player: Option<Complex<f32>>,

    timer: Timer,
}

impl FairySpin {
    pub fn new(assets: &AssetsManager) -> Self {
        let bullet = assets.textures.get(RED_BULLET).unwrap();
        let bullet_count = 20;
        let current_bullet = 0;
        let angular_seperation = 2. * PI / (bullet_count as f32);
        let timer = Timer::new(0.5, true);

        Self {
            bullet,
            bullet_count,
            current_bullet,
            angular_seperation,
            last_position: None,
            last_player: None,

            timer,
        }
    }
}

impl AttackSpawner for FairySpin {
    fn spawn(
        &mut self,
        world: &mut hecs::World,
        current: &Transform2D,
        player: &Transform2D,
        _delta: f32,
    ) {
        if self.current_bullet == 0 {
            self.timer.update();
            if !self.timer.completed() {
                return;
            }

            self.last_position = Some(*current.position());
            self.last_player = Some(*player.position());
        }

        self.current_bullet = (self.current_bullet + 1) % self.bullet_count;
        let dir = self
            .last_position
            .unwrap_or(*current.position())
            .dir(&self.last_player.unwrap_or(*player.position()));

        const BULLET_SPEED: f32 = 0.5;
        let angle = self.current_bullet as f32 * self.angular_seperation;
        // TODO : Make the first shot directed to player and then start the rotation from there
        let new_dir = dir * Complex::cdir(angle) * BULLET_SPEED;

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
            MoveParams::move_linear(new_dir),
        );

        world.spawn(component);
    }
}
