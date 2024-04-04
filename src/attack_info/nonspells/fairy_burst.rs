use macroquad::prelude::*;

use crate::{
    assets::konst::RED_BULLET,
    components::{attack_info::AttackSpawner, bullet::Bullet, enemy::Enemy, movement::MoveParams},
    engine::{
        assets::AssetsManager,
        components::{CircleHitbox2D, Sprite2D, Transform2D},
        math::ComplexExt,
        time::Timer,
    },
};

#[derive(Debug)]
pub struct FairyBurst {
    bullet: Texture2D,
    timer: Timer,
}

impl FairyBurst {
    pub fn new(assets: &AssetsManager) -> Self {
        let bullet = assets.textures.get(RED_BULLET).unwrap();
        let timer = Timer::new(0.5, true);

        Self { bullet, timer }
    }
}

impl AttackSpawner for FairyBurst {
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

        // Get the direction from target position (player)
        let dir = current.position().dir(player.as_ref());

        let transform = Transform2D {
            scale: vec2(0.03, 0.03),
            rotation: dir.rot(),
            ..*current
        };

        let component = (
            Enemy,
            Bullet,
            transform,
            CircleHitbox2D::new(0.010),
            Sprite2D::new(self.bullet.clone()),
            MoveParams::move_linear(dir),
        );

        world.spawn(component);
    }
}
