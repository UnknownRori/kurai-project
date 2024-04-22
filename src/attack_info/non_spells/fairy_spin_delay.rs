use std::f32::consts::PI;

use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;

use crate::{
    assets::konst::BULLET_RED,
    assets::Assets,
    cmpx,
    components::{
        AttackSpawner, Bullet, Enemy, Hitbox, MoveParams, Sprite, Transform2D, Waypoint,
        WaypointFactor, Waypoints,
    },
    math::ComplexExt,
    time::Timer,
};

#[derive(Debug)]
pub struct FairySpinDelay {
    bullet: Texture2D,
    bullet_count: usize,
    current_bullet: usize,
    angular_seperation: f32,
    last_position: Option<Complex<f32>>,
    last_player: Option<Complex<f32>>,

    timer: Timer,
}

impl FairySpinDelay {
    pub fn new(assets: &Assets) -> Self {
        let bullet = assets.textures.get(BULLET_RED).unwrap().clone();
        let bullet_count = 20;
        let current_bullet = 0;
        let angular_seperation = 2. * PI / (bullet_count as f32);
        let timer = Timer::new(2., true);

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

impl AttackSpawner for FairySpinDelay {
    fn spawn(&mut self, world: &mut World, current: &Transform2D, player: Option<&Transform2D>) {
        if player.is_none() {
            return;
        }

        if self.current_bullet == 0 {
            self.timer.update();
            if !self.timer.completed() {
                return;
            }

            self.last_position = Some(*current.position());
            self.last_player = Some(*player.unwrap().position());
        }

        self.current_bullet = (self.current_bullet + 1) % self.bullet_count;
        let dir = self
            .last_position
            .unwrap_or(*current.position())
            .dir(&self.last_player.unwrap_or(*player.unwrap().position()));

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
            Hitbox::new(0.010),
            Sprite::new(self.bullet.clone()),
            MoveParams::move_asymptotic_halflife(new_dir, new_dir, 1.),
            Waypoints::new(vec![Waypoint::new(
                3.,
                WaypointFactor::PreserveVelocity(MoveParams::move_accelerated(
                    cmpx!(0.),
                    new_dir * 0.2,
                )),
            )]),
        );

        world.spawn(component);
    }
}
